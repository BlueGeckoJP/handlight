use gtk4::prelude::*;
use relm4::prelude::*;

struct App {}

#[derive(Debug)]
enum Msg {}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = Msg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("handlight"),
            set_default_size: (500, 300),
        }
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = App {};

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, _msg: Self::Input, _sender: ComponentSender<Self>) {}
}

#[tokio::main]
async fn main() {
    let app = RelmApp::new("me.bluegecko.handlight");
    app.run::<App>(());
}
