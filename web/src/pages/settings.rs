use dioxus::prelude::*;

#[component]
pub fn SettingsPage() -> Element {
    rsx! {
        div { class: "flex-1 flex flex-col overflow-hidden",
            div { class: "h-12 border-b border-border flex items-center px-4 gap-3 shrink-0 bg-background lg:hidden",
                crate::components::SidebarToggle {}
                span { class: "font-mono text-xs text-dim", "ppdrive" }
                span { class: "font-mono text-xs text-border", "›" }
                span { class: "font-mono text-xs text-muted-foreground", "settings" }
                div { class: "flex-1" }
                span { class: "font-mono text-xs px-2 py-1 bg-success/10 text-success border border-success/20 rounded", "● live" }
            }
            div { class: "hidden lg:flex h-12 border-b border-border items-center px-5 gap-2 shrink-0 bg-background",
                span { class: "font-mono text-xs text-dim", "ppdrive" }
                span { class: "font-mono text-xs text-border", "›" }
                span { class: "font-mono text-xs text-muted-foreground", "settings" }
                div { class: "flex-1" }
                div { class: "flex items-center gap-3",
                    span { class: "font-mono text-xs px-2 py-1 bg-success/10 text-success border border-success/20 rounded", "● live" }
                    span { class: "font-mono text-xs text-dim", "21:58:29" }
                }
            }
            div { class: "flex-1 flex flex-col md:flex-row overflow-hidden min-w-0",
                    div { class: "hidden lg:flex w-[200px] border-r border-border bg-card shrink-0 flex-col",
                        div { class: "px-4 py-3 text-xs font-semibold uppercase text-muted-foreground border-b border-border tracking-widest", "Configuration" }
                        nav { class: "py-2 flex flex-col",
                            button { class: "block w-full text-left py-[7px] px-4 bg-primary/10 text-primary text-[13px] cursor-pointer font-medium", "General" }
                            button { class: "block w-full text-left py-[7px] px-4 bg-transparent text-regular text-[13px] cursor-pointer font-normal", "Storage" }
                            button { class: "block w-full text-left py-[7px] px-4 bg-transparent text-regular text-[13px] cursor-pointer font-normal", "Authentication" }
                            button { class: "block w-full text-left py-[7px] px-4 bg-transparent text-regular text-[13px] cursor-pointer font-normal", "Network & TLS" }
                            button { class: "block w-full text-left py-[7px] px-4 bg-transparent text-regular text-[13px] cursor-pointer font-normal", "Replication" }
                            button { class: "block w-full text-left py-[7px] px-4 bg-transparent text-regular text-[13px] cursor-pointer font-normal", "Notifications" }
                            button { class: "block w-full text-left py-[7px] px-4 bg-transparent text-regular text-[13px] cursor-pointer font-normal", "Users & Roles" }
                        }
                    }
                    div { class: "flex-1 flex flex-col overflow-hidden min-w-0",
                        div { class: "lg:hidden border-b border-border bg-card shrink-0 overflow-x-auto",
                            nav { class: "flex overflow-x-auto",
                                button { class: "block py-2.5 px-4 text-left bg-primary/10 text-primary text-[13px] cursor-pointer font-medium shrink-0", "General" }
                                button { class: "block py-2.5 px-4 text-left bg-transparent text-regular text-[13px] cursor-pointer font-normal shrink-0", "Storage" }
                                button { class: "block py-2.5 px-4 text-left bg-transparent text-regular text-[13px] cursor-pointer font-normal shrink-0", "Auth" }
                                button { class: "block py-2.5 px-4 text-left bg-transparent text-regular text-[13px] cursor-pointer font-normal shrink-0", "Network" }
                                button { class: "block py-2.5 px-4 text-left bg-transparent text-regular text-[13px] cursor-pointer font-normal shrink-0", "Replication" }
                                button { class: "block py-2.5 px-4 text-left bg-transparent text-regular text-[13px] cursor-pointer font-normal shrink-0", "Notifications" }
                                button { class: "block py-2.5 px-4 text-left bg-transparent text-regular text-[13px] cursor-pointer font-normal shrink-0", "Roles" }
                            }
                        }
                        div { class: "flex-1 overflow-y-auto p-4 md:p-6 md:pb-7 md:px-7",
                        div {
                            div { class: "pb-4 border-b border-border mb-5",
                                h2 { class: "text-sm font-semibold text-foreground", "General" }
                                p { class: "text-xs mt-1 text-muted-foreground", "Core server configuration and behavior." }
                            }
                            div { class: "flex flex-col gap-5 md:gap-6",
                                div { class: "flex flex-col md:flex-row md:items-start gap-2 md:gap-6",
                                    div { class: "md:min-w-[220px] shrink-0",
                                        div { class: "text-sm text-foreground", "Server Name" }
                                        div { class: "text-xs mt-0.5 text-dim", "Human-readable label shown in the UI" }
                                    }
                                    div { class: "flex-1",
                                        input { class: "bg-background border border-border rounded text-foreground text-[13px] font-sans py-[7px] px-2.5 w-full outline-none", value: "ppdrive-prod-01" }
                                    }
                                }
                                div { class: "flex flex-col md:flex-row md:items-start gap-2 md:gap-6",
                                    div { class: "md:min-w-[220px] shrink-0",
                                        div { class: "text-sm text-foreground", "Base URL" }
                                        div { class: "text-xs mt-0.5 text-dim", "Public URL of this instance" }
                                    }
                                    div { class: "flex-1",
                                        input { class: "bg-background border border-border rounded text-foreground text-[13px] font-mono py-[7px] px-2.5 w-full outline-none", value: "https://storage.company.com" }
                                    }
                                }
                                div { class: "flex flex-col md:flex-row md:items-start gap-2 md:gap-6",
                                    div { class: "md:min-w-[220px] shrink-0",
                                        div { class: "text-sm text-foreground", "API Root Path" }
                                        div { class: "text-xs mt-0.5 text-dim", "Path prefix for all API endpoints" }
                                    }
                                    div { class: "flex-1",
                                        input { class: "bg-background border border-border rounded text-foreground text-[13px] font-mono py-[7px] px-2.5 w-full outline-none", value: "/api/v2" }
                                    }
                                }
                                div { class: "flex flex-col md:flex-row md:items-start gap-2 md:gap-6",
                                    div { class: "md:min-w-[220px] shrink-0",
                                        div { class: "text-sm text-foreground", "Max Upload Size" }
                                        div { class: "text-xs mt-0.5 text-dim", "Maximum single-object upload size" }
                                    }
                                    div { class: "flex-1",
                                        select { class: "bg-background border border-border rounded text-foreground text-[13px] font-sans py-[7px] px-2.5 outline-none cursor-pointer w-full",
                                            option { "100 MB" }
                                            option { "500 MB" }
                                            option { "1 GB" }
                                            option { "5 GB" }
                                            option { "10 GB" }
                                            option { "Unlimited" }
                                        }
                                    }
                                }
                                div { class: "flex flex-col md:flex-row md:items-start gap-2 md:gap-6",
                                    div { class: "md:min-w-[220px] shrink-0",
                                        div { class: "text-sm text-foreground", "Log Level" }
                                    }
                                    div { class: "flex-1",
                                        select { class: "bg-background border border-border rounded text-foreground text-[13px] font-sans py-[7px] px-2.5 outline-none cursor-pointer w-full",
                                            option { "DEBUG" }
                                            option { "INFO" }
                                            option { "WARN" }
                                            option { "ERROR" }
                                        }
                                    }
                                }
                                div { class: "flex flex-col md:flex-row md:items-start gap-2 md:gap-6",
                                    div { class: "md:min-w-[220px] shrink-0",
                                        div { class: "text-sm text-foreground", "Debug Mode" }
                                        div { class: "text-xs mt-0.5 text-dim", "Enables verbose request/response logging" }
                                    }
                                    div { class: "flex-1",
                                        div { class: "flex items-center gap-3",
                                            button { class: "w-9 h-5 rounded-full bg-border cursor-pointer relative transition-colors shrink-0 border-none",
                                                div { class: "absolute top-[3px] left-[3px] w-3.5 h-3.5 rounded-full bg-foreground transition-all" }
                                            }
                                            span { class: "font-mono text-xs text-dim", "disabled" }
                                        }
                                    }
                                }
                                div { class: "flex flex-col md:flex-row md:items-start gap-2 md:gap-6",
                                    div { class: "md:min-w-[220px] shrink-0",
                                        div { class: "text-sm text-foreground", "Prometheus Metrics" }
                                        div { class: "text-xs mt-0.5 text-dim", "Expose /metrics endpoint for scraping" }
                                    }
                                    div { class: "flex-1",
                                        div { class: "flex items-center gap-3",
                                            button { class: "w-9 h-5 rounded-full bg-primary cursor-pointer relative transition-colors shrink-0 border-none",
                                                div { class: "absolute top-[3px] left-[18px] w-3.5 h-3.5 rounded-full bg-foreground transition-all" }
                                            }
                                            span { class: "font-mono text-xs text-primary", "enabled at :9090/metrics" }
                                        }
                                    }
                                }
                                div { class: "flex flex-col md:flex-row md:items-start gap-2 md:gap-6",
                                    div { class: "md:min-w-[220px] shrink-0",
                                        div { class: "text-sm text-foreground", "Audit Log" }
                                        div { class: "text-xs mt-0.5 text-dim", "Log all object access and admin actions" }
                                    }
                                    div { class: "flex-1",
                                        button { class: "w-9 h-5 rounded-full bg-primary cursor-pointer relative transition-colors shrink-0 border-none",
                                            div { class: "absolute top-[3px] left-[18px] w-3.5 h-3.5 rounded-full bg-foreground transition-all" }
                                        }
                                    }
                                }
                            }
                            div { class: "mt-6 pt-4 border-t border-border flex gap-2 justify-end",
                                button { class: "bg-transparent text-muted-foreground border border-border rounded py-[7px] px-4 text-[13px] cursor-pointer", "Reset" }
                                button { class: "bg-primary text-background border-none rounded py-[7px] px-5 text-[13px] font-semibold cursor-pointer", "Save Changes" }
                            }
                        }
                    }
                }
            }
        }
    }
}
