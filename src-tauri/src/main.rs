// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

use model::{FigureRecord, Record};
use power_stats::{categorize_by_time, get_date_time, get_file_content, DateTime, PeriodCategory};
use regex::Regex;

pub mod model;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn handle_submit(
    rated_capacity: f64,
    is_primary_load: bool,
    ratio: Option<f64>,
    filepath: String,
) -> Result<Vec<FigureRecord>, String> {
    let content = get_file_content(&filepath)?;
    let mut reader = csv::Reader::from_reader(content.as_bytes());

    // 用于匹配时间、日期的正则表达式，格式为 YYYY-MM-DD hh:mm:ss
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2}):(\d{2})").unwrap();
    // 用于存放时刻-功率对应关系的哈希表
    let mut map = HashMap::new();

    for result in reader.deserialize() {
        println!("{:?}", result);
        let record: Record = result.or(Err("failed to deserialize record"))?;

        let dt = get_date_time(&record.time, &re);
        let key = DateTime {
            year: dt.year,
            month: dt.month,
            day: dt.day,
            hour: dt.hour,
            minute: 0,
            second: 0,
        };

        // 简便起见，采用矩形积分
        map.entry(key).and_modify(|value| *value += record.active_power).or_insert(record.active_power);
    }

    let ratio = if is_primary_load {
        ratio.unwrap()
    } else {
        1.0
    };

    // 将哈希表转换为向量
    let mut data = Vec::new();
    for (k, v) in map.iter() {
        let time = format!("{}-{}-{} {}:00:00", k.year, k.month, k.day, k.hour);

        let category = categorize_by_time(&k);
        let r = match category {
            PeriodCategory::Peak => FigureRecord {
                time,
                peak: Some(v / 4.0 * ratio),
                off_peak: None,
                sharp: None,
            },
            PeriodCategory::OffPeak => FigureRecord {
                time,
                peak: None,
                off_peak: Some(v / 4.0 * ratio),
                sharp: None,
            },
            PeriodCategory::Sharp => FigureRecord {
                time,
                peak: None,
                off_peak: None,
                sharp: Some(v / 4.0 * ratio),
            },
            PeriodCategory::Other => continue,
        };

        data.push(r);
    }

    Ok(data)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, handle_submit])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
