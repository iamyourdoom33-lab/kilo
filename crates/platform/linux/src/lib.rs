//! Linux-specific window capture (X11 + Wayland support).

use crate::platform::window::DesktopWindow;

pub struct X11Capture;

impl X11Capture {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self)
    }
}

impl platform::window::WindowCapture for X11Capture {
    fn get_all_windows(&self) -> anyhow::Result<Vec<DesktopWindow>> {
        // Use XQueryTree with _NET_CLIENT_LIST property
        unimplemented!("X11 window enumeration")
    }

    fn get_focused_window(&self) -> anyhow::Result<Option<DesktopWindow>> {
        unimplemented!("Get focused window")
    }

    fn monitor_changes(&self) -> anyhow::Result<()> {
        unimplemented!("Window change monitoring")
    }
}
