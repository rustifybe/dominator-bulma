pub mod components;
pub mod form;

#[macro_export]
macro_rules! columns {
    ($($classes:literal,)* { $($methods:tt)* }) => {
        dominator::html!("div" => web_sys::HtmlDivElement, {
            .class("columns")
            $(.class($classes))*
            $($methods)*
        })
    };
}

#[macro_export]
macro_rules! column {
    ($($classes:literal,)* { $($methods:tt)* }) => {
        dominator::html!("div" => web_sys::HtmlDivElement, {
            .class("column")
            $(.class($classes))*
            $($methods)*
        })
    };
}

#[macro_export]
macro_rules! tile {
    ($($classes:literal,)* { $($methods:tt)* }) => {
        dominator::html!("div" => web_sys::HtmlDivElement, {
            .class("tile")
            $(.class($classes))*
            $($methods)*
        })
    };
}

#[macro_export]
macro_rules! section {
    ($($classes:literal,)* { $($methods:tt)* }) => {
        dominator::html!("section" => web_sys::HtmlElement, {
            .class("section")
            $(.class($classes))*
            $($methods)*
        })
    };
}

#[macro_export]
macro_rules! container {
    ($($classes:literal,)* { $($methods:tt)* }) => {
        dominator::html!("div" => web_sys::HtmlDivElement, {
            .class("container")
            $(.class($classes))*
            $($methods)*
        })
    };
}

#[macro_export]
macro_rules! level {
    ($($classes:literal,)* { $($methods:tt)* }) => {
        dominator::html!("nav" => web_sys::HtmlElement, {
            .class("level")
            $(.class($classes))*
            $($methods)*
        })
    };
}

#[macro_export]
macro_rules! image {
    ($($classes:literal,)* { $($methods:tt)* }) => {
        dominator::html!("figure" => web_sys::HtmlElement, {
            .class("image")
            $(.class($classes))*
            $($methods)*
        })
    };
}

#[macro_export]
macro_rules! icon {
    ($($classes:literal,)* { $($methods:tt)* }) => {
        dominator::html!("span" => web_sys::HtmlSpanElement, {
            .class("icon")
            $(.class($classes))*
            $($methods)*
        })
    };
}

#[macro_export]
macro_rules! icon_text {
    ($($classes:literal,)* { $($methods:tt)* }) => {
        dominator::html!("span" => web_sys::HtmlSpanElement, {
            .class("icon-text")
            $(.class($classes))*
            $($methods)*
        })
    };
}


#[macro_export]
macro_rules! tag {
    ($($classes:literal,)* { $($methods:tt)* }) => {
        dominator::html!("span" => web_sys::HtmlSpanElement, {
            .class("tag")
            $(.class($classes))*
            $($methods)*
        })
    };
}

#[macro_export]
macro_rules! button {
    ($($classes:literal,)* { $($methods:tt)* }) => {
        dominator::html!("button" => web_sys::HtmlButtonElement, {
            .class("button")
            $(.class($classes))*
            $($methods)*
        })
    };
}

#[macro_export]
macro_rules! content {
    ($($classes:literal,)* { $($methods:tt)* }) => {
        dominator::html!("div" => web_sys::HtmlDivElement, {
            .class("content")
            $(.class($classes))*
            $($methods)*
        })
    };
}

#[macro_export]
macro_rules! level_left {
    ($($classes:literal,)* { $($methods:tt)* }) => {
        dominator::html!("div" => web_sys::HtmlDivElement, {
            .class("level-left")
            $(.class($classes))*
            $($methods)*
        })
    };
}

#[macro_export]
macro_rules! level_right {
    ($($classes:literal,)* { $($methods:tt)* }) => {
        dominator::html!("div" => web_sys::HtmlDivElement, {
            .class("level-left")
            $(.class($classes))*
            $($methods)*
        })
    };
}

#[macro_export]
macro_rules! level_item {
    ($($classes:literal,)* { $($methods:tt)* }) => {
        dominator::html!("div" => web_sys::HtmlDivElement, {
            .class("level-item")
            $(.class($classes))*
            $($methods)*
        })
    };
}

#[macro_export]
macro_rules! block {
    ($($classes:literal,)* { $($methods:tt)* }) => {
        dominator::html!("div" => web_sys::HtmlDivElement, {
            .class("block")
            $(.class($classes))*
            $($methods)*
        })
    };
}
