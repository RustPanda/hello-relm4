use gtk::prelude::*;
use relm4::{AppUpdate, Model, RelmApp, Widgets};

// The application state will be stored here
struct AppModel {
    counter: u8
}

// Description of messages passed from view to model
enum AppMsg {
    Increment,
    Decrement
}

// Describe the widgets of the view
struct AppWidgets {
    window: gtk::ApplicationWindow,
    _increment: gtk::Button,
    _decrement: gtk::Button,
    counter_label: gtk::Label,
    _vbox: gtk::Box
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = ();
}

impl AppUpdate for AppModel {
    fn update(
        &mut self,
        msg: Self::Msg,
        _components: &Self::Components,
        _sender: relm4::Sender<Self::Msg>,
    ) -> bool {
        match msg {
            AppMsg::Increment => self.counter = self.counter.wrapping_add(1),
            AppMsg::Decrement => self.counter = self.counter.wrapping_sub(1),
        };

        true
    }
}

impl Widgets<AppModel, ()> for AppWidgets {
    type Root = gtk::ApplicationWindow;

    fn init_view(
        model: &AppModel,
        _parent_widgets: &(),
        sender: relm4::Sender<AppMsg>,
    ) -> Self {

        //Create a window and set the title
        let window = gtk::ApplicationWindow::builder()
            .title("My GTK App")
            .build();

        // Create counter label
        let label = gtk::LabelBuilder::new()
            .label(&format!("{}", model.counter))
            .margin_top(5)
            .margin_bottom(5)
            .margin_start(5)
            .margin_end(5)
            .build();

        // Create increment buttin
        let increment = gtk::ButtonBuilder::new()
            .label("Increment")
            .margin_top(5)
            .margin_bottom(5)
            .margin_start(5)
            .margin_end(5)
            .build();
        
        // Create decrement button
        let decrement = gtk::ButtonBuilder::new()
            .label("Decrement")
            .margin_top(5)
            .margin_bottom(5)
            .margin_start(5)
            .margin_end(5)
            .build();

        // Create vertical box conainer
        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(5)
            .margin_top(5)
            .margin_bottom(5)
            .margin_start(5)
            .margin_end(5)
            .build();

        // Append all widgets in vbox
        vbox.append(&increment);
        vbox.append(&label);
        vbox.append(&decrement);

        // Set vbox to application window
        window.set_child(Some(&vbox));

        // Connect event
        let increment_sender = sender.clone();
        increment.connect_clicked(move |_| {increment_sender.send(AppMsg::Increment).unwrap();});
        decrement.connect_clicked(move |_| {sender.send(AppMsg::Decrement).unwrap();});


        AppWidgets {
            window,
            _increment: increment,
            _decrement: decrement,
            counter_label: label,
            _vbox: vbox,
        }
    }

    fn root_widget(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(&mut self, model: &AppModel, _sender: relm4::Sender<AppMsg>) {
        self.counter_label.set_label(&format!("{}", model.counter));
    }
}

fn main() {
    let model = AppModel {
        counter: 5,
    };
    let app = RelmApp::new(model);
    app.run();
}
