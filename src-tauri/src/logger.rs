use polars::prelude::*;
use regex::Regex;
use serde::Serialize;
use std::{
    fs::{self, File, OpenOptions},
    io::BufWriter,
    io::Write,
    path::PathBuf,
};

use crate::{enums::LogLevel, helper, PACKAGEINFO};

pub fn format_text(text: &str, color: &str, bold: bool) -> String {
    let color_code = match color {
        "red" => "31",
        "green" => "32",
        "yellow" => "33",
        "blue" => "34",
        "magenta" => "35",
        "cyan" => "36",
        "white" => "37",
        "orange" => "38;5;208",
        _ => "0", // default color
    };

    if bold {
        format!("\x1b[1;{}m{}\x1b[0m", color_code, text)
    } else {
        format!("\x1b[{}m{}\x1b[0m", color_code, text)
    }
}
fn remove_ansi_codes(s: &str) -> String {
    let re = Regex::new(r"\x1B\[([0-9]{1,2}(;[0-9]{1,2})?)?[m|K]").unwrap();
    re.replace_all(s, "").to_string()
}
fn format_square_bracket(msg: &str) -> String {
    format!(
        "{}{}{}",
        format_text("[", "cyan", false),
        msg,
        format_text("]", "cyan", false)
    )
}

pub fn dolog(level: LogLevel, component: &str, msg: &str, console: bool, file: Option<&str>) {
    let time = format_square_bracket(
        chrono::Local::now()
            .naive_utc()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
            .as_str(),
    );
    let component = format_square_bracket(format_text(component, "magenta", true).as_str());
    let msg = format_text(msg, "white", false);
    let log_prefix = match level {
        LogLevel::Info => format_square_bracket(format_text("INFO", "green", true).as_str()),
        LogLevel::Warning => format_square_bracket(format_text("WARN", "orange", true).as_str()),
        LogLevel::Error => format_square_bracket(format_text("ERROR", "red", true).as_str()),
        LogLevel::Debug => format_square_bracket(format_text("DEBUG", "blue", true).as_str()),
        LogLevel::Trace => format_square_bracket(format_text("TRACE", "cyan", true).as_str()),
        LogLevel::Critical => format_square_bracket(format_text("CRITICAL", "red", true).as_str()),
        _ => format_square_bracket(format_text("UNKNOWN", "white", true).as_str()),
    };
    if console {
        println!("{} {} {} {}", time, log_prefix, component, msg);
    }

    if let Some(file) = file {
        let mut log_path = get_log_forlder();
        log_path.push(file);
        if !log_path.exists() {
            fs::File::create(&log_path).unwrap();
        }
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(log_path)
            .unwrap();

        if let Err(e) = writeln!(
            file,
            "{} {} {} {}",
            remove_ansi_codes(time.as_str()),
            remove_ansi_codes(log_prefix.as_str()),
            remove_ansi_codes(component.as_str()),
            remove_ansi_codes(msg.as_str())
        ) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}

pub fn get_log_forlder() -> PathBuf {
    let app_path = helper::get_app_roaming_path();
    let log_path = app_path.join("logs");
    // Create the directory if it does not exist
    if !log_path.exists() {
        fs::create_dir_all(&log_path).unwrap();
    }
    //create a folder for the current date
    let date = chrono::Local::now()
        .naive_utc()
        .format("%Y-%m-%d")
        .to_string();
    let log_path = log_path.join(date);
    // Create the directory if it does not exist
    if !log_path.exists() {
        fs::create_dir_all(&log_path).unwrap();
    }
    log_path
}

pub fn debug(component: &str, msg: &str, console: bool, file: Option<&str>) {
    dolog(LogLevel::Debug, component, msg, console, file);
}
pub fn debug_file(component: &str, msg: &str, file: Option<&str>) {
    debug(component, msg, false, file);
}
pub fn debug_con(component: &str, msg: &str) {
    debug(component, msg, true, None);
}

pub fn error(component: &str, msg: &str, console: bool, file: Option<&str>) {
    dolog(LogLevel::Error, component, msg, console, file);
}
pub fn error_file(component: &str, msg: &str, file: Option<&str>) {
    error(component, msg, false, file);
}
pub fn error_con(component: &str, msg: &str) {
    error(component, msg, true, None);
}

pub fn info(component: &str, msg: &str, console: bool, file: Option<&str>) {
    dolog(LogLevel::Info, component, msg, console, file);
}
pub fn info_file(component: &str, msg: &str, file: Option<&str>) {
    info(component, msg, false, file);
}
pub fn info_con(component: &str, msg: &str) {
    info(component, msg, true, None);
}

pub fn trace(component: &str, msg: &str, console: bool, file: Option<&str>) {
    dolog(LogLevel::Trace, component, msg, console, file);
}
pub fn trace_file(component: &str, msg: &str, file: Option<&str>) {
    trace(component, msg, false, file);
}
pub fn trace_con(component: &str, msg: &str) {
    trace(component, msg, true, None);
}

pub fn critical(component: &str, msg: &str, console: bool, file: Option<&str>) {
    dolog(LogLevel::Critical, component, msg, console, file);
}
pub fn critical_file(component: &str, msg: &str, file: Option<&str>) {
    critical(component, msg, false, file);
}
pub fn critical_con(component: &str, msg: &str) {
    critical(component, msg, true, None);
}

pub fn warning(component: &str, msg: &str, console: bool, file: Option<&str>) {
    dolog(LogLevel::Warning, component, msg, console, file);
}
pub fn warning_file(component: &str, msg: &str, file: Option<&str>) {
    warning(component, msg, false, file);
}
pub fn warning_con(component: &str, msg: &str) {
    warning(component, msg, true, None);
}
/// Logs the given DataFrame to a CSV file with the given name in the log folder.
/// The `df` argument is a mutable reference to the DataFrame to be logged.
/// The `name` argument is a string representing the name of the CSV file to be created.
/// The CSV file is created in the log folder, which is determined by the `get_log_folder` function.
/// If the file creation or write fails, an error message is printed to the console.
/// If the write is successful, an info message is printed to the console.
pub fn log_dataframe(df: &mut DataFrame, name: &str) {
    let mut log_path = get_log_forlder();
    log_path.push(name);
    // Cerate a csv file with the sorted DataFrame of price data
    let output_file: File = File::create(log_path).expect("create failed");
    let writer = BufWriter::new(output_file);
    // Write the DataFrame to a CSV file
    CsvWriter::new(writer).finish(df).expect("write failed");
    info(
        "Logger",
        format!("DataFrame logged to {}", format_text(name, "yellow", false)).as_str(),
        false,
        None,
    );
}

pub fn export_logs() {
    let date = chrono::Local::now()
        .naive_utc()
        .format("%Y-%m-%d")
        .to_string();

    let packageinfo = PACKAGEINFO
        .lock()
        .unwrap()
        .clone()
        .expect("Could not get package info");
    let version = packageinfo.version.to_string();

    let zip_path = helper::get_desktop_path().join(format!("{} v{} {} Logs.zip",packageinfo.name, version, date));
    let mut files_to_compress: Vec<helper::ZipEntry> = vec![];

    let mut logs_path = get_log_forlder();
    logs_path.pop();

    files_to_compress.push(helper::ZipEntry {
        file_path: logs_path,
        sub_path: Some("logs".to_string()),
        include_dir: true,
    });

    let app_path = helper::get_app_roaming_path();
    for path in fs::read_dir(app_path).unwrap() {
        let path = path.unwrap().path();
        // Check if path is auth.json
            if path.ends_with("auth.json") {
                info_con("Logger", "Skipping auth.json");
            } else {
                files_to_compress.push(helper::ZipEntry {
                    file_path: path.to_owned(),
                    sub_path: None,
                    include_dir: false,
                });                
            }
    }

    match helper::create_zip_file(files_to_compress, zip_path.to_str().unwrap_or_default()) {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
