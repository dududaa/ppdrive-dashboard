use dioxus::prelude::*;

#[component]
pub fn AppClientsPage() -> Element {
    rsx! {
        div { class: "flex-1 flex flex-col overflow-hidden",
            div { class: "h-12 border-b border-border flex items-center px-4 gap-3 shrink-0 bg-background lg:hidden",
                crate::components::SidebarToggle {}
                span { class: "font-mono text-xs text-dim", "ppdrive" }
                span { class: "font-mono text-xs text-border", "›" }
                span { class: "font-mono text-xs text-muted-foreground", "app clients" }
                div { class: "flex-1" }
                span { class: "font-mono text-xs px-2 py-1 bg-success/10 text-success border border-success/20 rounded", "● live" }
            }
            div { class: "hidden lg:flex h-12 border-b border-border items-center px-5 gap-2 shrink-0 bg-background",
                span { class: "font-mono text-xs text-dim", "ppdrive" }
                span { class: "font-mono text-xs text-border", "›" }
                span { class: "font-mono text-xs text-muted-foreground", "app clients" }
                div { class: "flex-1" }
                div { class: "flex items-center gap-3",
                    span { class: "font-mono text-xs px-2 py-1 bg-success/10 text-success border border-success/20 rounded", "● live" }
                    span { class: "font-mono text-xs text-dim", "21:56:17" }
                }
            }
            div { class: "flex-1 overflow-hidden",
                div { class: "flex flex-col h-full overflow-hidden",
                    div { class: "py-4 px-4 md:px-5 border-b border-border bg-background shrink-0",
                        div { class: "flex items-center justify-between mb-3 gap-3",
                            div {
                                h1 { class: "text-base font-semibold text-foreground", "App Clients" }
                                p { class: "font-mono text-xs mt-0.5 text-dim", "4 active · 6 total" }
                            }
                            button { class: "bg-primary text-background border-none rounded py-[7px] px-4 text-[13px] font-semibold cursor-pointer shrink-0", "+ New Client" }
                        }
                        div { class: "flex flex-col sm:flex-row sm:items-center gap-3",
                            div { class: "flex gap-1 flex-wrap",
                                button { class: "bg-primary text-background border border-primary rounded py-1 px-2.5 text-xs cursor-pointer font-semibold capitalize", "all" }
                                button { class: "bg-transparent text-muted-foreground border border-border rounded py-1 px-2.5 text-xs cursor-pointer font-normal capitalize", "active" }
                                button { class: "bg-transparent text-muted-foreground border border-border rounded py-1 px-2.5 text-xs cursor-pointer font-normal capitalize", "suspended" }
                                button { class: "bg-transparent text-muted-foreground border border-border rounded py-1 px-2.5 text-xs cursor-pointer font-normal capitalize", "revoked" }
                            }
                            input {
                                placeholder: "Search by name or ID...",
                                class: "bg-card border border-border rounded text-foreground text-xs font-mono py-[5px] px-2.5 w-full sm:w-[220px] outline-none",
                                value: "",
                            }
                        }
                    }
                    div { class: "overflow-y-auto flex-1",
                        table { class: "w-full border-collapse",
                            thead { class: "sticky top-0 bg-card z-[1]",
                                tr { class: "border-b border-border",
                    th { class: "text-xs font-medium px-4 py-2.5 text-left text-dim whitespace-nowrap", "Client" }
                    th { class: "text-xs font-medium px-4 py-2.5 text-left text-dim whitespace-nowrap", "Status" }
                    th { class: "hidden lg:table-cell text-xs font-medium px-4 py-2.5 text-left text-dim whitespace-nowrap", "Access Key" }
                    th { class: "hidden md:table-cell text-xs font-medium px-4 py-2.5 text-left text-dim whitespace-nowrap", "Permissions" }
                    th { class: "hidden sm:table-cell text-xs font-medium px-4 py-2.5 text-left text-dim whitespace-nowrap", "Last Seen" }
                    th { class: "hidden md:table-cell text-xs font-medium px-4 py-2.5 text-left text-dim whitespace-nowrap", "Req/30d" }
                    th { class: "hidden lg:table-cell text-xs font-medium px-4 py-2.5 text-left text-dim whitespace-nowrap", "Transferred" }
                    th { class: "text-xs font-medium px-4 py-2.5 text-left text-dim whitespace-nowrap" }
                                }
                            }
                            tbody {
                    tr { class: "bg-background border-b border-[#222630] cursor-pointer",
                        td { class: "px-4 py-3",
                            div { class: "text-sm font-medium text-foreground", "Frontend Web App" }
                            div { class: "font-mono text-xs mt-0.5 text-dim", "id: c8f2e91a" }
                        }
                        td { class: "px-4 py-3",
                            span { class: "font-mono text-xs text-success border-l-[3px] border-l-success pl-1.5", "active" }
                        }
                        td { class: "hidden lg:table-cell px-4 py-3",
                            span { class: "font-mono text-xs text-muted-foreground", "AKIAIOSFODNN7EXA…" }
                        }
                        td { class: "hidden md:table-cell px-4 py-3",
                            div { class: "flex gap-1 flex-wrap",
                                span { class: "font-mono text-xs px-1.5 py-0.5 bg-primary/13 text-primary border border-primary/27 rounded", "read" }
                                span { class: "font-mono text-xs px-1.5 py-0.5 bg-warning/13 text-warning border border-warning/27 rounded", "write" }
                                span { class: "font-mono text-xs px-1.5 py-0.5 bg-danger/13 text-danger border border-danger/27 rounded", "delete" }
                            }
                        }
                        td { class: "hidden sm:table-cell px-4 py-3", span { class: "font-mono text-xs text-muted-foreground", "2024-11-15 14:28:44" } }
                        td { class: "hidden md:table-cell px-4 py-3", span { class: "font-mono text-xs text-foreground", "142,830" } }
                        td { class: "hidden lg:table-cell px-4 py-3", span { class: "font-mono text-xs text-foreground", "847 GB" } }
                        td { class: "px-4 py-3", span { class: "font-mono text-xs text-primary", "view →" } }
                    }
                    tr { class: "bg-secondary border-b border-[#222630] cursor-pointer",
                        td { class: "px-4 py-3",
                            div { class: "text-sm font-medium text-foreground", "Backup Service" }
                            div { class: "font-mono text-xs mt-0.5 text-dim", "id: a3d901bc" }
                        }
                        td { class: "px-4 py-3",
                            span { class: "font-mono text-xs text-success border-l-[3px] border-l-success pl-1.5", "active" }
                        }
                        td { class: "hidden lg:table-cell px-4 py-3",
                            span { class: "font-mono text-xs text-muted-foreground", "AKIAJSIE27APIHZQ…" }
                        }
                        td { class: "hidden md:table-cell px-4 py-3",
                            div { class: "flex gap-1 flex-wrap",
                                span { class: "font-mono text-xs px-1.5 py-0.5 bg-primary/13 text-primary border border-primary/27 rounded", "read" }
                                span { class: "font-mono text-xs px-1.5 py-0.5 bg-warning/13 text-warning border border-warning/27 rounded", "write" }
                            }
                        }
                        td { class: "hidden sm:table-cell px-4 py-3", span { class: "font-mono text-xs text-muted-foreground", "2024-11-15 03:00:12" } }
                        td { class: "hidden md:table-cell px-4 py-3", span { class: "font-mono text-xs text-foreground", "2,340" } }
                        td { class: "hidden lg:table-cell px-4 py-3", span { class: "font-mono text-xs text-foreground", "5.4 TB" } }
                        td { class: "px-4 py-3", span { class: "font-mono text-xs text-primary", "view →" } }
                    }
                    tr { class: "bg-background border-b border-[#222630] cursor-pointer",
                        td { class: "px-4 py-3",
                            div { class: "text-sm font-medium text-foreground", "Mobile SDK v2" }
                            div { class: "font-mono text-xs mt-0.5 text-dim", "id: f7b44e22" }
                        }
                        td { class: "px-4 py-3",
                            span { class: "font-mono text-xs text-success border-l-[3px] border-l-success pl-1.5", "active" }
                        }
                        td { class: "hidden lg:table-cell px-4 py-3",
                            span { class: "font-mono text-xs text-muted-foreground", "AKIAXWIZLH5ZWN3X…" }
                        }
                        td { class: "hidden md:table-cell px-4 py-3",
                            div { class: "flex gap-1 flex-wrap",
                                span { class: "font-mono text-xs px-1.5 py-0.5 bg-primary/13 text-primary border border-primary/27 rounded", "read" }
                            }
                        }
                        td { class: "hidden sm:table-cell px-4 py-3", span { class: "font-mono text-xs text-muted-foreground", "2024-11-15 14:31:07" } }
                        td { class: "hidden md:table-cell px-4 py-3", span { class: "font-mono text-xs text-foreground", "984,210" } }
                        td { class: "hidden lg:table-cell px-4 py-3", span { class: "font-mono text-xs text-foreground", "2.1 TB" } }
                        td { class: "px-4 py-3", span { class: "font-mono text-xs text-primary", "view →" } }
                    }
                    tr { class: "bg-secondary border-b border-[#222630] cursor-pointer",
                        td { class: "px-4 py-3",
                            div { class: "text-sm font-medium text-foreground", "CI Pipeline" }
                            div { class: "font-mono text-xs mt-0.5 text-dim", "id: e12c77fd" }
                        }
                        td { class: "px-4 py-3",
                            span { class: "font-mono text-xs text-success border-l-[3px] border-l-success pl-1.5", "active" }
                        }
                        td { class: "hidden lg:table-cell px-4 py-3",
                            span { class: "font-mono text-xs text-muted-foreground", "AKIAXYZ987ABC654…" }
                        }
                        td { class: "hidden md:table-cell px-4 py-3",
                            div { class: "flex gap-1 flex-wrap",
                                span { class: "font-mono text-xs px-1.5 py-0.5 bg-primary/13 text-primary border border-primary/27 rounded", "read" }
                                span { class: "font-mono text-xs px-1.5 py-0.5 bg-warning/13 text-warning border border-warning/27 rounded", "write" }
                            }
                        }
                        td { class: "hidden sm:table-cell px-4 py-3", span { class: "font-mono text-xs text-muted-foreground", "2024-11-15 12:44:02" } }
                        td { class: "hidden md:table-cell px-4 py-3", span { class: "font-mono text-xs text-foreground", "8,920" } }
                        td { class: "hidden lg:table-cell px-4 py-3", span { class: "font-mono text-xs text-foreground", "344 GB" } }
                        td { class: "px-4 py-3", span { class: "font-mono text-xs text-primary", "view →" } }
                    }
                    tr { class: "bg-background border-b border-[#222630] cursor-pointer",
                        td { class: "px-4 py-3",
                            div { class: "text-sm font-medium text-foreground", "Analytics Exporter (Legacy)" }
                            div { class: "font-mono text-xs mt-0.5 text-dim", "id: b9aa3310" }
                        }
                        td { class: "px-4 py-3",
                            span { class: "font-mono text-xs text-warning border-l-[3px] border-l-warning pl-1.5", "suspended" }
                        }
                        td { class: "hidden lg:table-cell px-4 py-3",
                            span { class: "font-mono text-xs text-muted-foreground", "AKIALDEG000TESTK…" }
                        }
                        td { class: "hidden md:table-cell px-4 py-3",
                            div { class: "flex gap-1 flex-wrap",
                                span { class: "font-mono text-xs px-1.5 py-0.5 bg-primary/13 text-primary border border-primary/27 rounded", "read" }
                            }
                        }
                        td { class: "hidden sm:table-cell px-4 py-3", span { class: "font-mono text-xs text-muted-foreground", "2024-09-03 08:14:55" } }
                        td { class: "hidden md:table-cell px-4 py-3", span { class: "font-mono text-xs text-foreground", "0" } }
                        td { class: "hidden lg:table-cell px-4 py-3", span { class: "font-mono text-xs text-foreground", "0 B" } }
                        td { class: "px-4 py-3", span { class: "font-mono text-xs text-primary", "view →" } }
                    }
                    tr { class: "bg-secondary border-b border-[#222630] cursor-pointer",
                        td { class: "px-4 py-3",
                            div { class: "text-sm font-medium text-foreground", "Old Desktop Client" }
                            div { class: "font-mono text-xs mt-0.5 text-dim", "id: d4f81a05" }
                        }
                        td { class: "px-4 py-3",
                            span { class: "font-mono text-xs text-danger border-l-[3px] border-l-danger pl-1.5", "revoked" }
                        }
                        td { class: "hidden lg:table-cell px-4 py-3",
                            span { class: "font-mono text-xs text-muted-foreground", "AKIAREVOKED00000…" }
                        }
                        td { class: "hidden md:table-cell px-4 py-3",
                            div { class: "flex gap-1 flex-wrap",
                                span { class: "font-mono text-xs text-dim", "—" }
                            }
                        }
                        td { class: "hidden sm:table-cell px-4 py-3", span { class: "font-mono text-xs text-muted-foreground", "2024-07-20 15:30:00" } }
                        td { class: "hidden md:table-cell px-4 py-3", span { class: "font-mono text-xs text-foreground", "0" } }
                        td { class: "hidden lg:table-cell px-4 py-3", span { class: "font-mono text-xs text-foreground", "0 B" } }
                        td { class: "px-4 py-3", span { class: "font-mono text-xs text-primary", "view →" } }
                    }
                            }
                        }
                    }
                }
            }
        }
    }
}
