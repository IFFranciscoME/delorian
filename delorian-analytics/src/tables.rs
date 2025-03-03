
#[derive(Debug, Clone)]
pub struct JitoMetricsTable {
    pub tip_found: bool,
    pub tip_amount: u64,
    pub tip_account: String,
}

#[derive(Debug, Clone)]
pub struct CostsMetricsTable {
    pub total_fee: u64,
    pub compute_budget_call: bool,
    pub compute_unit_limit: u32,
    pub compute_unit_price: u64,
    pub compute_unit_consumed: u32,
}

#[derive(Debug, Clone)]
pub struct PfeMetricsTable {
    pub min: f64,
    pub low: f64,
    pub medium: f64,
    pub high: f64,
    pub very_high: f64,
    pub unsafe_max: f64,
}

#[derive(Debug, Clone)]
pub struct PfrMetricsTable {
    pub first_slot: u64,
    pub last_slot: u64,
    pub count_slot: i64,
    pub min_fee: f64,
    pub mean_fee: f64,
    pub median_fee: f64,
    pub max_fee: f64,
}

#[derive(Debug, Clone)]
pub struct TxAnalyticsTable {
    pub compute_units: u64,
    pub fee: u64,
    pub compute_budget_call: bool,
    pub net_profit: f64,
}

