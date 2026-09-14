use dioxus::prelude::*;

#[component]
pub fn SidebarToggle() -> Element {
    let mut sidebar_open = use_context::<Signal<bool>>();

    rsx! {
        button {
            class: "bg-transparent border-none cursor-pointer text-foreground p-1 flex items-center -ml-1",
            onclick: move |_| sidebar_open.set(true),
            hamburger_icon {}
        }
    }
}

fn hamburger_icon() -> Element {
    rsx! {
        svg {
            fill: "none",
            height: "18",
            view_box: "0 0 18 18",
            width: "18",
            path {
                d: "M2 4h14M2 9h14M2 14h14",
                stroke: "currentColor",
                stroke_linecap: "round",
                stroke_width: "1.5",
            }
        }
    }
}
