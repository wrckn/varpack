use super::ptr::Ptr;

use std::{
    error::Error,
    rc::Rc,
    fs::File,
    cell::{
        Ref,
        RefMut,
        RefCell
    },
    convert::{
        AsMut,
        AsRef
    },
    ops::{
        Deref,
        DerefMut
    }
};

use gtk::{
    prelude::*,
    ApplicationWindowBuilder,
    Application,
    Window,
    BoxBuilder,
    GridBuilder,
    Builder,
    Align,
    Dialog,
    AboutDialog,
    MenuBarBuilder,
    TreeViewBuilder,
    MenuItem,
    TreeStore,
    ResponseType,
    PackDirection,
    Orientation,
    LabelBuilder,
    MenuItemBuilder
};
use gio::{
    prelude::*,
    ApplicationFlags
};
use string_error::{
    static_err
};
use var::{
    VarWriter
};
/// The main.glade file
pub static MAIN_GLADE: &'static str = include_str!("glade/main.glade");

/// The application state struct
pub struct AppState {
    pub window_main: Window,
    pub dialog_about: AboutDialog,
    pub dialog_loading: Dialog,
    pub tree_store: Option<TreeStore>,
    pub archive: Option<VarWriter<File>>
}

impl AppState {
    pub fn new(window_main: Window, dialog_about: AboutDialog, dialog_loading: Dialog) -> Self {
        Self {
            window_main: window_main,
            dialog_about: dialog_about,
            dialog_loading: dialog_loading,
            tree_store: None,
            archive: None
        }
    }
}

/// Create application
pub fn create_app() -> Result<Application, Box<dyn Error>> {
    let app = Application::new(
        Some("space.wrckn.varpack-gtk"),
        ApplicationFlags::FLAGS_NONE
    )?;

    let builder = Builder::new_from_string(MAIN_GLADE);
    let main_window: Window = builder.get_object("window_main")
        .ok_or(static_err("Corrupt main.glade file!"))?;
    let dialog_about: AboutDialog = builder.get_object("dialog_about")
        .ok_or(static_err("Corrupt main.glade file!"))?;
    let dialog_loading: Dialog = builder.get_object("dialog_loading")
        .ok_or(static_err("Corrupt main.glade file!"))?;
    let menu_help_about: MenuItem = builder.get_object("menu_help_about")
        .ok_or(static_err("Corrupt main.glade file!"))?;

    let app_state = Ptr::new(AppState::new(main_window, dialog_about, dialog_loading));
    
    app.connect_activate(move |app| {
        app.add_window(&app_state.get().window_main);
        app_state.get().window_main.show_all();

        {
            let app_state = app_state.clone_ptr();
            menu_help_about.connect_activate(move |_item| {
                let res = app_state.get().dialog_about.run();
                match res {
                    ResponseType::DeleteEvent => app_state.get().dialog_about.hide(),
                    _ => {}
                };
            });
        }
    });

    Ok(app)
}


/// Builds the GTK window
#[deprecated = "Use the app.glade file"]
pub fn build_window(application: &Application, title: &str, size: (i32, i32)) {
    let menu_bar = MenuBarBuilder::default()
        .valign(Align::Start)
        .halign(Align::Fill)
        .pack_direction(PackDirection::Ltr)
        .hexpand(true)
        .build();
    menu_bar.append(&MenuItemBuilder::default().halign(Align::Start).label("File").build());
    menu_bar.append(&MenuItemBuilder::default().halign(Align::Start).label("Help").build());
    
    let tree_label = LabelBuilder::default()
        .label("Archive file tree")
        .build();
    let tree_box = BoxBuilder::default()
        .valign(Align::Center)
        .halign(Align::Center)
        .child(&tree_label)
        .orientation(Orientation::Vertical)
        .width_request(75)
        .build();

    let info_label = LabelBuilder::default()
        .label("Archife file info")
        .build();
    let info_box = BoxBuilder::default()
        .valign(Align::Center)
        .halign(Align::Center)
        .child(&info_label)
        .orientation(Orientation::Vertical)
        .width_request(75)
        .build();
    let main_grid = GridBuilder::default()
        .column_spacing(10)
        .orientation(Orientation::Horizontal)
        .build();
    main_grid.add(&tree_box);
    main_grid.add(&info_box);
    
    let vbox = BoxBuilder::default()
        .orientation(Orientation::Vertical)
        .vexpand(true)
        .hexpand(true)
        .child(&menu_bar)
        .build();
    
    let win = ApplicationWindowBuilder::default()
        .application(application)
        .title(title)
        .default_width(size.0)
        .default_height(size.1)
        .child(&vbox)
        .build();

    win.show_all();
}


/// Starts the GTK app
pub fn run_app() -> Result<(), Box<dyn Error>> {
    let app = create_app()?;
    app.run(&std::env::args().collect::<Vec<_>>());
    Ok(())
}