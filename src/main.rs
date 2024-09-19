use gtk::prelude::*;
use rlua::Lua;
use gtk_layer_shell::{Layer, Edge};
use gtk::glib::Bytes;

mod widget_manager;
mod lua_config;
mod widgets;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");
    
    let application = gtk::Application::new(Some("com.example.widget-manager"), Default::default());
    
    application.connect_activate(|app| {
        let lua = Lua::new();
        let config = lua_config::load_config(&lua, "config.lua").expect("Failed to load config");
        
        let widget_manager = widget_manager::WidgetManager::new(config);
        
        let window = gtk::ApplicationWindow::new(app);
        
        // Set up the window as a layer shell surface
        gtk_layer_shell::init_for_window(&window);
        gtk_layer_shell::set_layer(&window, Layer::Overlay);
        gtk_layer_shell::set_anchor(&window, Edge::Left, true);
        gtk_layer_shell::set_anchor(&window, Edge::Top, true);
        
        window.set_app_paintable(true);
        window.set_decorated(false);
        
        if let lua_config::Layout::Absolute { width, height } = &widget_manager.config.layout {
            window.set_size_request(*width, *height);
        }
        
        // Load and apply CSS
        let css_provider = gtk::CssProvider::new();
        let css_data = include_bytes!("./styles.css");
        css_provider.load_from_data(&Bytes::from_static(css_data)).expect("Failed to load CSS");
        gtk::StyleContext::add_provider_for_screen(
            &gtk::gdk::Screen::default().expect("Error initializing gtk css provider."),
            &css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        
        widget_manager.add_widgets_to_window(&window);
        
        window.show_all();
    });
    
    application.run();
}
