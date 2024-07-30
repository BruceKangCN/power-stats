// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use power_stats::model::Response;
use power_stats::{build_power_records, build_work_records, get_file_content, map_records};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// 根据前端提供的信息构建用于绘图的数据
#[tauri::command]
async fn build_series(
    rated_capacity: f64,
    is_primary_load: bool,
    factor: Option<f64>,
    filepath: String,
) -> Result<Response, String> {
    let content = get_file_content(&filepath)?;
    let map = map_records(&content)?;

    // 将 map 排序并放入 vector 中
    //
    // 注：数据中可能有缺失的记录，所以不能直接将 map.len() 作为向量的长度，
    // 而是应计算时间间隔进而计算向量长度。缺失的记录会被填充为 0。
    //
    // PS: 该步骤同时还计算出了许多后续步骤需要用到的变量，不适合提取为函数。
    let start_point = map.keys().min().unwrap().to_owned();
    let time_delta = *map.keys().max().unwrap() - start_point;
    let factor = if is_primary_load {
        factor.unwrap()
    } else {
        1.0
    };
    let mut power_vec = vec![0.0; (time_delta.num_minutes() as usize) / 15 + 1];
    for (k, v) in map.iter() {
        // 采样间隔为 15 分钟，所以索引值为经过的时间（以分钟为单位）除以 15
        let idx = (*k - start_point).num_minutes() / 15;
        power_vec[idx as usize] = *v * factor;
    }

    let power_records = build_power_records(&power_vec, &start_point, rated_capacity);
    let work_records = build_work_records(&power_vec, &start_point);

    Ok(Response {
        power_records,
        work_records,
    })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, build_series,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
