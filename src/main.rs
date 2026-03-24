mod app_info;

use std::process::{self, Stdio};

use app_info::{AppInfo, Item, get_installed_apps};
use gtk4::prelude::*;
use gtk4::{gdk, glib};
use relm4::factory::FactoryVecDeque;
use relm4::prelude::*;

struct AppRow {
    app: Item,
}

#[derive(Debug)]
enum AppRowMsg {}

#[derive(Debug)]
enum AppRowOutput {}

#[relm4::factory]
impl FactoryComponent for AppRow {
    type Init = Item;
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
                set_icon_name: Some(self.app.icon_name().unwrap_or("application-x-executable")),
                set_pixel_size: 32,
            },

            gtk4::Label {
                #[watch]
                set_label: self.app.name(),
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
    RowSelected(Option<i32>),
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
            set_default_size: (700, 450),

            connect_is_active_notify => move |window| {
                if !window.is_active() {
                    std::process::exit(0);
                }
            },

            add_controller = gtk4::EventControllerKey {
                set_propagation_phase: gtk4::PropagationPhase::Capture,
                connect_key_pressed => move |_, keyval, _, _| {
                    if keyval == gdk::Key::Escape {
                        std::process::exit(0);
                    }
                    glib::Propagation::Proceed
                }
            },

            gtk4::Box {
                set_orientation: gtk4::Orientation::Vertical,
                add_css_class: "ui-container",

                gtk4::SearchEntry {
                    set_hexpand: true,
                    set_margin_all: 8,
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
                                gdk::Key::Return => {
                                    sender.input(Msg::AppClicked(-1)); // -1 means use current selection
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
                            },
                            connect_row_selected[sender] => move |_, row| {
                                sender.input(Msg::RowSelected(row.map(|r| r.index())));
                            }
                        }
                    },

                    #[wrap(Some)]
                    set_end_child = &gtk4::Box {
                        set_orientation: gtk4::Orientation::Vertical,
                        set_valign: gtk4::Align::Center,
                        set_halign: gtk4::Align::Center,
                        set_spacing: 12,
                        set_margin_all: 24,
                        set_hexpand: true,

                        gtk4::Image {
                            set_pixel_size: 128,
                            #[watch]
                            set_icon_name: model.filtered_apps_data.get(model.selected_index as usize)
                                .and_then(|a| a.icon_name.as_deref())
                                .or(Some("application-x-executable")),
                        },

                        gtk4::Label {
                            add_css_class: "app-title",
                            #[watch]
                            set_label: model.filtered_apps_data.get(model.selected_index as usize)
                                .map(|a| a.name.as_str())
                                .unwrap_or(""),
                        },

                        gtk4::Label {
                            add_css_class: "dim-label",
                            set_wrap: true,
                            set_max_width_chars: 40,
                            set_justify: gtk4::Justification::Center,
                            #[watch]
                            set_label: model.filtered_apps_data.get(model.selected_index as usize)
                                .and_then(|a| a.description.as_deref())
                                .unwrap_or(""),
                        },

                        gtk4::Label {
                            add_css_class: "dim-label-monospace",
                            set_wrap: true,
                            set_max_width_chars: 40,
                            set_ellipsize: gtk4::pango::EllipsizeMode::End,
                            #[watch]
                            set_label: model.filtered_apps_data.get(model.selected_index as usize)
                                .and_then(|a| a.exec.as_deref())
                                .unwrap_or(""),
                        },
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
        // Select the first row by default
        if let Some(row) = list_box.row_at_index(0) {
            list_box.select_row(Some(&row));
        }

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            Msg::MoveSelectionUp => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                    let list_box = self.filtered_apps.widget();
                    if let Some(row) = list_box.row_at_index(self.selected_index) {
                        list_box.select_row(Some(&row));
                    }
                }
            }
            Msg::MoveSelectionDown => {
                if (self.selected_index as usize) < self.filtered_apps_data.len() - 1 {
                    self.selected_index += 1;
                    let list_box = self.filtered_apps.widget();
                    if let Some(row) = list_box.row_at_index(self.selected_index) {
                        list_box.select_row(Some(&row));
                    }
                }
            }
            Msg::SearchChanged(query) => {
                let lower_query = query.to_lowercase();
                {
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
                self.selected_index = 0;
                let list_box = self.filtered_apps.widget();
                if let Some(row) = list_box.row_at_index(0) {
                    list_box.select_row(Some(&row));
                }
            }
            Msg::RowSelected(index) => {
                if let Some(idx) = index {
                    self.selected_index = idx;
                }
            }
            Msg::AppClicked(index) => {
                let target_index = if index == -1 { self.selected_index } else { index };
                if let Some(app) = self.filtered_apps_data.get(target_index as usize) {
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
                box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
            }
            .app-title {
                font-size: 24pt;
                font-weight: bold;
            }
            .dim-label {
                color: @insensitive_fg_color;
                font-size: 11pt;
            }
            .dim-label-monospace {
                color: @insensitive_fg_color;
                font-family: monospace;
                font-size: 9pt;
            }
        "#,
    );

    app.run::<App>(());
}
