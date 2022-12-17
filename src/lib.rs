mod machineid;

pub use machineid::get;
pub use machineid::get_via_linux_shell;
pub use machineid::get_via_macos_shell;
pub use machineid::get_via_windows_shell;
pub use machineid::transform_windows_output;
pub use machineid::MachineIdError;
