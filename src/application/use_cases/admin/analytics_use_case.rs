use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyticsData {
    pub total_orders: u64,
    pub total_revenue: f64,
    pub total_users: u64,
    pub total_products: u64,
}

pub struct AnalyticsUseCase;

impl AnalyticsUseCase {
    pub fn new() -> Self {
        Self
    }

    pub async fn get_dashboard_data(&self) -> Result<AnalyticsData, String> {
        // 実装は後で追加
        todo!("Get dashboard data implementation")
    }
}
