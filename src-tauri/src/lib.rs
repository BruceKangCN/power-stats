use std::path::PathBuf;

use regex::Regex;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DateTime {
    pub year: i64,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
    pub second: i32,
}

/// 时段分类
#[derive(PartialEq, Eq)]
pub enum PeriodCategory {
    /// 峰时
    Peak,
    /// 谷时
    OffPeak,
    /// 尖时
    Sharp,
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

pub fn get_date_time(s: &str, re: &Regex) -> DateTime {
    let caps = re.captures(s).unwrap();

    DateTime {
        year: caps[1].parse().unwrap(),
        month: caps[2].parse().unwrap(),
        day: caps[3].parse().unwrap(),
        hour: caps[4].parse().unwrap(),
        minute: caps[5].parse().unwrap(),
        second: caps[6].parse().unwrap(),
    }
}

pub fn categorize_by_time(dt: &DateTime) -> PeriodCategory {
    if 0 <= dt.hour && dt.hour < 8 || 11 <= dt.hour && dt.hour < 13 {
        return PeriodCategory::OffPeak;
    }

    if [1, 7, 8, 11].contains(&dt.month) {
        if 9 <= dt.hour && dt.hour < 11 || 15 <= dt.hour && dt.hour < 17 {
            return PeriodCategory::Sharp;
        }
    } else {
        if 8 <= dt.hour && dt.hour < 11 || 13 <= dt.hour && dt.hour < 17 {
            return PeriodCategory::Peak;
        }
    }

    return PeriodCategory::Other;
}
