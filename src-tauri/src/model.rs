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

/// 用于绘图的数据
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
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

fn default_if_empty<'de, D, T>(de: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de> + Default,
{
    Option::<T>::deserialize(de).map(|x| x.unwrap_or_else(|| T::default()))
}
