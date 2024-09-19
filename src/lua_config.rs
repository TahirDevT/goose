use rlua::{Lua, Result, Value};
use std::fs;

#[derive(Debug)]
pub enum Layout {
    Absolute { width: i32, height: i32 },
    Vertical { spacing: i32 },
    Horizontal { spacing: i32 },
}

#[derive(Debug)]
pub struct WidgetPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
pub struct WidgetSize {
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone)]
pub struct WidgetStyle {
    pub style_class: String,
}

#[derive(Debug)]
pub enum WidgetConfig {
    Text { content: String, position: WidgetPosition, style: WidgetStyle },
    Group { layout: Layout, widgets: Vec<WidgetConfig>, position: WidgetPosition, style: WidgetStyle },
    Button { content: String, position: WidgetPosition, size: WidgetSize, style: WidgetStyle },
}

#[derive(Debug)]
pub struct Config {
    pub layout: Layout,
    pub widgets: Vec<WidgetConfig>,
}

fn parse_layout(table: &rlua::Table) -> Result<Layout> {
    let layout_type: String = table.get("type")?;
    match layout_type.as_str() {
        "absolute" => {
            let width = table.get("width")?;
            let height = table.get("height")?;
            Ok(Layout::Absolute { width, height })
        },
        "vertical" => {
            let spacing = table.get("spacing")?;
            Ok(Layout::Vertical { spacing })
        },
        "horizontal" => {
            let spacing = table.get("spacing")?;
            Ok(Layout::Horizontal { spacing })
        },
        _ => Err(rlua::Error::RuntimeError(format!("Unknown layout type: {}", layout_type))),
    }
}

fn parse_widget(value: Value) -> Result<WidgetConfig> {
    if let Value::Table(widget_table) = value {
        let widget_type: String = widget_table.get("type")?;
        let x: i32 = widget_table.get("x").unwrap_or(0);
        let y: i32 = widget_table.get("y").unwrap_or(0);
        let position = WidgetPosition { x, y };
        let style_class: String = widget_table.get("style_class").unwrap_or_default();
        let style = WidgetStyle { style_class };

        match widget_type.as_str() {
            "text" => {
                let content: String = widget_table.get("content")?;
                Ok(WidgetConfig::Text { content, position, style })
            },
            "group" => {
                let layout = parse_layout(&widget_table.get::<_, rlua::Table>("layout")?)?;
                let widgets_table: rlua::Table = widget_table.get("widgets")?;
                let mut widgets = Vec::new();
                for pair in widgets_table.pairs::<Value, Value>() {
                    let (_, widget_value) = pair?;
                    widgets.push(parse_widget(widget_value)?);
                }
                Ok(WidgetConfig::Group { layout, widgets, position, style })
            },
            "button" => {
                let content: String = widget_table.get("content")?;
                let width: i32 = widget_table.get("width")?;
                let height: i32 = widget_table.get("height")?;
                let size = WidgetSize { width, height };
                Ok(WidgetConfig::Button { content, position, size, style })
            },
            _ => Err(rlua::Error::RuntimeError(format!("Unknown widget type: {}", widget_type))),
        }
    } else {
        Err(rlua::Error::RuntimeError("Widget config is not a table".to_string()))
    }
}

pub fn load_config(lua: &Lua, config_path: &str) -> Result<Config> {
    let config_content = fs::read_to_string(config_path).expect("Failed to read config file");
    
    lua.context(|ctx| {
        let config: Value = ctx.load(&config_content).eval()?;
        if let Value::Table(config_table) = config {
            let layout = parse_layout(&config_table.get::<_, rlua::Table>("layout")?)?;
            let widgets_table: rlua::Table = config_table.get("widgets")?;
            
            let mut widgets = Vec::new();
            for pair in widgets_table.pairs::<Value, Value>() {
                let (_, widget_value) = pair?;
                widgets.push(parse_widget(widget_value)?);
            }
            
            Ok(Config { layout, widgets })
        } else {
            Err(rlua::Error::RuntimeError("Config is not a table".to_string()))
        }
    })
}
