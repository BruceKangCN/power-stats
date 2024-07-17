use serde::{Deserialize, Serialize};

/// 数据表中的数据结构
#[derive(Debug, Deserialize)]
pub struct Record {
    /// 时间
    #[serde(alias = "日期")]
    pub time: String,
    /// 瞬时有功功率
    #[serde(alias = "瞬时有功")]
    pub active_power: f64,
    // /// 瞬时视在功率
    // pub apparent_power: f64,
}

/// 用于绘图的数据
#[derive(Debug, Serialize)]
pub struct FigureRecord {
    /// 时间
    pub time: String,
    /// 峰时功率
    pub peak: Option<f64>,
    /// 谷时功率
    pub off_peak: Option<f64>,
    /// 尖时功率
    pub sharp: Option<f64>,
}
