//! "Start with Windows": a per-user Run key that launches the app with `--hidden`, so it
//! comes up straight in the tray while its local MCP server becomes reachable for agents.
//!
//! The value name and command line are stable across builds: the tray menu reads the same
//! key it writes, so the check mark always reflects what Windows will actually run.

#[cfg(windows)]
mod platform {
    use std::io;

    use winreg::enums::{HKEY_CURRENT_USER, KEY_READ};
    use winreg::RegKey;

    const RUN_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
    /// Value name shown in Task Manager's Startup tab.
    const VALUE_NAME: &str = "OpenPencil";
    /// The flag the app reads in `setup` to stay hidden on launch.
    const HIDDEN_FLAG: &str = "--hidden";

    pub fn is_enabled() -> bool {
        RegKey::predef(HKEY_CURRENT_USER)
            .open_subkey_with_flags(RUN_KEY, KEY_READ)
            .and_then(|key| key.get_value::<String, _>(VALUE_NAME))
            .is_ok()
    }

    pub fn set_enabled(enabled: bool) -> io::Result<()> {
        let (key, _) = RegKey::predef(HKEY_CURRENT_USER).create_subkey(RUN_KEY)?;
        if !enabled {
            return match key.delete_value(VALUE_NAME) {
                Ok(()) => Ok(()),
                Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
                Err(error) => Err(error),
            };
        }

        let executable = std::env::current_exe()?;
        let command = format!("\"{}\" {HIDDEN_FLAG}", executable.display());
        key.set_value(VALUE_NAME, &command)
    }
}

#[cfg(not(windows))]
mod platform {
    use std::io;

    pub fn is_enabled() -> bool {
        false
    }

    pub fn set_enabled(_enabled: bool) -> io::Result<()> {
        Ok(())
    }
}

pub use platform::{is_enabled, set_enabled};
