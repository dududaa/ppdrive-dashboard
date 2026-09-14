use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn DashboardLayout() -> Element {
    let route = use_route::<Route>();
    let is_dashboard = matches!(route, Route::Dashboard {});
    let is_explorer = matches!(route, Route::Explorer {});
    let is_clients = matches!(route, Route::Clients {});
    let is_settings = matches!(route, Route::Settings {});
    let mut sidebar_open = use_signal(|| false);
    use_context_provider(|| sidebar_open);

    rsx! {
        div { class: "flex h-full bg-background overflow-hidden relative",
            if sidebar_open() {
                div {
                    class: "fixed inset-0 bg-black/50 z-40 lg:hidden",
                    onclick: move |_| sidebar_open.set(false),
                }
            }
            aside {
                class: if sidebar_open() {
                    "w-[220px] bg-card border-r border-border flex flex-col shrink-0 fixed inset-y-0 left-0 z-50 transition-transform lg:static lg:translate-x-0"
                } else {
                    "w-[220px] bg-card border-r border-border flex flex-col shrink-0 fixed inset-y-0 left-0 z-50 transition-transform -translate-x-full lg:static lg:translate-x-0"
                },
                div { class: "px-4 h-12 border-b border-border flex items-center gap-2.5",
                    div { class: "w-[26px] h-[26px] bg-primary rounded flex items-center justify-center shrink-0",
                        icons::icon_0 {}
                    }
                    div {
                        div { class: "text-sm font-bold text-foreground tracking-tight leading-tight", "PPDRIVE" }
                        div { class: "font-mono text-[10px] text-dim leading-tight", "v2.4.1" }
                    }
                }
                div { class: "py-2.5 px-4 border-b border-border",
                    div { class: "flex items-center gap-2",
                        div { class: "w-1.5 h-1.5 rounded-full bg-success shrink-0" }
                        span { class: "font-mono text-xs text-dim", "ppdrive-prod-01" }
                    }
                    div { class: "font-mono mt-1 text-[10px] text-dim", "Ubuntu 22.04 · linux/amd64" }
                }
                nav { class: "flex flex-col py-3 gap-0.5 px-2 flex-1",
                    div { class: "px-2 pb-2 text-xs font-medium uppercase text-dim tracking-widest text-[10px]", "Navigation" }
                    Link {
                        to: Route::Dashboard {},
                        class: if is_dashboard { "flex items-center gap-[9px] py-[7px] px-2.5 rounded bg-primary/12 text-primary text-[13px] font-medium cursor-pointer w-full text-left transition-all no-underline border-none" } else { "flex items-center gap-[9px] py-[7px] px-2.5 rounded bg-transparent text-muted-foreground text-[13px] cursor-pointer w-full text-left transition-all no-underline border-none" },
                        onclick: move |_| sidebar_open.set(false),
                        span { class: if is_dashboard { "opacity-100" } else { "opacity-70" }, icons::icon_1 {} }
                        "Dashboard"
                    }
                    Link {
                        to: Route::Explorer {},
                        class: if is_explorer { "flex items-center gap-[9px] py-[7px] px-2.5 rounded bg-primary/12 text-primary text-[13px] font-medium cursor-pointer w-full text-left transition-all no-underline border-none" } else { "flex items-center gap-[9px] py-[7px] px-2.5 rounded bg-transparent text-muted-foreground text-[13px] cursor-pointer w-full text-left transition-all no-underline border-none" },
                        onclick: move |_| sidebar_open.set(false),
                        span { class: if is_explorer { "opacity-100" } else { "opacity-70" }, icons::icon_2 {} }
                        "File Explorer"
                    }
                    Link {
                        to: Route::Clients {},
                        class: if is_clients { "flex items-center gap-[9px] py-[7px] px-2.5 rounded bg-primary/12 text-primary text-[13px] font-medium cursor-pointer w-full text-left transition-all no-underline border-none" } else { "flex items-center gap-[9px] py-[7px] px-2.5 rounded bg-transparent text-muted-foreground text-[13px] cursor-pointer w-full text-left transition-all no-underline border-none" },
                        onclick: move |_| sidebar_open.set(false),
                        span { class: if is_clients { "opacity-100" } else { "opacity-70" }, icons::icon_3 {} }
                        "App Clients"
                    }
                    Link {
                        to: Route::Settings {},
                        class: if is_settings { "flex items-center gap-[9px] py-[7px] px-2.5 rounded bg-primary/12 text-primary text-[13px] font-medium cursor-pointer w-full text-left transition-all no-underline border-none" } else { "flex items-center gap-[9px] py-[7px] px-2.5 rounded bg-transparent text-muted-foreground text-[13px] cursor-pointer w-full text-left transition-all no-underline border-none" },
                        onclick: move |_| sidebar_open.set(false),
                        span { class: if is_settings { "opacity-100" } else { "opacity-70" }, icons::icon_4 {} }
                        "Settings"
                    }
                }
                div { class: "border-t border-border py-2.5 px-3.5",
                    div { class: "flex items-center justify-between",
                        div {
                            div { class: "font-mono text-xs text-muted-foreground", "admin@company.com" }
                            div { class: "font-mono text-[10px] text-dim", "Administrator" }
                        }
                        button {
                            class: "bg-transparent border-none cursor-pointer text-dim p-1 flex items-center",
                            title: "Sign out",
                            icons::icon_5 {}
                        }
                    }
                }
            }
            Outlet::<Route> {}
        }
    }
}

