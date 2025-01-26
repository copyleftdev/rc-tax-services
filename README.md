# RC Tax Services

**A minimal, high-performance property tax service and load test—built in just a couple nights as a middle finger to bloated “enterprise” solutions that have overcharged government agencies for decades.**

## Overview

Many government offices get saddled with overpriced, needlessly complex systems. Our goal: **prove** you can build a **lean, maintainable** microservice for property taxes (or similar use cases) in a matter of days—not months—while achieving **millisecond** response times under massive load.

This repository showcases:
- A **Compute** service (handling property records + DB inserts)
- An **Ingest** service (optional WebSocket-based pipeline)
- A **Loadtest** service (Rust-based)  
- An **alternative K6** script for more advanced, scenario-based testing

All coded swiftly to highlight that huge vendor solutions often **overpromise** and **underdeliver**, while simpler approaches can get the job done faster, cheaper, and more efficiently.

---

## Bloat vs. Lean (Mermaid Diagram)

Below is a **Mermaid** diagram contrasting the **bloated** vs. **lean** approach:

```mermaid
flowchart LR

    subgraph "Typical Over-Bloated Stack"
      direction TB
      A[Legacy Vendor<br>Portal UI] --> B[Massive<br>Service Bus]
      B --> C[Overly Complex<br>Workflow Engine]
      C --> D[Large Java EE<br>Monolith or Dozens of JARs]
      D --> E[Heavy ESB /<br>Integration Layer]
      E --> F[Complicated DB<br>(Expensive Licenses)]
      F --> G[Minimal Real Value<br>But Big Price Tag]
    end

    subgraph "RC Tax Services"
      direction TB
      X[Simple Web UI<br>(Optional)] --> Y[Lean Rust Service(s)]
      Y --> Z[Postgres DB<br>(Free & Straightforward)]
      Z --> L[Real Value in Days,<br>Millisecond Latencies]
    end

    A --- X
```

**What It Shows**:
- **Over-Bloated**: A typical high-level architecture from big vendors, with a labyrinth of bus layers, huge licensable components, and multiple frameworks. This often results in slow performance, huge complexity, and a painful integration story—despite the high cost.  
- **RC Tax Services**: A minimal approach—a Rust microservice or two, straightforward Postgres DB, optional Web UI or ingestion service. Built in **days**, with sub-10ms latencies under thousands of RPS.

---

## Why This Approach

1. **Frustration with Bloat**  
   Big corps keep delivering monstrous solutions to gov agencies at sky-high prices—often with slow UIs, countless layers, and minimal real optimization.  
2. **Show Don’t Tell**  
   In a couple nights, we coded these services from scratch, achieving sub-10ms latencies and thousands of requests per second, on minimal hardware.  
3. **Simplicity**  
   Rust microservices + straightforward DB schema—no endless vendor “integration.”  
4. **Scalability**  
   Our K6 scripts ramp up to hundreds of Virtual Users and attempt up to 1,000+ RPS. The system soaks it easily with near-zero errors.

---

## Usage (Local)

1. **Clone** the repo:
   ```bash
   git clone https://github.com/copyleftdev/rc-tax-services.git
   cd rc-tax-services
   ```
2. **Run** via Docker Compose:
   ```bash
   docker compose build
   docker compose up
   ```
   - `db` container: Postgres on port 5432  
   - `compute` container: Microservice on port 8080  
   - `ingest` container: WebSocket-based ingress on port 3000  
   - `loadtest` container (optional): Fires a quick, integrated Rust load test.
3. **Check** logs. You’ll see `compute` and `ingest` spin up, plus load test results if enabled.

### K6 Testing

- Install [k6](https://k6.io/docs/getting-started/installation/)  
- Then run:
  ```bash
  cd loadtest/
  BASE_URL=http://localhost:8080 k6 run k6_test.js
  ```
  or if you want to push extreme concurrency:
  ```bash
  BASE_URL=http://localhost:8080 k6 run extremeLoadTest.js
  ```
- Observe latencies, RPS, and error rates in the console output.

---

## Recent Extreme Load Test Results

**Example**: We ramped to 500 Virtual Users, then hammered 1,000 RPS:

```
=== LOADTEST RESULTS ===
Total Requests: 874,420  
RPS: ~2,428  
Avg Latency: ~2.71ms (p95 ~6.96ms)  
Errors: 0%  
```
No errors, sub-7ms 95th percentile—even at thousands of requests per second—showing it’s entirely possible to avoid “enterprise meltdown.”

---

## Key Takeaways

1. **Lean > Bloated**: A simpler microservice can handle high concurrency with minimal overhead.  
2. **Fast Delivery**: Built in two nights—contrasting months-long vendor solutions.  
3. **Cost**: Lower dev hours, minimal hardware, zero licensing nightmares.  
4. **Performance**: Sub-10ms latencies for 95% of requests, scaling to thousands RPS.

---

## Disclaimer

- **Prototype**: This code is a reference example—not fully production-grade (no advanced auth, multi-region DB, etc.).  
- **Scaling**: For truly massive traffic, you might add caching, replication, or more robust logic.  
- **Real Data**: Adjust field definitions for your actual county or governmental needs.

---

## Conclusion

**rc-tax-services** demonstrates a **middle finger** to those who claim only monstrous, overpriced software can serve government agencies effectively. **Yes**, you can do better: simpler architecture, Rust or similarly efficient languages, direct Postgres usage, and thorough load testing can deliver blazing-fast solutions on modest hardware—**in just days.**

Stop overpaying for bloated solutions. Build something lean, test it thoroughly, and keep your government agency’s software from turning into an endless money pit.

