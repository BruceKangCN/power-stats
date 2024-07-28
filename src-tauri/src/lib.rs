use std::{collections::HashMap, path::PathBuf};

use chrono::{NaiveDateTime, TimeDelta, Timelike};
use csv::Trim;
use model::{PowerFigureRecord, Record, WorkFigureRecord};

pub mod model;

/// 时段分类
#[derive(PartialEq, Eq)]
pub enum PeriodCategory {
    ///晚谷时
    EveningOffPeak,
    /// 早峰时
    MorningPeak,
    /// 午谷时
    NoonOffPeak,
    /// 午峰时
    NoonPeak,
    /// 平时
    Normal,
}

/// 根据文件路径读取数据。
///
/// 自动判断文件的编码并转换为 UTF-8 编码格式字符串返回。
pub fn get_file_content(filepath: &str) -> Result<String, String> {
    let matches = charset_normalizer_rs::from_path(&PathBuf::from(filepath), None)?;
    let best_match = matches.get_best().ok_or("failed to get best encoding")?;
    let content = best_match
        .decoded_payload()
        .ok_or("failed to decode text")?;

    Ok(content.into())
}

/// 将数据映射成 HashMap 以便于下一步处理。
pub fn map_records(content: &str) -> Result<HashMap<NaiveDateTime, f64>, String> {
    // 读取时需清除空白字符，不然会导致数值解析出错
    let mut reader = csv::ReaderBuilder::new()
        .trim(Trim::All)
        .from_reader(content.as_bytes());

    let mut map = HashMap::new();
    for result in reader.deserialize() {
        let record: Record = result.or(Err("failed to deserialize record"))?;

        let dt = NaiveDateTime::parse_from_str(&record.time, "%Y-%m-%d %H:%M:%S")
            .expect("failed to parse datetime");

        // 注意：如果有重复数据，新数据会覆盖旧数据
        map.insert(dt, record.active_power);
    }

    Ok(map)
}

/// 生成功率数据
pub fn get_power_data(
    power_vec: &Vec<f64>,
    start_point: &NaiveDateTime,
    rated_capacity: f64,
) -> Vec<PowerFigureRecord> {
    let mut power_data = Vec::new();

    let start_point = *start_point;
    for (i, p) in power_vec.iter().enumerate() {
        let dt = start_point + TimeDelta::minutes((i as i64) * 15);
        let cat = categorize_by_datetime(&dt);

        power_data.push(match cat {
            PeriodCategory::EveningOffPeak => PowerFigureRecord {
                time: dt.format("%Y-%m-%d %H:%M:00").to_string(),
                evening_off_peak: Some(*p),
                morning_peak: None,
                noon_off_peak: None,
                noon_peak: None,
                evening_remain: Some(rated_capacity - *p),
                noon_remain: None,
            },
            PeriodCategory::MorningPeak => PowerFigureRecord {
                time: dt.format("%Y-%m-%d %H:%M:00").to_string(),
                evening_off_peak: None,
                morning_peak: Some(*p),
                noon_off_peak: None,
                noon_peak: None,
                evening_remain: None,
                noon_remain: None,
            },
            PeriodCategory::NoonOffPeak => PowerFigureRecord {
                time: dt.format("%Y-%m-%d %H:%M:00").to_string(),
                evening_off_peak: None,
                morning_peak: None,
                noon_off_peak: Some(*p),
                noon_peak: None,
                evening_remain: None,
                noon_remain: Some(rated_capacity - *p),
            },
            PeriodCategory::NoonPeak => PowerFigureRecord {
                time: dt.format("%Y-%m-%d %H:%M:00").to_string(),
                evening_off_peak: None,
                morning_peak: None,
                noon_off_peak: None,
                noon_peak: Some(*p),
                evening_remain: None,
                noon_remain: None,
            },
            PeriodCategory::Normal => PowerFigureRecord {
                time: dt.format("%Y-%m-%d %H:%M:00").to_string(),
                evening_off_peak: None,
                morning_peak: None,
                noon_off_peak: None,
                noon_peak: None,
                evening_remain: None,
                noon_remain: None,
            },
        });
    }

    power_data
}

/// 生成能耗数据
pub fn get_work_data(
    power_vec: &Vec<f64>,
    start_point: &NaiveDateTime,
    time_delta: &TimeDelta,
) -> Vec<WorkFigureRecord> {
    let start_point = *start_point;
    let time_delta = *time_delta;

    let days = time_delta.num_days();
    let mut work_data = Vec::new();
    // 填写能耗数据
    for day in start_point.date().iter_days().take((days + 1) as usize) {
        // 当前步骤所计算的日期的 0 时相对于起始时间点的时长，以 15 分钟为单位
        let offset = (day.and_hms_opt(0, 0, 0).unwrap() - start_point).num_minutes() / 15;

        let max_idx = (power_vec.len() - 1) as i64;

        let morning_peak = if offset + 8 * 4 > max_idx || offset + 11 * 4 <= 0 {
            0.0
        } else {
            let begin = std::cmp::max(offset + 8 * 4, 0) as usize;
            let end = std::cmp::min(offset + 11 * 4 - 1, max_idx) as usize;

            power_vec[begin..end].iter().sum::<f64>() * 0.25
        };

        let noon_peak = if offset + 13 * 4 > max_idx || offset + 17 * 4 <= 0 {
            0.0
        } else {
            let begin = std::cmp::max(offset + 13 * 4, 0) as usize;
            let end = std::cmp::min(offset + 17 * 4 - 1, max_idx) as usize;

            power_vec[begin..end].iter().sum::<f64>() * 0.25
        };

        work_data.push(WorkFigureRecord {
            date: day.format("%Y-%m-%d").to_string(),
            morning_peak,
            noon_peak,
        });
    }

    work_data
}

/// 根据时间返回时段分类。
///
/// 注：以下均为左闭右开区间
///
/// - 晚谷时：0 - 8
/// - 早峰时：8 - 11
/// - 午谷时：11 - 13
/// - 午峰时：13 - 17
/// - 平时：其他时段
pub fn categorize_by_datetime(dt: &NaiveDateTime) -> PeriodCategory {
    // 注意：0 <= dt.hour() 必定为 true，写到条件表达式中会报警告
    if dt.hour() < 8 {
        return PeriodCategory::EveningOffPeak;
    }

    if 8 <= dt.hour() && dt.hour() < 11 {
        return PeriodCategory::MorningPeak;
    }

    if 11 <= dt.hour() && dt.hour() < 13 {
        return PeriodCategory::NoonOffPeak;
    }

    if 13 <= dt.hour() && dt.hour() < 17 {
        return PeriodCategory::NoonPeak;
    }

    return PeriodCategory::Normal;
}
