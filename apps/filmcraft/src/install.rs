//! `trackline --install`: install this build under the user's `~/.local`.
//!
//! The flag is meant for the Linux standalone tree and the AppImage, where there is no installer.
//! It copies the binary (or the AppImage) into `~/.local/bin`, writes the packaged `.desktop`
//! entry with an absolute `Exec=`, and unpacks the hicolor icon PNGs (and the scalable SVG) under
//! `~/.local/share/icons/hicolor`, so the entry and its icon show in the app menu and file manager
//! without a `sudo` package install. Everything goes under `$HOME` (the data files under
//! `$XDG_DATA_HOME`, else `~/.local/share`), nothing system-wide, and nothing is written on any
//! other platform.

#[cfg(not(target_os = "linux"))]
pub fn install() -> Result<String, String> {
    Err("--install is only supported on Linux".into())
}

#[cfg(target_os = "linux")]
pub(crate) use linux::install;

#[cfg(target_os = "linux")]
mod linux {
    use std::os::unix::fs::PermissionsExt;
    use std::path::{Path, PathBuf};

    use crate::APP_ID;

    /// The hicolor PNG sizes written for the app menu, file manager and taskbar. Embedded, so a
    /// release binary installs its own icons with no checkout or AppDir beside it.
    const ICONS: &[(&str, &[u8])] = &[
        ("16x16", include_bytes!("../../../assets/app-icon/hicolor/16x16/apps/ru.illoprin.trackline.png")),
        ("24x24", include_bytes!("../../../assets/app-icon/hicolor/24x24/apps/ru.illoprin.trackline.png")),
        ("32x32", include_bytes!("../../../assets/app-icon/hicolor/32x32/apps/ru.illoprin.trackline.png")),
        ("48x48", include_bytes!("../../../assets/app-icon/hicolor/48x48/apps/ru.illoprin.trackline.png")),
        ("64x64", include_bytes!("../../../assets/app-icon/hicolor/64x64/apps/ru.illoprin.trackline.png")),
        ("128x128", include_bytes!("../../../assets/app-icon/hicolor/128x128/apps/ru.illoprin.trackline.png")),
        ("256x256", include_bytes!("../../../assets/app-icon/hicolor/256x256/apps/ru.illoprin.trackline.png")),
        ("512x512", include_bytes!("../../../assets/app-icon/hicolor/512x512/apps/ru.illoprin.trackline.png")),
    ];

    /// The scalable icon, for sizes the PNGs don't cover.
    const SCALABLE: &[u8] = include_bytes!("../../../assets/app-icon/hicolor/scalable/apps/ru.illoprin.trackline.svg");

    /// The packaged desktop entry, with `Exec=`/`TryExec=` pointed at the installed launcher.
    const DESKTOP: &str = include_str!("../../../packaging/linux/ru.illoprin.trackline.desktop");

    /// Install this build for the current user and return a message to print.
    pub fn install() -> Result<String, String> {
        let home = home_dir()?;
        let (source, name, is_appimage) = source_build()?;
        let bin_dir = home.join(".local").join("bin");
        let data_dir = crate::dev_icon::user_data_dir().map_err(|e| format!("cannot find the data directory: {e}"))?;
        let launcher = install_into(&bin_dir, &data_dir, &source, name, is_appimage)?;
        refresh_caches(&data_dir);
        Ok(format!("Installed Trackline in {}\nLaunch it from the application menu or run \"{}\".\n", bin_dir.display(), launcher.display()))
    }

    /// `$HOME`, absolute, as the XDG spec requires; never a guess that would write into the working
    /// directory.
    fn home_dir() -> Result<PathBuf, String> {
        match std::env::var_os("HOME") {
            Some(h) if !h.is_empty() && Path::new(&h).is_absolute() => Ok(PathBuf::from(h)),
            _ => Err("HOME is not set to an absolute path".into()),
        }
    }

    /// The build to install: the AppImage when we are one (`$APPIMAGE` is set by its runtime),
    /// else this very executable.
    fn source_build() -> Result<(PathBuf, &'static str, bool), String> {
        if let Some(appimage) = std::env::var_os("APPIMAGE").filter(|p| !p.is_empty()) {
            return Ok((PathBuf::from(appimage), "trackline.AppImage", true));
        }
        let exe = std::env::current_exe().map_err(|e| format!("cannot find this executable: {e}"))?;
        Ok((exe, "trackline", false))
    }