mod icons {
    use super::*;

    pub(crate) fn icon_0() -> Element {
        rsx! {
            svg {
                fill: "none",
                height: "13",
                view_box: "0 0 13 13",
                width: "13",
                rect {
                    height: "3.5",
                    rx: "0.8",
                    stroke: "#16181D",
                    stroke_width: "1.1",
                    width: "12",
                    x: "0.5",
                    y: "1.5",
                }
                rect {
                    height: "3.5",
                    rx: "0.8",
                    stroke: "#16181D",
                    stroke_width: "1.1",
                    width: "12",
                    x: "0.5",
                    y: "8",
                }
                circle {
                    cx: "10.5",
                    cy: "3.25",
                    fill: "#16181D",
                    r: "0.8",
                }
                circle {
                    cx: "10.5",
                    cy: "9.75",
                    fill: "#16181D",
                    r: "0.8",
                }
            }
        }
    }

    pub(crate) fn icon_1() -> Element {
        rsx! {
            svg {
                fill: "none",
                height: "15",
                view_box: "0 0 15 15",
                width: "15",
                rect {
                    height: "5.5",
                    rx: "1",
                    stroke: "currentColor",
                    stroke_width: "1.3",
                    width: "5.5",
                    x: "1",
                    y: "1",
                }
                rect {
                    height: "5.5",
                    rx: "1",
                    stroke: "currentColor",
                    stroke_width: "1.3",
                    width: "5.5",
                    x: "8.5",
                    y: "1",
                }
                rect {
                    height: "5.5",
                    rx: "1",
                    stroke: "currentColor",
                    stroke_width: "1.3",
                    width: "5.5",
                    x: "1",
                    y: "8.5",
                }
                rect {
                    height: "5.5",
                    rx: "1",
                    stroke: "currentColor",
                    stroke_width: "1.3",
                    width: "5.5",
                    x: "8.5",
                    y: "8.5",
                }
            }
        }
    }

    pub(crate) fn icon_2() -> Element {
        rsx! {
            svg {
                fill: "none",
                height: "15",
                view_box: "0 0 15 15",
                width: "15",
                path {
                    d: "M1 3a1 1 0 011-1h4l1.5 1.5H13a1 1 0 011 1v7a1 1 0 01-1 1H2a1 1 0 01-1-1V3z",
                    stroke: "currentColor",
                    stroke_width: "1.3",
                }
            }
        }
    }

    pub(crate) fn icon_3() -> Element {
        rsx! {
            svg {
                fill: "none",
                height: "15",
                view_box: "0 0 15 15",
                width: "15",
                rect {
                    height: "9",
                    rx: "1",
                    stroke: "currentColor",
                    stroke_width: "1.3",
                    width: "13",
                    x: "1",
                    y: "3",
                }
                path { d: "M5 9l2-2 2 2", stroke: "currentColor", stroke_width: "1.3" }
                path { d: "M7 7V11", stroke: "currentColor", stroke_width: "1.3" }
                path { d: "M1 6h13", stroke: "currentColor", stroke_width: "1.3" }
            }
        }
    }

    pub(crate) fn icon_4() -> Element {
        rsx! {
            svg {
                fill: "none",
                height: "15",
                view_box: "0 0 15 15",
                width: "15",
                circle {
                    cx: "7.5",
                    cy: "7.5",
                    r: "2",
                    stroke: "currentColor",
                    stroke_width: "1.3",
                }
                path {
                    d: "M7.5 1v1.5M7.5 12.5V14M14 7.5h-1.5M2.5 7.5H1M12.07 2.93l-1.06 1.06M4 11l-1.07 1.07M12.07 12.07l-1.06-1.06M4 4L2.93 2.93",
                    stroke: "currentColor",
                    stroke_linecap: "round",
                    stroke_width: "1.3",
                }
            }
        }
    }

    pub(crate) fn icon_5() -> Element {
        rsx! {
            svg {
                fill: "none",
                height: "14",
                view_box: "0 0 14 14",
                width: "14",
                path {
                    d: "M9 3l4 4-4 4M13 7H5",
                    stroke: "currentColor",
                    stroke_linecap: "round",
                    stroke_width: "1.3",
                }
                path {
                    d: "M5 1H2a1 1 0 00-1 1v10a1 1 0 001 1h3",
                    stroke: "currentColor",
                    stroke_linecap: "round",
                    stroke_width: "1.3",
                }
            }
        }
    }
}
