use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn AuthPage() -> Element {
    rsx! {
        div { class: "flex items-center justify-center bg-background min-h-screen relative",
            div { class: "absolute inset-0 opacity-5",
                style: "background-image: linear-gradient(var(--primary) 1px, transparent 1px), linear-gradient(90deg, var(--primary) 1px, transparent 1px); background-size: 40px 40px;",
            }
            div { class: "relative w-full max-w-sm",
                div { class: "mb-8 text-center",
                    div { class: "inline-flex items-center gap-2 mb-3",
                        div { class: "w-8 h-8 flex items-center justify-center bg-primary rounded",
                            icons::icon_0 {}
                        }
                        span { class: "text-xl font-bold tracking-tight text-foreground", style: "letter-spacing: -0.02em;", "PPDRIVE" }
                    }
                    p { class: "font-mono text-xs text-dim", "self-hosted object storage" }
                }
                div { class: "p-6 bg-card border border-border rounded",
                    div { class: "mb-5",
                        h1 { class: "text-sm font-semibold text-foreground", "Sign in to your account" }
                        p { class: "text-xs mt-0.5 text-muted-foreground", "Administrator access only" }
                    }
                    form { class: "flex flex-col gap-4",
                        div {
                            label { class: "block text-xs font-medium mb-1.5 text-muted-foreground", "EMAIL ADDRESS" }
                            input {
                                autocomplete: "email",
                                placeholder: "admin@ppdrive.local",
                                r#type: "email",
                                class: "bg-background border border-border rounded text-foreground text-[13px] w-full py-2 px-2.5 outline-none transition-colors font-mono",
                                value: "",
                            }
                        }
                        div {
                            label { class: "block text-xs font-medium mb-1.5 text-muted-foreground", "PASSWORD" }
                            input {
                                autocomplete: "current-password",
                                placeholder: "••••••••",
                                r#type: "password",
                                class: "bg-background border border-border rounded text-foreground text-[13px] w-full py-2 px-2.5 outline-none transition-colors font-mono",
                                value: "",
                            }
                        }
                        Link {
                            to: Route::Dashboard {},
                            class: "bg-primary text-background rounded py-[9px] px-4 text-[13px] font-semibold cursor-pointer w-full flex items-center justify-center gap-2 transition-colors mt-1 no-underline text-center",
                            "Sign In"
                        }
                    }
                }
                p { class: "text-center font-mono text-xs mt-4 text-dim", "v2.4.1 · ppdrive/server · linux/amd64" }
            }
        }
    }
}

mod icons {
    use super::*;

    pub(crate) fn icon_0() -> Element {
        rsx! {
            svg {
                fill: "none",
                height: "16",
                view_box: "0 0 16 16",
                width: "16",
                rect {
                    fill: "none",
                    height: "4",
                    rx: "1",
                    stroke: "#16181D",
                    stroke_width: "1.5",
                    width: "14",
                    x: "1",
                    y: "3",
                }
                rect {
                    fill: "none",
                    height: "4",
                    rx: "1",
                    stroke: "#16181D",
                    stroke_width: "1.5",
                    width: "14",
                    x: "1",
                    y: "9",
                }
                circle {
                    cx: "12.5",
                    cy: "5",
                    fill: "#16181D",
                    r: "1",
                }
                circle {
                    cx: "12.5",
                    cy: "11",
                    fill: "#16181D",
                    r: "1",
                }
            }
        }
    }
}
