use gtk4::prelude::*;
use gtk4::{gdk, glib};
use relm4::prelude::*;

struct App {
    selected_index: i32,
}

#[derive(Debug)]
enum Msg {
    MoveSelectionUp,
    MoveSelectionDown,
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
                        #[name = "list_box"]
                        gtk4::ListBox {}
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
        let model = App { selected_index: 0 };

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
