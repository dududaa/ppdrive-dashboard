use crate::Route;
use dioxus::prelude::*;
use gloo_net::http::Request;
use ppdrive_dashboard_shared::LoginRequest;

const API_BASE: &str = "http://localhost:8081/dashboard";

#[component]
pub fn AuthPage() -> Element {
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut loading = use_signal(|| false);
    let mut error = use_signal(|| Option::<String>::None);
    let navigator = use_navigator();

    let handle_submit = move |event: Event<FormData>| {
        event.prevent_default();

        if loading() {
            return;
        }

        error.set(None);

        let email_val = email();
        let password_val = password();

        if email_val.is_empty() || !email_val.contains('@') {
            error.set(Some("Please enter a valid email address".into()));
            return;
        }
        if password_val.is_empty() {
            error.set(Some("Please enter your password".into()));
            return;
        }

        loading.set(true);

        let req = LoginRequest {
            email: email_val,
            password: password_val,
        };
        let body = match serde_json::to_string(&req) {
            Ok(b) => b,
            Err(_) => {
                error.set(Some("Failed to serialize request".into()));
                loading.set(false);
                return;
            }
        };

        let nav = navigator.clone();
        spawn(async move {
            let request = match Request::post(&format!("{API_BASE}/login"))
                .header("Content-Type", "application/json")
                .body(body)
            {
                Ok(r) => r,
                Err(_) => {
                    error.set(Some("Failed to build request".into()));
                    loading.set(false);
                    return;
                }
            };

            let resp = match request.send().await {
                Ok(r) => r,
                Err(_) => {
                    error.set(Some("Connection failed. Is the server running?".into()));
                    loading.set(false);
                    return;
                }
            };

            let status = resp.status();
            if status == 200 {
                nav.push(Route::Dashboard {});
            } else if status == 401 {
                error.set(Some("Invalid email or password".into()));
            } else {
                let msg = resp.text().await.unwrap_or_else(|_| "Something went wrong".into());
                error.set(Some(msg));
            }
            loading.set(false);
        });
    };

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
                    form { class: "flex flex-col gap-4", onsubmit: handle_submit,
                        div {
                            label { class: "block text-xs font-medium mb-1.5 text-muted-foreground", "EMAIL ADDRESS" }
                            input {
                                autocomplete: "email",
                                placeholder: "admin@ppdrive.local",
                                r#type: "email",
                                class: "bg-background border border-border rounded text-foreground text-[13px] w-full py-2 px-2.5 outline-none transition-colors font-mono",
                                value: "{email}",
                                oninput: move |e| email.set(e.value()),
                            }
                        }
                        div {
                            label { class: "block text-xs font-medium mb-1.5 text-muted-foreground", "PASSWORD" }
                            input {
                                autocomplete: "current-password",
                                placeholder: "\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}",
                                r#type: "password",
                                class: "bg-background border border-border rounded text-foreground text-[13px] w-full py-2 px-2.5 outline-none transition-colors font-mono",
                                value: "{password}",
                                oninput: move |e| password.set(e.value()),
                            }
                        }
                        if let Some(msg) = error() {
                            div { class: "bg-error/10 border border-error/20 text-error rounded p-2.5 text-xs font-mono",
                                "{msg}"
                            }
                        }
                        button {
                            r#type: "submit",
                            disabled: loading(),
                            class: "bg-primary text-background rounded py-[9px] px-4 text-[13px] font-semibold cursor-pointer w-full flex items-center justify-center gap-2 transition-colors mt-1 disabled:opacity-50 disabled:cursor-not-allowed",
                            if loading() {
                                svg {
                                    class: "animate-spin",
                                    width: "14",
                                    height: "14",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    circle {
                                        cx: "12",
                                        cy: "12",
                                        r: "10",
                                        stroke: "currentColor",
                                        stroke_width: "3",
                                        opacity: "0.3",
                                    }
                                    path {
                                        d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z",
                                        fill: "currentColor",
                                    }
                                }
                                "Signing in..."
                            } else {
                                "Sign In"
                            }
                        }
                    }
                }
                p { class: "text-center font-mono text-xs mt-4 text-dim", "v2.4.1 \u{00b7} ppdrive/server \u{00b7} linux/amd64" }
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
