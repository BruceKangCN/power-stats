// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

use chrono::{NaiveDateTime, Timelike};
use csv::Trim;
use model::{FigureRecord, Record};
use power_stats::{categorize_by_datetime, get_file_content, PeriodCategory};

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
    // 读取时需清除空白字符，不然会导致数值解析出错
    let mut reader = csv::ReaderBuilder::new()
        .trim(Trim::All)
        .from_reader(content.as_bytes());

    // 用于存放时刻-功率对应关系的哈希表
    let mut map = HashMap::new();

    for result in reader.deserialize() {
        let record: Record = result.or(Err("failed to deserialize record"))?;

        // let dt = get_date_time(&record.time, &re);
        let dt = NaiveDateTime::parse_from_str(&record.time, "%Y-%m-%d %H:%M:%S")
            .expect("failed to parse datetime");
        let key = dt.date().and_hms_opt(dt.hour(), 0, 0).unwrap();

        // 简便起见，采用矩形积分
        map.entry(key)
            .and_modify(|value| *value += record.active_power)
            .or_insert(record.active_power);
    }

    // TODO: 将 map 排序并放入 vector 中，以距离起始经过的时间为单位（以小时计）

    let ratio = if is_primary_load { ratio.unwrap() } else { 1.0 };

    // 将哈希表转换为向量
    let mut data = Vec::new();
    for (k, v) in map.iter() {
        let time = k.format("%Y-%m-%d %H:00:00").to_string();

        let category = categorize_by_datetime(&k);
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
            PeriodCategory::Other => FigureRecord {
                time,
                peak: None,
                off_peak: None,
                sharp: None,
            },
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
