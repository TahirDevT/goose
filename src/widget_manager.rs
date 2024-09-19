use crate::lua_config::{Config, WidgetConfig, Layout};
use crate::widgets::{TextWidget, ButtonWidget};
use gtk::prelude::*;

pub struct WidgetManager {
    pub config: Config,
}

impl WidgetManager {
    pub fn new(config: Config) -> Self {
        WidgetManager { config }
    }

    pub fn add_widgets_to_window(&self, window: &gtk::ApplicationWindow) {
        let drawing_area = gtk::DrawingArea::new();
        drawing_area.connect_draw(|_, cr| {
            // Set the entire drawing area to be transparent
            cr.set_source_rgba(0.0, 0.0, 0.0, 0.0);
            cr.set_operator(cairo::Operator::Source);
            let _ = cr.paint();
            Inhibit(false)
        });

        let fixed = gtk::Fixed::new();
        self.add_widgets_to_fixed(&fixed, &self.config.widgets);
        
        let overlay = gtk::Overlay::new();
        overlay.add(&drawing_area);
        overlay.add_overlay(&fixed);

        window.add(&overlay);
    }

    fn add_widgets_to_fixed(&self, fixed: &gtk::Fixed, widgets: &[WidgetConfig]) {
        for widget in widgets {
            match widget {
                WidgetConfig::Text { content, position, style } => {
                    let text_widget = TextWidget::new(content.clone(), style.clone());
                    let widget = text_widget.create_widget();
                    fixed.put(&widget, position.x, position.y);
                },
                WidgetConfig::Group { layout, widgets, position, style } => {
                    let group_box = self.create_layout_box(layout);
                    group_box.style_context().add_class(&style.style_class);
                    self.add_widgets_to_container(&group_box, widgets);
                    fixed.put(&group_box, position.x, position.y);
                },
                WidgetConfig::Button { content, position, size, style } => {
                    let button_widget = ButtonWidget::new(content.clone(), style.clone(), size.clone());
                    let widget = button_widget.create_widget();
                    fixed.put(&widget, position.x, position.y);
                },
            }
        }
    }

    fn create_layout_box(&self, layout: &Layout) -> gtk::Box {
        match layout {
            Layout::Vertical { spacing } => {
                let box_container = gtk::Box::new(gtk::Orientation::Vertical, 0);
                box_container.set_spacing(*spacing);
                box_container
            },
            Layout::Horizontal { spacing } => {
                let box_container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
                box_container.set_spacing(*spacing);
                box_container
            },
            Layout::Absolute { .. } => gtk::Box::new(gtk::Orientation::Vertical, 0),
        }
    }

    fn add_widgets_to_container(&self, container: &gtk::Box, widgets: &[WidgetConfig]) {
        for widget in widgets {
            match widget {
                WidgetConfig::Text { content, style, .. } => {
                    let text_widget = TextWidget::new(content.clone(), style.clone());
                    container.add(&text_widget.create_widget());
                },
                WidgetConfig::Group { layout, widgets, style, .. } => {
                    let group_box = self.create_layout_box(layout);
                    group_box.style_context().add_class(&style.style_class);
                    self.add_widgets_to_container(&group_box, widgets);
                    container.add(&group_box);
                },
                WidgetConfig::Button { content, size, style, .. } => {
                    let button_widget = ButtonWidget::new(content.clone(), style.clone(), size.clone());
                    container.add(&button_widget.create_widget());
                },
            }
        }
    }
}
