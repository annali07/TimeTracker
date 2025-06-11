// src-tauri/src/lib.rs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::Local;
use std::{
  fs::{create_dir_all, OpenOptions},
  io::Write,
};

#[tauri::command]
fn log_session_csv(activity: String, duration: u64, directory: String) -> Result<(), String> {
    let today = Local::now().format("%Y-%m-%d").to_string();
    let row = format!("{},{},{}\n", today, activity, duration);

    let dir = std::path::PathBuf::from(directory);
    create_dir_all(&dir).map_err(|e| e.to_string())?;

    let file_path = dir.join("sessions.csv");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .map_err(|e| e.to_string())?;

    file.write_all(row.as_bytes()).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn get_daily_summary_csv(directory: String) -> Result<std::collections::HashMap<String, u64>, String> {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let today = Local::now().format("%Y-%m-%d").to_string();
    let file_path = std::path::PathBuf::from(directory).join("sessions.csv");

    let file = File::open(&file_path).map_err(|e| format!("Failed to open CSV: {}", e))?;
    let reader = BufReader::new(file);

    let mut totals = HashMap::new();
    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 3 { continue; }
        if parts[0] == today {
            let activity = parts[1].to_string();
            let duration: u64 = parts[2].parse().unwrap_or(0);
            *totals.entry(activity).or_insert(0) += duration;
        }
    }

    Ok(totals)
}


#[tauri::command]
fn get_weekly_summary_csv(directory: String) -> Result<std::collections::HashMap<String, std::collections::HashMap<String, u64>>, String> {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use chrono::{NaiveDate, Datelike};

    let mut week_data: HashMap<String, HashMap<String, u64>> = HashMap::new();
    let weekdays = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];

    // Initialize empty structure
    for day in weekdays.iter() {
        let mut day_block = HashMap::new();
        day_block.insert("U".to_string(), 0);
        day_block.insert("E".to_string(), 0);
        day_block.insert("S".to_string(), 0);
        day_block.insert("F".to_string(), 0);
        week_data.insert(day.to_string(), day_block);
    }

    // Load file
    let file_path = std::path::PathBuf::from(directory).join("sessions.csv");
    let file = File::open(&file_path).map_err(|e| format!("Failed to open CSV: {}", e))?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;
        let parts: Vec<&str> = line.trim().split(',').collect();
        if parts.len() != 3 { continue; }

        let date_str = parts[0];
        let activity = parts[1];
        let duration: u64 = parts[2].parse().unwrap_or(0);

        // Parse date string into weekday
        let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
            .map_err(|e| format!("Date parse error: {}", e))?;
        let weekday = match date.weekday().number_from_monday() {
            1 => "Monday",
            2 => "Tuesday",
            3 => "Wednesday",
            4 => "Thursday",
            5 => "Friday",
            6 => "Saturday",
            7 => "Sunday",
            _ => continue, // skip invalid weekdays
        };

        // Aggregate duration
        if let Some(day_block) = week_data.get_mut(weekday) {
            *day_block.entry(activity.to_string()).or_insert(0) += duration;
        }
    }

    Ok(week_data)
}


#[tauri::command]
fn get_monthly_summary_csv(directory: String) -> Result<std::collections::HashMap<String, u64>, String> {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use chrono::{NaiveDate, Datelike, Local};

    let file_path = std::path::PathBuf::from(directory).join("sessions.csv");
    let file = File::open(&file_path).map_err(|e| format!("Failed to open CSV: {}", e))?;
    let reader = BufReader::new(file);

    let today = Local::now().naive_local();
    let current_month = today.month();
    let current_year = today.year();

    let mut totals: HashMap<String, u64> = HashMap::new();

    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;
        let parts: Vec<&str> = line.trim().split(',').collect();
        if parts.len() != 3 { continue; }

        let date_str = parts[0];
        let activity = parts[1];
        let duration: u64 = parts[2].parse().unwrap_or(0);

        let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
            .map_err(|e| format!("Date parse error: {}", e))?;

        if date.month() == current_month && date.year() == current_year {
            *totals.entry(activity.to_string()).or_insert(0) += duration;
        }
    }

    Ok(totals)
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())                    // ← register FS plugin
        .plugin(tauri_plugin_opener::init())                // ← register opener plugin
        .invoke_handler(tauri::generate_handler![
          log_session_csv,
          get_daily_summary_csv,
          get_weekly_summary_csv,
          get_monthly_summary_csv
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
