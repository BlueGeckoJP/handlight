use freedesktop_desktop_entry::DesktopEntry;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Category {
    Applications,
}

impl Category {
    pub fn display_name(&self) -> &str {
        match self {
            Self::Applications => "Applications",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    Application(AppInfo),
}

impl Item {
    pub fn category(&self) -> Category {
        match self {
            Self::Application(_) => Category::Applications,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Application(app) => &app.name,
        }
    }

    pub fn icon_name(&self) -> Option<&str> {
        match self {
            Self::Application(app) => app.icon_name.as_deref(),
        }
    }

    pub fn description(&self) -> Option<&str> {
        match self {
            Self::Application(app) => app.description.as_deref(),
        }
    }

    pub fn exec(&self) -> Option<&str> {
        match self {
            Self::Application(app) => app.exec.as_deref(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppInfo {
    pub name: String,
    pub icon_name: Option<String>,
    pub exec: Option<String>,
    pub description: Option<String>,
}

pub fn get_installed_apps() -> Vec<Item> {
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
                            apps.push(Item::Application(AppInfo {
                                name: name.to_string(),
                                icon_name: desktop_entry.icon().map(|s| s.to_string()),
                                exec: desktop_entry.exec().map(|s| s.to_string()),
                                description: desktop_entry
                                    .comment(&[] as &[&str])
                                    .map(|s| s.to_string()),
                            }));
                        }
                    }
                }
            }
        }
    }

    apps.sort_by(|a, b| match a.category().cmp(&b.category()) {
        std::cmp::Ordering::Equal => a.name().cmp(b.name()),
        other => other,
    });
    apps
}
