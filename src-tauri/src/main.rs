// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

use chrono::{NaiveDateTime, TimeDelta};
use csv::Trim;
use model::{PowerFigureRecord, Record, RespondData, WorkFigureRecord};
use power_stats::{categorize_by_datetime, get_file_content, PeriodCategory};

pub mod model;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// 根据前端提供的信息构建用于绘图的数据集
#[tauri::command]
async fn build_datasets(
    rated_capacity: f64,
    is_primary_load: bool,
    factor: Option<f64>,
    filepath: String,
) -> Result<RespondData, String> {
    let content = get_file_content(&filepath)?;
    // 读取时需清除空白字符，不然会导致数值解析出错
    let mut reader = csv::ReaderBuilder::new()
        .trim(Trim::All)
        .from_reader(content.as_bytes());

    // 用于存放时刻-功率对应关系的哈希表
    let mut map = HashMap::new();
    for result in reader.deserialize() {
        let record: Record = result.or(Err("failed to deserialize record"))?;

        let dt = NaiveDateTime::parse_from_str(&record.time, "%Y-%m-%d %H:%M:%S")
            .expect("failed to parse datetime");

        map.insert(dt, record.active_power);
    }

    // 将 map 排序并放入 vector 中
    //
    // 注：数据中可能有缺失的记录，所以不能直接将 map.len() 作为向量的长度，
    // 而是应计算时间间隔进而计算向量长度。
    let start_point = map.keys().min().unwrap().to_owned();
    let time_delta = *map.keys().max().unwrap() - start_point;
    let mut power_vec = vec![0.0; (time_delta.num_minutes() as usize) / 15 + 1];
    let factor = if is_primary_load {
        factor.unwrap()
    } else {
        1.0
    };
    for (k, v) in map.iter() {
        // 采样间隔为 15 分钟，所以索引值为经过的时间（以分钟为单位）除以 15
        let idx = (*k - start_point).num_minutes() / 15;
        power_vec[idx as usize] = *v * factor;
    }

    // 填写功率数据
    let mut power_data = Vec::new();
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
            PeriodCategory::Other => PowerFigureRecord {
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

    let days = time_delta.num_days();
    let mut work_data = Vec::new();
    // 填写能耗数据
    for day in start_point.date().iter_days().take((days + 1) as usize) {
        // 当前步骤所计算的日期的 0 时相对于起始时间点的时长，以 15 分钟为单位
        let offset = (day.and_hms_opt(0, 0, 0).unwrap() - start_point).num_minutes() / 15;

        let max_idx = (map.len() - 1) as i64;

        let morning_peak = if offset + 8 * 4 > max_idx || offset + 11 * 4 <= 0 {
            0.0
        } else {
            let begin = std::cmp::max(offset + 8 * 4, 0) as usize;
            let end = std::cmp::min(offset + 11 * 4 - 1, max_idx) as usize;

            power_vec[begin..end].iter().sum()
        };

        let noon_peak = if offset + 13 * 4 > max_idx || offset + 17 * 4 <= 0 {
            0.0
        } else {
            let begin = std::cmp::max(offset + 13 * 4, 0) as usize;
            let end = std::cmp::min(offset + 17 * 4 - 1, max_idx) as usize;

            power_vec[begin..end].iter().sum()
        };

        work_data.push(WorkFigureRecord {
            date: day.format("%Y-%m-%d").to_string(),
            morning_peak,
            noon_peak,
        });
    }

    Ok(RespondData {
        power_data,
        work_data,
    })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, build_datasets])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
