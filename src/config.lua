return {
    layout = {
        type = "absolute",
        width = 1366,
        height = 768,
    },
    widgets = {
        {
            type = "text",
            content = "Hello, Widget Manager!",
            x = 10,
            y = 10,
            style_class = "greeting",
        },
        {
            type = "text",
            content = "This is positioned text.",
            x = 50,
            y = 50,
            style_class = "positioned-text",
        },
        {
            type = "button",
            content = "Click me!",
            x = 500,
            y = 500,
            width = 100,
            height = 100,
            style_class = "my-button",
        },
        {
            type = "group",
            x = 100,
            y = 100,
            style_class = "group-container",
            layout = {
                type = "vertical",
                spacing = 5,
            },
            widgets = {
                {
                    type = "text",
                    content = "Group Item 1",
                    style_class = "group-item",
                },
                {
                    type = "text",
                    content = "Group Item 2",
                    style_class = "group-item",
                },
            },
        },
    },
}
