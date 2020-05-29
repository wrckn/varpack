use std::{
    error::Error
};

use gtk::{
    prelude::*,
    ApplicationWindowBuilder,
    Application,
    BoxBuilder,
    GridBuilder,
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

/// Create application
pub fn create_app(title: &'static str, size: (i32, i32)) -> Result<Application, Box<dyn Error>> {
    let app = Application::new(
        Some("space.wrckn.varpack-gtk"),
        ApplicationFlags::FLAGS_NONE
    )?;

    app.connect_activate(move |app| build_window(app, title, size));

    Ok(app)
}

/// Builds the GTK window
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