    /// Copy `source` into `bin_dir` and write the desktop entry and icons under `data_dir`. Returns
    /// the launcher the desktop entry should run (the `trackline` symlink for an AppImage).
    fn install_into(bin_dir: &Path, data_dir: &Path, source: &Path, name: &str, appimage: bool) -> Result<PathBuf, String> {
        let launcher = install_binary(bin_dir, source, name, appimage)?;
        write_desktop(data_dir, &launcher)?;
        write_icons(data_dir)?;
        Ok(launcher)
    }

    /// Copy the build into `bin_dir/<name>` and make it executable; for an AppImage also leave a
    /// `trackline` symlink beside it, so a terminal can run it by name.
    fn install_binary(bin_dir: &Path, source: &Path, name: &str, appimage: bool) -> Result<PathBuf, String> {
        std::fs::create_dir_all(bin_dir).map_err(|e| format!("{}: {e}", bin_dir.display()))?;
        let dest = bin_dir.join(name);
        // Another install may have left `dest` as a symlink (an AppImage); remove it so the copy
        // lands in `bin_dir` instead of writing through the link.
        if std::fs::symlink_metadata(&dest).is_ok_and(|m| m.file_type().is_symlink()) {
            std::fs::remove_file(&dest).map_err(|e| format!("{}: {e}", dest.display()))?;
        }
        // Re-installing over itself cannot copy a file onto itself.
        let same = std::fs::canonicalize(source).ok().zip(std::fs::canonicalize(&dest).ok()).is_some_and(|(a, b)| a == b);
        if !same {
            std::fs::copy(source, &dest).map_err(|e| format!("copying {} to {}: {e}", source.display(), dest.display()))?;
        }
        std::fs::set_permissions(&dest, std::fs::Permissions::from_mode(0o755)).map_err(|e| format!("{}: {e}", dest.display()))?;
        if appimage {
            let link = bin_dir.join("trackline");
            let _ = std::fs::remove_file(&link);
            std::os::unix::fs::symlink(name, &link).map_err(|e| format!("{}: {e}", link.display()))?;
            return Ok(link);
        }
        Ok(dest)
    }

    /// Write `~/.local/share/applications/<app id>.desktop` with the launcher in `Exec=`.
    fn write_desktop(data_dir: &Path, launcher: &Path) -> Result<(), String> {
        let exe = launcher.to_str().ok_or("the install path is not valid UTF-8")?;
        if exe.contains('=') || exe.contains('%') || exe.chars().any(char::is_control) {
            // A desktop entry cannot carry these: '=' separates key and value, '%' introduces a
            // field code, control characters are invalid in the string layer (install.sh agrees).
            return Err("the install path has a character a desktop entry cannot carry (=, % or a control character)".into());
        }
        let applications = data_dir.join("applications");
        std::fs::create_dir_all(&applications).map_err(|e| format!("{}: {e}", applications.display()))?;
        let path = applications.join(format!("{APP_ID}.desktop"));
        std::fs::write(&path, desktop_entry(exe)).map_err(|e| format!("{}: {e}", path.display()))
    }

    /// The packaged `.desktop`, with `Exec` (`"<path>" %F`) and `TryExec` pointed at `exe`. Mirrors
    /// `packaging/linux/install.sh`: the shell layer escapes the argument, the string layer doubles
    /// every backslash.
    fn desktop_entry(exe: &str) -> String {
        let exec = crate::dev_icon::escape_exec_argument(exe);
        let try_exec = exe.replace('\\', "\\\\");
        let mut out = String::with_capacity(DESKTOP.len() + 2 * exe.len());
        for line in DESKTOP.lines() {
            if line.starts_with("Exec=") {
                out.push_str("Exec=\"");
                out.push_str(&exec);
                out.push_str("\" %F\n");
            } else if line.starts_with("TryExec=") {
                out.push_str("TryExec=");
                out.push_str(&try_exec);
                out.push('\n');
            } else {
                out.push_str(line);
                out.push('\n');
            }
        }
        out
    }

    /// Write every hicolor icon size and the scalable SVG under `data_dir/icons/hicolor`.
    fn write_icons(data_dir: &Path) -> Result<(), String> {
        for (size, bytes) in ICONS {
            let dir = data_dir.join("icons").join("hicolor").join(size).join("apps");
            std::fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
            let path = dir.join(format!("{APP_ID}.png"));
            std::fs::write(&path, bytes).map_err(|e| format!("{}: {e}", path.display()))?;
        }
        let dir = data_dir.join("icons").join("hicolor").join("scalable").join("apps");
        std::fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
        let path = dir.join(format!("{APP_ID}.svg"));
        std::fs::write(&path, SCALABLE).map_err(|e| format!("{}: {e}", path.display()))
    }

