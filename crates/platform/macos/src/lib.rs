//! macOS-specific window capture using Accessibility API.

use crate::platform::window::DesktopWindow;

pub struct MacOSAccessibilityCapture;

impl MacOSAccessibilityCapture {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self)
    }
}

impl platform::window::WindowCapture for MacOSAccessibilityCapture {
    fn get_all_windows(&self) -> anyhow::Result<Vec<DesktopWindow>> {
        // Use AXUIElementCopyAttributeValue with kAXWindowsAttribute
        unimplemented!("macOS window enumeration")
    }

    fn get_focused_window(&self) -> anyhow::Result<Option<DesktopWindow>> {
        unimplemented!("Get focused window")
    }

    fn monitor_changes(&self) -> anyhow::Result<()> {
        unimplemented!("Window change monitoring")
    }
}
