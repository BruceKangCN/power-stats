use std::path::PathBuf;

use chrono::{NaiveDateTime, Timelike};

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
    ///其他时段
    Other,
}

pub fn get_file_content(filepath: &str) -> Result<String, String> {
    let matches = charset_normalizer_rs::from_path(&PathBuf::from(filepath), None)?;
    let best_match = matches.get_best().ok_or("failed to get best encoding")?;
    let content = best_match
        .decoded_payload()
        .ok_or("failed to decode text")?;

    Ok(content.into())
}

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

    return PeriodCategory::Other;
}
