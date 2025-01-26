import http from 'k6/http';
import { check, sleep } from 'k6';
import { SharedArray } from 'k6/data';
import { Rate } from 'k6/metrics';

// Track the error rate
export let errorRate = new Rate('errors');

// Base URL for the compute service
// e.g. "http://localhost:8080" or "http://compute:8080"
const BASE_URL = __ENV.BASE_URL || 'http://localhost:8080';
const ENDPOINT = `${BASE_URL}/api/compute`;

// We define two "extreme" load scenarios
export let options = {
  thresholds: {
    // We'll fail if 95% of requests take more than 2 seconds
    http_req_duration: ['p(95)<2000'],
    // We'll fail if more than 5% of requests fail
    errors: ['rate<0.05'],
  },

  scenarios: {
    // 1) ramping_vus scenario: quickly scale VUs up to 500
    big_ramp: {
      executor: 'ramping-vus',
      startVUs: 0,
      stages: [
        { duration: '30s', target: 100 },  // Ramp to 100 VUs over 30s
        { duration: '1m', target: 100 },   // Stay at 100 VUs for 1m
        { duration: '30s', target: 500 },  // Ramp to 500 VUs over 30s
        { duration: '2m', target: 500 },   // Stay at 500 VUs for 2m
        { duration: '30s', target: 0 },    // Ramp down to 0
      ],
      exec: 'postCompute',   // the function below
      gracefulStop: '30s',
    },

    // 2) constant-arrival-rate scenario: attempt 1000 RPS
    // after the ramp scenario finishes
    massive_rate: {
      executor: 'constant-arrival-rate',
      rate: 1000,         // 1000 iterations per second
      timeUnit: '1s',     // i.e. 1000 RPS
      duration: '1m',     // run for 1 minute
      preAllocatedVUs: 500,  // start with 500 VUs
      maxVUs: 1000,       // up to 1000 VUs
      startTime: '5m',    // wait until big_ramp finishes ( ~5m ) 
      exec: 'postCompute',
      gracefulStop: '30s',
    },
  },
};

// Our test function
export function postCompute() {
  let record = generateRandomPropertyRecord();

  let res = http.post(ENDPOINT, JSON.stringify(record), {
    headers: { 'Content-Type': 'application/json' },
  });

  let passed = check(res, {
    'status is 200': (r) => r.status === 200,
  });

  // If check fails, mark an error
  if (!passed) {
    errorRate.add(1);
  }

  // short sleep to avoid pegging CPU in a tight loop
  sleep(0.1);
}

// Create a random property-like object
function generateRandomPropertyRecord() {
  // random 6-char suffix
  const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789';
  let suffix = '';
  for (let i = 0; i < 6; i++) {
    suffix += chars[Math.floor(Math.random() * chars.length)];
  }

  // random assessed_value, e.g. 50k to 2M
  let assessed = 50000 + Math.random() * 1_950_000;

  // 10% chance overdue
  let overdue = Math.random() < 0.1;

  return {
    property_id: 'EXTREME-' + suffix,
    owner_name: 'Load Tester ' + suffix,
    address: {
      street: '123 Main St',
      city: 'Riverside',
      state: 'CA',
      zip: '92501',
    },
    assessed_value: assessed,
    location_code: 'RIV-CA',
    is_overdue: overdue,
    // use some i64-like value e.g. now or older
    last_payment_unix: overdue ? 1670000000 : 1680000000,
  };
}
