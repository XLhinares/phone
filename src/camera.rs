use crate::tools::{self};
use std::{
    fs::read_to_string,
    io::{self, Error, ErrorKind, Write},
    process::{Child, Command, Stdio},
};

// PUBLIC =====================================================================

pub fn enable() {
    // Your implementation for enabling the camera
    enable_with_camera_choice(Some(true)).expect("Error enabling the camera.");
}

pub fn disable() {
    let pid: u32 = read_to_string("/tmp/phone.camera.pid")
        .expect("Could not find pid.")
        .trim()
        .parse()
        .expect("Could not parse pid.");
    Command::new("kill")
        .arg(pid.to_string())
        .spawn()
        .expect("Could not spawn kill process.")
        .wait()
        .expect("Waiting for kill process to end failed.");
}

pub fn toggle() {
    // Your implementation for enabling the camera
    let currently_enabled = is_enabled();
    return if currently_enabled {
        disable();
    } else {
        enable_with_camera_choice(Some(false)).expect("Error enabling the camera.");
    };
}

pub fn print_is_enabled() {
    // Your implementation for disabling the camera
    let currently_enabled = is_enabled();
    println!("{}", currently_enabled);
}

// PRIVATE ====================================================================

fn enable_with_camera_choice(use_default_camera: Option<bool>) -> std::io::Result<()> {
    let devices = tools::get_connected_devices();
    let selected_device = tools::select_device(&devices);
    if selected_device.is_none() {
        return Err(Error::new(ErrorKind::Other, "No device selected."));
    }
    let child = start_camera(&selected_device.unwrap(), use_default_camera)?;
    tools::write_pid_to_file(child.id(), "/tmp/phone.camera.pid")?;
    Ok(())
}

fn is_enabled() -> bool {
    let ps = Command::new("ps")
        .args(["-x"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to fetch processes.");

    let grep = Command::new("grep")
        .args(["scrcpy.*video_source=camera"])
        .stdin(Stdio::from(ps.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to find match.");

    // Wait for grep to finish and get its output
    let output = grep.wait_with_output().expect("Failed to wait for grep.");

    // Count the number of lines in the output (number of matches)
    let matches = String::from_utf8_lossy(&output.stdout);
    let number_of_matches = matches.lines().count();

    // print!("Matches (): {}", number_of_matches, matches);

    // If the camera is enabled, the grep should have caught three processes:
    // - the grep itself
    // - this function (is_enabled)
    // - the camera function
    return number_of_matches >= 2;
}

// SCRCPY =====================================================================

pub fn start_camera(ip: &str, use_default_camera: Option<bool>) -> std::io::Result<Child> {
    let camera_id = if use_default_camera.unwrap_or(false) {
        select_camera(&ip).expect("Failed to get camera ID.")
    } else {
        1
    };
    let camera_arg = format!("--camera-id={}", camera_id);
    Command::new("scrcpy")
        .args([
            "-s",
            &ip,
            "--video-source=camera",
            &camera_arg,
            "--camera-size=1920x1080",
            "--v4l2-sink=/dev/video0",
            "--no-audio",
            "--no-window",
        ])
        .spawn()
}

fn select_camera(ip: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let default_camera = 1;
    let input_msg = format!("Use default camera ({})? [Y/n] ", default_camera);
    print!("{}", input_msg);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim().to_lowercase();
    if trimmed != "n" {
        return Ok(default_camera);
    }

    Command::new("scrcpy")
        .args(["-s", &ip, "--list-cameras"])
        .status()
        .unwrap();

    print!("Enter selected camera ID: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let idx: u32 = input.trim().parse()?;

    return Ok(idx);
}
