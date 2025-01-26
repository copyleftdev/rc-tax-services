use serde::{Deserialize, Serialize};

/// A sub-struct for mailing or property address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip: String,
}

/// A more realistic property record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyRecord {
    pub property_id: String,
    pub owner_name: String,
    pub address: Address,
    pub assessed_value: f64,
    pub location_code: String,
    pub is_overdue: bool,
    // Use i64 so we can store it in a BIGINT column via SQLx
    pub last_payment_unix: Option<i64>,
}

/// Example tax computation
pub fn compute_tax(record: &PropertyRecord) -> f64 {
    let base_rate = 0.012; // e.g. 1.2% base
    let local_surcharge = match record.location_code.as_str() {
        "RIV-CA" => 0.003,
        "LA-CA" => 0.004,
        "SD-CA" => 0.0025,
        _ => 0.0,
    };
    let penalty_rate = if record.is_overdue { 0.005 } else { 0.0 };

    let total_rate = base_rate + local_surcharge + penalty_rate;
    record.assessed_value * total_rate
}
