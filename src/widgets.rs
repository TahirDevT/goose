use gtk::prelude::*;
use crate::lua_config::{WidgetStyle, WidgetSize};

pub struct TextWidget {
    pub content: String,
    pub style: WidgetStyle,
}

pub struct ButtonWidget {
    pub content: String,
    pub style: WidgetStyle,
    pub size: WidgetSize,
}

impl TextWidget {
    pub fn new(content: String, style: WidgetStyle) -> Self {
        TextWidget { content, style }
    }

    pub fn create_widget(&self) -> gtk::Widget {
        let label = gtk::Label::new(Some(&self.content));
        label.set_markup(&format!("<span>{}</span>", self.content));
        label.style_context().add_class(&self.style.style_class);
        label.upcast()
    }
}

impl ButtonWidget {
    pub fn new(content: String, style: WidgetStyle, size: WidgetSize) -> Self {
        ButtonWidget { content, style, size }
    }

    pub fn create_widget(&self) -> gtk::Widget {
        let button = gtk::Button::with_label(&self.content);
        button.set_size_request(self.size.width, self.size.height);
        button.style_context().add_class(&self.style.style_class);
        button.upcast()
    }
}
