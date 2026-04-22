use crate::tools::{self};

// HIDDEN / UTILITY FUNCTIONS =================================================

pub fn print_raw_commands() {
    println!("adb mdns services");
    println!("adb devices -l");
    println!("adb -s DEVICE_ID usb");
    println!("adb -s DEVICE_ID tcpip 5555");
    println!("adb -s DEVICE_ID shell ip route get 1.1.1.1");
    println!("adb connect DEVICE_ID:5555");
    println!("adb disconnect DEVICE_ID:5555");
}

/// Generates bash completions file in `~/.local/share/bash-completion/completions/`
pub fn generate_bash_completions(mut cmd: clap::Command) {
    let path_str = tools::get_completions_file();
    let path = std::path::Path::new(&path_str);

    println!("Generating bash completions in [{}]...", &path_str);

    // Create directory if it doesn't exist
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create directory");
    }

    let mut file = std::fs::File::create(path).expect("Failed to create file");

    clap_complete::generate(
        clap_complete::Shell::Bash,
        &mut cmd,
        tools::get_package_name(),
        &mut file,
    );
    println!("Successfuly generated bash completions!");
}
