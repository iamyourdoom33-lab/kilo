//! Platform abstraction layer for window capture
//!
//! Provides unified API across Windows, macOS, and Linux
//! for enumerating windows and tracking their properties.

pub mod window {
    pub struct DesktopWindow {
        pub id: u64,
        pub title: String,
        pub rect: (i32, i32, i32, i32), // x, y, width, height
        pub is_visible: bool,
        pub is_minimized: bool,
        pub owner_pid: u32,
    }

    pub trait WindowCapture {
        fn get_all_windows(&self) -> anyhow::Result<Vec<DesktopWindow>>;
        fn get_focused_window(&self) -> anyhow::Result<Option<DesktopWindow>>;
        fn monitor_changes(&self) -> anyhow::Result<()>;
    }
}

pub mod platform {
    #[cfg(target_os = "windows")]
    pub type PlatformImpl = windows::win32_window::Win32Capture;

    #[cfg(target_os = "macos")]
    pub type PlatformImpl = macos::accessibility::MacOSAccessibilityCapture;

    #[cfg(all(unix, not(target_os = "macos")))]
    pub type PlatformImpl = linux::x11_window::X11Capture;

    pub fn create_capture() -> anyhow::Result<Box<dyn window::WindowCapture>> {
        Ok(Box::new(PlatformImpl::new()?))
    }
}
