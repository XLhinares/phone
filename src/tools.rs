use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::fs::{self};
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, Deserialize, Serialize)]
struct Device {
    #[serde(rename = "IP")]
    ip: String,
    #[serde(rename = "NAME")]
    name: String,
}

// DEVICE INFORMATION ========================================================

pub fn get_known_devices() -> HashMap<String, String> {
    let file_str = get_devices_file();
    let file_path = Path::new(&file_str);

    // Create directory if it doesn't exist
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create directory");
    }
    // Create file with headers if it doesn't exist
    if !file_path.exists() {
        let mut wtr = csv::Writer::from_path(file_path).expect("Failed to create file");
        wtr.write_record(&["IP", "NAME"])
            .expect("Failed to write header");
        return HashMap::new();
    }

    // Open file
    let file = File::open(file_path).expect("Failed to open file");
    let mut rdr = csv::Reader::from_reader(file);

    // Deserialize rows into a HashMap
    rdr.deserialize::<Device>()
        .filter_map(|result| result.ok()) // Ignore rows that fail to parse
        .map(|row| (row.ip, row.name)) // Transform struct into a tuple (K, V)
        .collect()
}

fn get_local_ip_from_string(text: &str) -> Vec<String> {
    // Matches any valid IPv4 format
    let pattern = Regex::new(r"192.168.\d{1,3}.\d{1,3}").unwrap();

    pattern
        .find_iter(text)
        .map(|mat| mat.as_str().to_string()) // Convert &str to String
        .collect() // Gather into Vec<String>
}

fn get_device_map_from_string(text: &str) -> HashMap<String, String> {
    let ip_list: Vec<String> = get_local_ip_from_string(text);
    let mut known_devices: HashMap<String, String> = get_known_devices();

    // Save the unknown IPs to the device file
    for ip in &ip_list {
        if known_devices.contains_key(ip) {
            continue;
        }
        save_device_info(&ip, "Unknown");
        known_devices.insert(ip.to_string(), "Unknown".to_string());
    }

    // Just retain the IPs that were actually in the input text
    known_devices.retain(|ip, _| ip_list.contains(ip));
    known_devices
}

pub fn select_device(devices: &HashMap<String, String>) -> Option<String> {
    // Get a list of the refs to the keys.
    let keys: Vec<&String> = devices.keys().collect();

    if keys.is_empty() {
        print_no_device();
        return None;
    }

    if keys.len() == 1 {
        return Some(keys[0].to_string());
    }

    for (i, ip) in keys.iter().enumerate() {
        // devices[*ip] gets the name associated with the IP
        // Basically, it's the reverse lookup from &
        println!("{}.\t{}\t({})", i + 1, ip, devices[*ip]);
    }
    print!("Enter selected index: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;

    let idx: usize = input.trim().parse().ok()?;

    if idx > 0 && idx <= devices.len() {
        return Some(keys[&idx - 1].to_string());
    } else {
        println!("Invalid index.");
        return None;
    }
}

pub fn save_device_info(ip: &str, name: &str) {
    let devices_file = get_devices_file();
    let mut devices = get_known_devices();
    devices.insert(ip.to_string(), name.to_string());

    let mut lines: Vec<String> = devices
        .iter()
        .map(|(ip, name)| format!("{},{}", ip, name))
        .collect();

    lines.insert(0, "IP,NAME".to_string());
    fs::write(&devices_file, lines.join("\n")).expect("Write failed");
}

pub fn rename_device_by_ip(ip: &str) {
    print!("Choose nickname for device: ");
    io::stdout().flush().ok(); // Force the `print!` to be displayed.

    let mut name_input = String::new();
    io::stdin()
        .read_line(&mut name_input)
        .expect("error: unable to read user input");
    let name = name_input.trim();
    let final_name = if name.is_empty() { "No nickname" } else { name };
    save_device_info(&ip, final_name);
}

// ADB FUNCTIONS ==============================================================

pub fn get_available_devices() -> HashMap<String, String> {
    let output = Command::new("adb")
        .arg("mdns")
        .arg("services")
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    get_device_map_from_string(&stdout)
}

pub fn get_connected_devices() -> HashMap<String, String> {
    let output = Command::new("adb")
        .arg("devices")
        .arg("-l")
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    get_device_map_from_string(&stdout)
}

// Returns the first USB ID
pub fn get_usb_device() -> Option<String> {
    let output = Command::new("adb")
        .arg("devices")
        .arg("-l")
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .lines()
        .find(|line| line.contains("usb:")) // Find the first line with "usb:"
        .and_then(|line| line.split_whitespace().next()) // Get the first word
        .map(|id| id.to_string()) // Convert
}

pub fn connect_to_ip(ip: &str) {
    let full_ip = format!("{}:5555", ip);
    Command::new("adb")
        .args(["connect", &full_ip])
        .status()
        .ok();
}

pub fn disconnect_from_ip(ip: &str) {
    let full_ip = format!("{}:5555", ip);
    Command::new("adb")
        .args(["disconnect", &full_ip])
        .status()
        .ok();
}

// DISPLAY ====================================================================

pub fn print_devices(devices: &HashMap<String, String>) {
    for (key, value) in devices {
        println!("\t{}\t{}", key, value);
    }
}

pub fn print_no_device() {
    println!("No devices available.");
    println!(
        "Try to turn wireless debugging off and on again then wait a couple seconds then reconnect."
    );
}

// PATHS ======================================================================

pub fn get_package_name() -> String {
    env!("CARGO_PKG_NAME").to_string()
}

/// Returns the path to the home directory.
pub fn get_home_dir() -> String {
    std::env::var("HOME").expect("Could not find HOME directory")
}

/// Returns the path to the home directory.
pub fn get_devices_file() -> String {
    let home = get_home_dir();
    let pkg = get_package_name();
    format!("{}/.local/state/{}/devices", home, pkg)
}

/// Returns the path to the home directory.
pub fn get_completions_file() -> String {
    let home = get_home_dir();
    let pkg = get_package_name();
    format!("{}/.local/share/bash-completion/completions/{}", home, pkg)
}

// FILES ======================================================================

pub fn write_pid_to_file(pid: u32, path: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(pid.to_string().as_bytes())?;
    Ok(())
}

// Utilitary ==================================================================

pub fn pick_a_device_then<F>(devices: &HashMap<String, String>, then: F)
where
    F: Fn(&str),
{
    if devices.is_empty() {
        print_no_device();
    } else if let Some(ip) = select_device(&devices) {
        return then(&ip);
    } else {
        println!("Could not retrieve selected device.")
    }
}

pub fn wait_for_input() {
    let _ = io::stdin().read_line(&mut String::new());
}

// Sleep for .5 second
pub fn give_it_a_sec() {
    sleep(Duration::from_millis(500));
}

// Sleep for 2 second
pub fn give_it_a_while() {
    sleep(Duration::from_millis(2000));
}
