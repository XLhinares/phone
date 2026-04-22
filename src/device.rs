use crate::tools::{self};
use std::process::Command;

// GET LIST OF DEVICES ========================================================

pub fn available() {
    let devices = tools::get_available_devices();

    if devices.is_empty() {
        tools::print_no_device();
        return;
    }

    println!("The following devices are available:");
    tools::print_devices(&devices);
}

pub fn connected() {
    let devices = tools::get_connected_devices();

    if devices.is_empty() {
        println!("No devices connected.");
    } else {
        println!("The following devices are connected:");
        tools::print_devices(&devices);
    }
}

pub fn print_is_connected() {
    let is_connected = check_if_connected();
    print!("{}", is_connected);
}

// ADD / EDIT DEVICES =========================================================

pub fn new_device() {
    println!("Enable the wireless debug in the device's developer settings");
    println!("Plug the device in then press [Enter].");
    tools::wait_for_input();
    if let Some(id) = tools::get_usb_device() {
        println!("Found device: {}", id);

        // Switch to USB mode
        Command::new("adb").args(["-s", &id, "usb"]).status().ok();

        tools::give_it_a_while();

        // Switch to TCPIP mode
        Command::new("adb")
            .args(["-s", &id, "tcpip", "5555"])
            .status()
            .ok();

        tools::give_it_a_while();

        // Replace your ip_out block with this:
        let mut ip = String::new();
        println!("Waiting for device to provide IP...");

        for _ in 0..10 {
            // Try for 10 seconds
            let ip_out = Command::new("adb")
                .args(["-s", &id, "shell", "ip", "route", "get", "1.1.1.1"])
                .output()
                .expect("Failed to execute adb command");

            let output_str = String::from_utf8_lossy(&ip_out.stdout);
            ip = output_str
                .split_whitespace()
                .zip(output_str.split_whitespace().skip(1))
                .find(|(word, _)| *word == "src")
                .map(|(_, next)| next.to_string())
                .unwrap_or_default();

            if !ip.is_empty() {
                break;
            }

            tools::give_it_a_sec();
        }

        if ip.is_empty() {
            println!("Could not establish connection to device.");
            return;
        }

        println!("Device IP: {}", ip);
        tools::rename_device_by_ip(&ip);

        println!("Unplug device and press [Enter]");
        tools::wait_for_input();

        tools::connect_to_ip(&ip);
    }
}

pub fn rename_known_device() {
    let devices = tools::get_known_devices();
    if let Some(ip) = tools::select_device(&devices) {
        tools::rename_device_by_ip(&ip);
    } else {
        println!("Could not rename device");
    }
}

// CONNECT / DISCONNECT =======================================================

/// Selects an available device and connects to it.
///
/// If only one device is available, tries to connect directly.
pub fn connect() {
    let devices = tools::get_available_devices();
    tools::pick_a_device_then(&devices, tools::connect_to_ip);
}

/// Selects a connected device and disconnects from it.
pub fn disconnect() {
    let devices = tools::get_connected_devices();
    tools::pick_a_device_then(&devices, tools::disconnect_from_ip);
}

pub fn toggle_connection() {
    if check_if_connected() {
        return connect();
    } else {
        return disconnect();
    }
}

// SCRCPY =====================================================================

pub fn mirror_device() {
    let devices = tools::get_connected_devices();
    tools::pick_a_device_then(&devices, |ip| {
        Command::new("scrcpy")
            .args(["-s", &ip])
            .spawn()
            .expect("Failed to start scrcpy");
    });
}

// PRIVATE ====================================================================

fn check_if_connected() -> bool {
    let devices = tools::get_connected_devices();
    return devices.len() > 0;
}
