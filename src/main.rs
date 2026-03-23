mod app_info;

use std::process::{self, Stdio};

use app_info::{AppInfo, get_installed_apps};
use gtk4::prelude::*;
use gtk4::{gdk, glib};
use relm4::factory::FactoryVecDeque;
use relm4::prelude::*;

struct AppRow {
    app: AppInfo,
}

#[derive(Debug)]
enum AppRowMsg {}

#[derive(Debug)]
enum AppRowOutput {}

#[relm4::factory]
impl FactoryComponent for AppRow {
    type Init = AppInfo;
    type Input = AppRowMsg;
    type Output = AppRowOutput;
    type CommandOutput = ();
    type ParentWidget = gtk4::ListBox;

    view! {
        gtk4::Box {
            set_orientation: gtk4::Orientation::Horizontal,
            set_spacing: 12,
            set_margin_all: 8,

            gtk4::Image {
                set_icon_name: Some(self.app.icon_name.as_deref().unwrap_or("application-x-executable")),
                set_pixel_size: 32,
            },

            gtk4::Label {
                #[watch]
                set_label: &self.app.name,
                set_halign: gtk4::Align::Start,
            }
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        AppRow { app: init }
    }

    fn update(&mut self, _msg: Self::Input, _sender: FactorySender<Self>) {}
}

struct App {
    selected_index: i32,
    all_apps: Vec<AppInfo>,
    filtered_apps: FactoryVecDeque<AppRow>,
    filtered_apps_data: Vec<AppInfo>,
}

#[derive(Debug)]
enum Msg {
    MoveSelectionUp,
    MoveSelectionDown,
    SearchChanged(String),
    AppClicked(i32),
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = Msg;
    type Output = ();

    view! {
        gtk4::Window {
            set_decorated: false,
            add_css_class: "transparent-window",
            set_default_size: (600, 400),

            gtk4::Box {
                set_orientation: gtk4::Orientation::Vertical,
                add_css_class: "ui-container",

                gtk4::SearchEntry {
                    set_hexpand: true,
                    connect_search_changed[sender] => move |entry| {
                        sender.input(Msg::SearchChanged(entry.text().to_string()));
                    },
                    add_controller = gtk4::EventControllerKey {
                        connect_key_pressed[sender] => move |_, keyval, _, _| {
                            match keyval {
                                gdk::Key::Up => {
                                    sender.input(Msg::MoveSelectionUp);
                                    glib::Propagation::Stop
                                }
                                gdk::Key::Down => {
                                    sender.input(Msg::MoveSelectionDown);
                                    glib::Propagation::Stop
                                }
                                _ => glib::Propagation::Proceed,
                            }
                        }
                    }
                },

                gtk4::Paned {
                    set_orientation: gtk4::Orientation::Horizontal,
                    set_vexpand: true,
                    set_position: 250,

                    #[wrap(Some)]
                    set_start_child = &gtk4::ScrolledWindow {
                        set_policy: (gtk4::PolicyType::Never, gtk4::PolicyType::Automatic),
                        #[local_ref]
                        list_box -> gtk4::ListBox {
                            set_selection_mode: gtk4::SelectionMode::Single,
                            set_activate_on_single_click: false,
                            connect_row_activated[sender] => move |_, row| {
                                sender.input(Msg::AppClicked(row.index()));
                            }
                        }
                    },

                    #[wrap(Some)]
                    set_end_child = &gtk4::ScrolledWindow {
                        set_policy: (gtk4::PolicyType::Automatic, gtk4::PolicyType::Automatic),
                        gtk4::Box {
                            set_orientation: gtk4::Orientation::Vertical,
                        }
                    }
                }
            }
        }
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let all_apps = get_installed_apps();

        // Initialize the filtered list to show all apps initially
        let mut filtered_apps = FactoryVecDeque::builder()
            .launch(gtk4::ListBox::default())
            .forward(sender.input_sender(), |_| unreachable!());

        {
            let mut guard = filtered_apps.guard();
            for app in &all_apps {
                guard.push_back(app.clone());
            }
        }

        let filtered_apps_data = all_apps.clone();

        let model = App {
            selected_index: 0,
            all_apps,
            filtered_apps,
            filtered_apps_data,
        };

        let list_box = model.filtered_apps.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            Msg::MoveSelectionUp => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            Msg::MoveSelectionDown => {
                self.selected_index += 1;
            }
            Msg::SearchChanged(query) => {
                let lower_query = query.to_lowercase();
                let mut guard = self.filtered_apps.guard();
                guard.clear();
                self.filtered_apps_data.clear();

                for app in &self.all_apps {
                    if app.name.to_lowercase().contains(&lower_query) {
                        guard.push_back(app.clone());
                        self.filtered_apps_data.push(app.clone());
                    }
                }
            }
            Msg::AppClicked(index) => {
                if let Some(app) = self.filtered_apps_data.get(index as usize) {
                    if let Some(exec) = &app.exec {
                        let exec_clean = exec
                            .replace("%u", "")
                            .replace("%U", "")
                            .replace("%f", "")
                            .replace("%F", "")
                            .replace("%c", &app.name)
                            .replace("%k", "");

                        let _ = std::process::Command::new("sh")
                            .arg("-c")
                            .arg(&exec_clean)
                            .stderr(Stdio::null())
                            .stdout(Stdio::null())
                            .stdin(Stdio::null())
                            .spawn();

                        process::exit(0);
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let app = RelmApp::new("me.bluegecko.handlight");

    relm4::set_global_css(
        r#"
            window.transparent-window {
                background-color: transparent;
            }
            .ui-container {
                background-color: @window_bg_color;
                border-radius: 12px;
            }
        "#,
    );

    app.run::<App>(());
}