    /// Best effort: refresh the shell's desktop-entry cache so the entry appears without a
    /// re-login. A missing tool is fine (the shell rebuilds its cache on its own schedule).
    fn refresh_caches(data_dir: &Path) {
        let applications = data_dir.join("applications");
        if let Err(e) = std::process::Command::new("update-desktop-database").arg("-q").arg(&applications).status() {
            log::debug!("update-desktop-database not run: {e}");
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::sync::atomic::{AtomicU32, Ordering};

        /// A fresh directory under the system temp dir; removed before use so a crashed earlier
        /// run cannot leave state behind.
        fn temp_root() -> PathBuf {
            static N: AtomicU32 = AtomicU32::new(0);
            let dir = std::env::temp_dir().join(format!("trackline-install-{}-{}", std::process::id(), N.fetch_add(1, Ordering::Relaxed)));
            let _ = std::fs::remove_dir_all(&dir);
            std::fs::create_dir_all(&dir).unwrap();
            dir
        }

        /// A standalone binary install writes the binary, the desktop entry and every icon.
        #[test]
        fn installing_a_binary_writes_bin_desktop_and_icons() {
            let root = temp_root();
            let (bin, data) = (root.join("bin"), root.join("share"));
            let source = root.join("trackline");
            std::fs::write(&source, b"#!/bin/sh\n").unwrap();

            let launcher = install_into(&bin, &data, &source, "trackline", false).unwrap();

            assert_eq!(launcher, bin.join("trackline"));
            assert_eq!(std::fs::read(&launcher).unwrap(), b"#!/bin/sh\n");
            assert_eq!(std::fs::metadata(&launcher).unwrap().permissions().mode() & 0o111, 0o111, "the launcher is executable");

            let desktop = std::fs::read_to_string(data.join("applications").join(format!("{APP_ID}.desktop"))).unwrap();
            assert!(desktop.contains(&format!("Exec=\"{}\" %F", launcher.display())), "{desktop}");
            assert!(desktop.contains(&format!("TryExec={}", launcher.display())), "{desktop}");

            for (size, _) in ICONS {
                let p = data.join("icons/hicolor").join(size).join("apps").join(format!("{APP_ID}.png"));
                assert!(p.is_file(), "{}", p.display());
            }
            assert!(data.join("icons/hicolor/scalable/apps").join(format!("{APP_ID}.svg")).is_file());

            let _ = std::fs::remove_dir_all(&root);
        }

        /// An AppImage is kept under its `.AppImage` name and a `trackline` symlink points at it;
        /// the desktop entry runs the symlink.
        #[test]
        fn installing_an_appimage_keeps_it_and_links_trackline() {
            let root = temp_root();
            let (bin, data) = (root.join("bin"), root.join("share"));
            let source = root.join("Trackline-x86_64.AppImage");
            std::fs::write(&source, b"appimage-bytes").unwrap();

            let launcher = install_into(&bin, &data, &source, "trackline.AppImage", true).unwrap();

            assert_eq!(launcher, bin.join("trackline"));
            assert_eq!(std::fs::read(bin.join("trackline.AppImage")).unwrap(), b"appimage-bytes");
            assert_eq!(std::fs::read_link(&launcher).unwrap(), PathBuf::from("trackline.AppImage"));
            let desktop = std::fs::read_to_string(data.join("applications").join(format!("{APP_ID}.desktop"))).unwrap();
            assert!(desktop.contains(&format!("Exec=\"{}\" %F", launcher.display())), "{desktop}");

            let _ = std::fs::remove_dir_all(&root);
        }

        /// The written entry is the packaged file with `Exec`/`TryExec` substituted, so the two
        /// cannot drift.
        #[test]
        fn desktop_entry_is_the_packaged_file_with_the_path_substituted() {
            let body = desktop_entry("/home/u/.local/bin/trackline");
            let expected = DESKTOP
                .lines()
                .map(|l| {
                    if l.starts_with("Exec=") {
                        "Exec=\"/home/u/.local/bin/trackline\" %F"
                    } else if l.starts_with("TryExec=") {
                        "TryExec=/home/u/.local/bin/trackline"
                    } else {
                        l
                    }
                })
                .collect::<Vec<_>>()
                .join("\n")
                + "\n";
            assert_eq!(body, expected);
        }
    }
}
