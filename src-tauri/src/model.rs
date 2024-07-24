use serde::{Deserialize, Serialize};

/// 数据表中的数据结构
#[derive(Debug, Deserialize)]
pub struct Record {
    /// 时间
    #[serde(alias = "日期")]
    pub time: String,
    /// 瞬时有功功率
    // 少量数据会缺失，需要默认处理
    // CSV 无法区分空字符串和空白字段，所以需要显式指定反序列化函数，以实现默认值功能
    #[serde(alias = "瞬时有功", deserialize_with = "default_if_empty")]
    pub active_power: f64,
    // /// 瞬时视在功率
    // pub apparent_power: f64,
}

/// 用于绘制功率图表的数据
///
/// 注：字段通过 rename 进行简写以优化传输、解析性能
///
/// 简写构成：时段+类型
///
/// 时段：
///     - 早：m (morning)
///     - 午：n (noon)
///     - 晚：e (evening)
/// 类型：
///     - 峰时功率：p (peak)
///     - 谷时功率：o (off peak)
///     - 谷时余量：r (remain)
#[derive(Debug, Serialize)]
pub struct PowerFigureRecord {
    /// 时间
    #[serde(rename = "t")]
    pub time: String,
    /// 晚谷时功率
    #[serde(rename = "eo")]
    pub evening_off_peak: Option<f64>,
    /// 早峰时功率
    #[serde(rename = "mp")]
    pub morning_peak: Option<f64>,
    /// 午谷时功率
    #[serde(rename = "no")]
    pub noon_off_peak: Option<f64>,
    /// 午峰时功率
    #[serde(rename = "np")]
    pub noon_peak: Option<f64>,
    /// 晚谷时余量
    #[serde(rename = "er")]
    pub evening_remain: Option<f64>,
    /// 午谷时余量
    #[serde(rename = "nr")]
    pub noon_remain: Option<f64>,
}

/// 用于绘制能耗图表的数据
///
/// 注：字段通过 rename 进行简写以优化传输、解析性能
///
/// 简写构成：时段
///
/// 时段：
///     - 早：m (morning)
///     - 午：n (noon)
#[derive(Debug, Serialize)]
pub struct WorkFigureRecord {
    /// 时间
    #[serde(rename = "d")]
    pub date: String,
    /// 早峰时能耗
    #[serde(rename = "m")]
    pub morning_peak: f64,
    /// 午峰时能耗
    #[serde(rename = "n")]
    pub noon_peak: f64,
}

/// 发回前端的数据
#[derive(Debug, Serialize)]
pub struct RespondData {
    #[serde(rename = "p")]
    pub power_data: Vec<PowerFigureRecord>,
    #[serde(rename = "w")]
    pub work_data: Vec<WorkFigureRecord>,
}

// 抄的，我也看不懂
fn default_if_empty<'de, D, T>(de: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de> + Default,
{
    Option::<T>::deserialize(de).map(|x| x.unwrap_or_default())
}
