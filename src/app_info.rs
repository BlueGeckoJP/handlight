use freedesktop_desktop_entry::DesktopEntry;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppInfo {
    pub name: String,
    pub icon_name: Option<String>,
    pub exec: Option<String>,
}

pub fn get_installed_apps() -> Vec<AppInfo> {
    let mut apps = Vec::new();

    let mut dirs = vec![PathBuf::from("/usr/share/applications")];
    if let Ok(home) = std::env::var("HOME") {
        dirs.push(PathBuf::from(home).join(".local/share/applications"));
    }

    for dir in dirs {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                if entry.path().extension().is_some_and(|ext| ext == "desktop") {
                    let path = entry.path();
                    if let Ok(desktop_entry) = DesktopEntry::from_path(&path, None::<&[String]>) {
                        if desktop_entry.no_display() {
                            continue;
                        }
                        if let Some(name) = desktop_entry.name(&[] as &[&str]) {
                            apps.push(AppInfo {
                                name: name.to_string(),
                                icon_name: desktop_entry.icon().map(|s| s.to_string()),
                                exec: desktop_entry.exec().map(|s| s.to_string()),
                            });
                        }
                    }
                }
            }
        }
    }

    apps.sort_by(|a, b| a.name.cmp(&b.name));
    apps
}
