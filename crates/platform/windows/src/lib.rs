//! Windows-specific window capture using Win32 API.

use crate::platform::window::DesktopWindow;

pub struct Win32Capture;

impl Win32Capture {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self)
    }
}

impl platform::window::WindowCapture for Win32Capture {
    fn get_all_windows(&self) -> anyhow::Result<Vec<DesktopWindow>> {
        // Use EnumWindows + GetWindowRect + GetWindowText
        unimplemented!("Win32 window enumeration")
    }

    fn get_focused_window(&self) -> anyhow::Result<Option<DesktopWindow>> {
        unimplemented!("Get focused window")
    }

    fn monitor_changes(&self) -> anyhow::Result<()> {
        unimplemented!("Window change monitoring")
    }
}
