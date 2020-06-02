use std::{
    error::Error,
    rc::Rc,
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
    MenuBarBuilder,
    TreeViewBuilder,
    TreeStore,
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

/// A wrapper type for Rc<RefCell<T>>
struct GRc<T> {
    inner_rc: Rc<RefCell<T>>
}

impl<T> GRc<T> {
    /// Creates a new Rc<RefCell<T>> wrapper
    pub fn new(item: T) -> Self {
        Self {
            inner_rc: Rc::new(RefCell::new(item))
        }
    }

    /// Get an immutable reference to the content
    pub fn get<'r>(&'r self) -> Ref<'r, T> {
        self.inner_rc.as_ref().borrow()
    }
    
    /// Get a mutable reference to the content
    pub fn get_mut<'r>(&'r self) -> RefMut<'r, T> {
        self.inner_rc.as_ref().borrow_mut()
    }
}

/// The app.glade file
pub static WINDOW_GLADE: &'static str = include_str!("app.glade");

/// Create application
pub fn create_app(title: &'static str, size: (i32, i32)) -> Result<Application, Box<dyn Error>> {
    let app = Application::new(
        Some("space.wrckn.varpack-gtk"),
        ApplicationFlags::FLAGS_NONE
    )?;

    let builder = Builder::new_from_string(WINDOW_GLADE);
    let window: Window = builder.get_object("window")
        .ok_or(static_err("Corrupt app.glade file!"))?;


    app.connect_activate(move |app| {
        app.add_window(&window);
        window.show_all();
        
    });

    Ok(app)
}

/// Loads the GTK window from a .glade file

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
    let app = create_app("varpack-gtk", (800, 600))?;
    app.run(&std::env::args().collect::<Vec<_>>());
    Ok(())
}