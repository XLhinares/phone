use clap::{CommandFactory, Parser, Subcommand};

mod camera;
mod commands;
mod device;
mod tools;

#[derive(Parser)]
#[command(version, about, author)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all pairable devices
    Available,
    /// List all connected devices
    Connected,
    /// Whether at least one device is currently connected.
    IsConnected,
    /// Pairs a new device
    New,
    /// Rename a known device
    Rename,
    /// Connect a selected device
    Connect,
    /// Disconnect a connected device
    Disconnect,
    /// Connect or disconnect the device.
    ToggleConnection,
    /// Mirror display via scrcpy
    Mirror,
    /// Use device as virtual camera
    Camera {
        #[command(subcommand)]
        command: CameraCommands,
    },

    /// [Hidden] Prints a list of adb shell commands for reference
    #[command(hide = true)]
    RawCommands,
    /// [Hidden] Generates bash completion file.
    #[command(hide = true)]
    BashCompletions,
}

#[derive(Subcommand)]
enum CameraCommands {
    /// Enable the device as a virtual camera
    Enable,
    /// Disable the device as a virtual camera
    Disable,
    /// Toggle the device as a virtual camera
    Toggle,
    /// Returns whether a device is currently used as camera
    IsEnabled,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Available => device::available(),
        Commands::Connected => device::connected(),
        Commands::IsConnected => device::print_is_connected(),
        Commands::New => device::new_device(),
        Commands::Rename => device::rename_known_device(),
        Commands::Connect => device::connect(),
        Commands::Disconnect => device::disconnect(),
        Commands::ToggleConnection => device::toggle_connection(),
        Commands::Mirror => device::mirror_device(),
        Commands::Camera { command } => match command {
            CameraCommands::Enable => camera::enable(),
            CameraCommands::Disable => camera::disable(),
            CameraCommands::Toggle => camera::toggle(),
            CameraCommands::IsEnabled => camera::print_is_enabled(),
        },
        Commands::RawCommands => commands::print_raw_commands(),
        Commands::BashCompletions => commands::generate_bash_completions(Cli::command()),
    }
}
