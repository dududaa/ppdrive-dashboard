use dioxus::prelude::*;

#[component]
pub fn DashboardOverviewPage() -> Element {
    rsx! {
        div { class: "flex-1 flex flex-col overflow-hidden",
            div { class: "h-12 border-b border-border flex items-center px-4 gap-3 shrink-0 bg-background lg:hidden",
                crate::components::SidebarToggle {}
                span { class: "font-mono text-xs text-dim", "ppdrive" }
                span { class: "font-mono text-xs text-border", "›" }
                span { class: "font-mono text-xs text-muted-foreground", "dashboard" }
                div { class: "flex-1" }
                span { class: "font-mono text-xs px-2 py-1 bg-success/10 text-success border border-success/20 rounded", "● live" }
            }
            div { class: "hidden lg:flex h-12 border-b border-border items-center px-5 gap-2 shrink-0 bg-background",
                span { class: "font-mono text-xs text-dim", "ppdrive" }
                span { class: "font-mono text-xs text-border", "›" }
                span { class: "font-mono text-xs text-muted-foreground", "dashboard" }
                div { class: "flex-1" }
                div { class: "flex items-center gap-3",
                    span { class: "font-mono text-xs px-2 py-1 bg-success/10 text-success border border-success/20 rounded", "● live" }
                    span { class: "font-mono text-xs text-dim", "21:09:54" }
                }
            }
            div { class: "flex-1 overflow-hidden",
                div { class: "flex flex-col gap-4 p-4 md:p-5 overflow-y-auto h-full",
                    div { class: "flex items-center justify-between",
                        div {
                            h1 { class: "text-base font-semibold text-foreground", "System Overview" }
                            p { class: "font-mono text-xs mt-0.5 text-dim", "ppdrive-prod-01 · Ubuntu 22.04.4 LTS" }
                        }
                        div { class: "flex items-center gap-2",
                            span { class: "font-mono text-xs px-2 py-1 bg-success/12 text-success border border-success/25 rounded", "● ONLINE" }
                        }
                    }
                    div { class: "grid grid-cols-2 lg:grid-cols-4 gap-3",
                        div { class: "bg-card border border-border rounded p-3.5 border-l-[3px] border-l-primary",
                            div { class: "text-xs font-medium uppercase text-muted-foreground tracking-widest", "Registered Users" }
                            div { class: "font-mono mt-1.5 font-semibold text-[22px] text-primary leading-none", "148" }
                            div { class: "font-mono text-xs mt-1 text-dim", "+3 this week" }
                        }
                        div { class: "bg-card border border-border rounded p-3.5",
                            div { class: "text-xs font-medium uppercase text-muted-foreground tracking-widest", "App Clients" }
                            div { class: "font-mono mt-1.5 font-semibold text-[22px] text-foreground leading-none", "12" }
                            div { class: "font-mono text-xs mt-1 text-dim", "9 active" }
                        }
                        div { class: "bg-card border border-border rounded p-3.5",
                            div { class: "text-xs font-medium uppercase text-muted-foreground tracking-widest", "Total Objects" }
                            div { class: "font-mono mt-1.5 font-semibold text-[22px] text-foreground leading-none", "94,271" }
                            div { class: "font-mono text-xs mt-1 text-dim", "across 8 buckets" }
                        }
                        div { class: "bg-card border border-border rounded p-3.5",
                            div { class: "text-xs font-medium uppercase text-muted-foreground tracking-widest", "Storage Used" }
                            div { class: "font-mono mt-1.5 font-semibold text-[22px] text-foreground leading-none", "5.4 TB" }
                            div { class: "font-mono text-xs mt-1 text-dim", "of 12 TB capacity" }
                        }
                    }
                    div { class: "grid gap-3 md:grid-cols-2",
                        div { class: "bg-card border border-border rounded",
                            div { class: "px-4 py-2.5 text-xs font-semibold uppercase text-muted-foreground border-b border-border tracking-widest", "Host System" }
                            div { class: "p-4 flex flex-col gap-2.5",
                                div { class: "flex items-baseline justify-between gap-4",
                                    span { class: "text-xs shrink-0 text-muted-foreground min-w-[100px]", "OS" }
                                    span { class: "font-mono text-xs text-right text-foreground", "Ubuntu 22.04.4 LTS" }
                                }
                                div { class: "flex items-baseline justify-between gap-4",
                                    span { class: "text-xs shrink-0 text-muted-foreground min-w-[100px]", "Kernel" }
                                    span { class: "font-mono text-xs text-right text-foreground", "5.15.0-112-generic" }
                                }
                                div { class: "hidden sm:flex items-baseline justify-between gap-4",
                                    span { class: "text-xs shrink-0 text-muted-foreground min-w-[100px]", "Architecture" }
                                    span { class: "font-mono text-xs text-right text-foreground", "x86_64" }
                                }
                                div { class: "flex items-baseline justify-between gap-4",
                                    span { class: "text-xs shrink-0 text-muted-foreground min-w-[100px]", "Hostname" }
                                    span { class: "font-mono text-xs text-right text-foreground", "ppdrive-prod-01" }
                                }
                                div { class: "flex items-baseline justify-between gap-4",
                                    span { class: "text-xs shrink-0 text-muted-foreground min-w-[100px]", "Uptime" }
                                    span { class: "font-mono text-xs text-right text-foreground", "47 days, 3 hours, 22 minutes" }
                                }
                                div { class: "hidden sm:flex items-baseline justify-between gap-4",
                                    span { class: "text-xs shrink-0 text-muted-foreground min-w-[100px]", "CPU" }
                                    span { class: "font-mono text-xs text-right text-foreground", "Intel(R) Xeon(R) E5-2680 v4 @ 2.40GHz" }
                                }
                                div { class: "hidden sm:flex items-baseline justify-between gap-4",
                                    span { class: "text-xs shrink-0 text-muted-foreground min-w-[100px]", "CPU Cores" }
                                    span { class: "font-mono text-xs text-right text-foreground", "16" }
                                }
                                div { class: "hidden sm:flex items-baseline justify-between gap-4",
                                    span { class: "text-xs shrink-0 text-muted-foreground min-w-[100px]", "Load Avg" }
                                    span { class: "font-mono text-xs text-right text-foreground", "2.34 · 2.18 · 2.05" }
                                }
                            }
                        }
                        div { class: "flex flex-col gap-3",
                            div { class: "bg-card border border-border rounded",
                                div { class: "px-4 py-2.5 text-xs font-semibold uppercase text-muted-foreground border-b border-border tracking-widest", "CPU Usage" }
                                div { class: "p-4",
                                    div { class: "flex justify-between items-end mb-2",
                                        span { class: "font-mono text-2xl font-semibold text-primary", "24.4%" }
                                        span { class: "font-mono text-xs text-dim", "16 cores" }
                                    }
                                    div { class: "bg-background rounded h-1.5 w-full",
                                        div { class: "bg-primary rounded h-full transition-all", style: "width: 24.4033%;" }
                                    }
                                    div { class: "font-mono text-xs mt-2 text-dim", "load avg: 2.34 · 2.18 · 2.05" }
                                }
                            }
                            div { class: "bg-card border border-border rounded",
                                div { class: "px-4 py-2.5 text-xs font-semibold uppercase text-muted-foreground border-b border-border tracking-widest", "Memory" }
                                div { class: "p-4",
                                    div { class: "flex justify-between items-end mb-2",
                                        span { class: "font-mono text-2xl font-semibold text-foreground", "60%" }
                                        span { class: "font-mono text-xs text-dim", "38.4 / 64 GB" }
                                    }
                                    div { class: "bg-background rounded h-1.5 w-full",
                                        div { class: "bg-primary rounded h-full transition-all", style: "width: 60%;" }
                                    }
                                    div { class: "font-mono text-xs mt-2 text-dim", "25.6 GB free" }
                                }
                            }
                            div { class: "bg-card border border-border rounded",
                                div { class: "px-4 py-2.5 text-xs font-semibold uppercase text-muted-foreground border-b border-border tracking-widest", "Throughput" }
                                div { class: "p-4 grid grid-cols-2 gap-3",
                                    div {
                                        div { class: "text-xs text-muted-foreground", "Ingress" }
                                        div { class: "font-mono text-sm font-semibold mt-0.5 text-primary", "124 MB/s" }
                                    }
                                    div {
                                        div { class: "text-xs text-muted-foreground", "Egress" }
                                        div { class: "font-mono text-sm font-semibold mt-0.5 text-primary", "87 MB/s" }
                                    }
                                    div {
                                        div { class: "text-xs text-muted-foreground", "Req/s" }
                                        div { class: "font-mono text-sm font-semibold mt-0.5 text-primary", "2,840" }
                                    }
                                    div {
                                        div { class: "text-xs text-muted-foreground", "Latency p99" }
                                        div { class: "font-mono text-sm font-semibold mt-0.5 text-primary", "12ms" }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "bg-card border border-border rounded",
                        div { class: "px-4 py-2.5 text-xs font-semibold uppercase text-muted-foreground border-b border-border tracking-widest", "Filesystem" }
                        table { class: "w-full border-collapse",
                            thead {
                                tr { class: "border-b border-border",
                    th { class: "text-xs font-medium px-4 py-2 text-left text-dim", "Mount" }
                    th { class: "hidden sm:table-cell text-xs font-medium px-4 py-2 text-left text-dim", "Device" }
                    th { class: "hidden md:table-cell text-xs font-medium px-4 py-2 text-left text-dim", "Type" }
                    th { class: "text-xs font-medium px-4 py-2 text-left text-dim", "Total" }
                    th { class: "text-xs font-medium px-4 py-2 text-left text-dim", "Used" }
                    th { class: "hidden sm:table-cell text-xs font-medium px-4 py-2 text-left text-dim", "Free" }
                    th { class: "text-xs font-medium px-4 py-2 text-left text-dim", "Usage" }
                                }
                            }
                            tbody {
                    tr { class: "bg-card border-b border-[#222630]",
                        td { class: "font-mono text-xs px-4 py-2.5 text-primary", "/" }
                        td { class: "hidden sm:table-cell font-mono text-xs px-4 py-2.5 text-muted-foreground", "/dev/sda1" }
                        td { class: "hidden md:table-cell font-mono text-xs px-4 py-2.5 text-foreground", "ext4" }
                        td { class: "font-mono text-xs px-4 py-2.5 text-foreground", "500 GB" }
                        td { class: "font-mono text-xs px-4 py-2.5 text-foreground", "312 GB" }
                        td { class: "hidden sm:table-cell font-mono text-xs px-4 py-2.5 text-foreground", "188 GB" }
                        td { class: "px-4 py-2.5 min-w-[120px]",
                            div { class: "flex items-center gap-2",
                                div { class: "flex-1",
                                    div { class: "bg-background rounded h-1.5 w-full",
                                        div { class: "bg-primary rounded h-full transition-all", style: "width: 62%;" }
                                    }
                                }
                                span { class: "font-mono text-xs text-primary min-w-[32px]", "62%" }
                            }
                        }
                    }
                    tr { class: "bg-background border-b border-[#222630]",
                        td { class: "font-mono text-xs px-4 py-2.5 text-primary", "/data" }
                        td { class: "hidden sm:table-cell font-mono text-xs px-4 py-2.5 text-muted-foreground", "/dev/sdb1" }
                        td { class: "hidden md:table-cell font-mono text-xs px-4 py-2.5 text-foreground", "xfs" }
                        td { class: "font-mono text-xs px-4 py-2.5 text-foreground", "8.0 TB" }
                        td { class: "font-mono text-xs px-4 py-2.5 text-foreground", "5.4 TB" }
                        td { class: "hidden sm:table-cell font-mono text-xs px-4 py-2.5 text-foreground", "2.6 TB" }
                        td { class: "px-4 py-2.5 min-w-[120px]",
                            div { class: "flex items-center gap-2",
                                div { class: "flex-1",
                                    div { class: "bg-background rounded h-1.5 w-full",
                                        div { class: "bg-warning rounded h-full transition-all", style: "width: 68%;" }
                                    }
                                }
                                span { class: "font-mono text-xs text-warning min-w-[32px]", "68%" }
                            }
                        }
                    }
                    tr { class: "bg-card border-b border-[#222630]",
                        td { class: "font-mono text-xs px-4 py-2.5 text-primary", "/backup" }
                        td { class: "hidden sm:table-cell font-mono text-xs px-4 py-2.5 text-muted-foreground", "/dev/sdc1" }
                        td { class: "hidden md:table-cell font-mono text-xs px-4 py-2.5 text-foreground", "ext4" }
                        td { class: "font-mono text-xs px-4 py-2.5 text-foreground", "4.0 TB" }
                        td { class: "font-mono text-xs px-4 py-2.5 text-foreground", "1.2 TB" }
                        td { class: "hidden sm:table-cell font-mono text-xs px-4 py-2.5 text-foreground", "2.8 TB" }
                        td { class: "px-4 py-2.5 min-w-[120px]",
                            div { class: "flex items-center gap-2",
                                div { class: "flex-1",
                                    div { class: "bg-background rounded h-1.5 w-full",
                                        div { class: "bg-primary rounded h-full transition-all", style: "width: 30%;" }
                                    }
                                }
                                span { class: "font-mono text-xs text-primary min-w-[32px]", "30%" }
                            }
                        }
                    }
                            }
                        }
                    }
                    div { class: "bg-card border border-border rounded",
                        div { class: "px-4 py-2.5 flex items-center justify-between border-b border-border",
                            span { class: "text-xs font-semibold uppercase text-muted-foreground tracking-widest", "Recent Activity" }
                            span { class: "font-mono text-xs text-dim", "live · refreshing every 30s" }
                        }
                        div { class: "py-3 px-4 flex flex-col gap-1.5",
                            div { class: "flex flex-col sm:flex-row sm:items-start gap-1 sm:gap-3",
                                span { class: "font-mono text-xs shrink-0 text-dim sm:min-w-[140px]", "2024-11-15 14:32:18" }
                                span { class: "font-mono text-xs text-success min-w-[40px] inline-block", "INFO" }
                                span { class: "font-mono text-xs text-regular", "Upload completed: /media/videos/demo-reel-v3.mp4 (2.1 GB)" }
                            }
                            div { class: "flex flex-col sm:flex-row sm:items-start gap-1 sm:gap-3",
                                span { class: "font-mono text-xs shrink-0 text-dim sm:min-w-[140px]", "2024-11-15 14:28:44" }
                                span { class: "font-mono text-xs text-success min-w-[40px] inline-block", "INFO" }
                                span { class: "font-mono text-xs text-regular", "Client auth: app_client_id=c8f2e91a connected from 10.0.1.45" }
                            }
                            div { class: "flex flex-col sm:flex-row sm:items-start gap-1 sm:gap-3",
                                span { class: "font-mono text-xs shrink-0 text-dim sm:min-w-[140px]", "2024-11-15 14:21:07" }
                                span { class: "font-mono text-xs text-warning min-w-[40px] inline-block", "WARN" }
                                span { class: "font-mono text-xs text-regular", "Disk usage on /data exceeded 65% threshold" }
                            }
                            div { class: "flex flex-col sm:flex-row sm:items-start gap-1 sm:gap-3",
                                span { class: "font-mono text-xs shrink-0 text-dim sm:min-w-[140px]", "2024-11-15 14:15:33" }
                                span { class: "font-mono text-xs text-success min-w-[40px] inline-block", "INFO" }
                                span { class: "font-mono text-xs text-regular", "Download: /documents/reports/q3-financials.pdf — client c8f2e91a" }
                            }
                            div { class: "flex flex-col sm:flex-row sm:items-start gap-1 sm:gap-3",
                                span { class: "font-mono text-xs shrink-0 text-dim sm:min-w-[140px]", "2024-11-15 14:09:02" }
                                span { class: "font-mono text-xs text-success min-w-[40px] inline-block", "INFO" }
                                span { class: "font-mono text-xs text-regular", "Bucket created: archive-2024 by user martin.kowalski@company.com" }
                            }
                            div { class: "flex flex-col sm:flex-row sm:items-start gap-1 sm:gap-3",
                                span { class: "font-mono text-xs shrink-0 text-dim sm:min-w-[140px]", "2024-11-15 13:58:17" }
                                span { class: "font-mono text-xs text-danger min-w-[40px] inline-block", "ERROR" }
                                span { class: "font-mono text-xs text-regular", "Failed multipart upload: /raw/sensor-data-20241115.csv — checksum mismatch" }
                            }
                            div { class: "flex flex-col sm:flex-row sm:items-start gap-1 sm:gap-3",
                                span { class: "font-mono text-xs shrink-0 text-dim sm:min-w-[140px]", "2024-11-15 13:47:55" }
                                span { class: "font-mono text-xs text-success min-w-[40px] inline-block", "INFO" }
                                span { class: "font-mono text-xs text-regular", "User login: sandra.lee@company.com from 192.168.1.102" }
                            }
                            div { class: "flex flex-col sm:flex-row sm:items-start gap-1 sm:gap-3",
                                span { class: "font-mono text-xs shrink-0 text-dim sm:min-w-[140px]", "2024-11-15 13:40:28" }
                                span { class: "font-mono text-xs text-success min-w-[40px] inline-block", "INFO" }
                                span { class: "font-mono text-xs text-regular", "Presigned URL generated for /media/images/product-banner.jpg (TTL 3600s)" }
                            }
                            div { class: "flex flex-col sm:flex-row sm:items-start gap-1 sm:gap-3",
                                span { class: "font-mono text-xs shrink-0 text-dim sm:min-w-[140px]", "2024-11-15 13:31:11" }
                                span { class: "font-mono text-xs text-success min-w-[40px] inline-block", "INFO" }
                                span { class: "font-mono text-xs text-regular", "Replication sync completed: 847 objects → replica-eu-west" }
                            }
                            div { class: "flex flex-col sm:flex-row sm:items-start gap-1 sm:gap-3",
                                span { class: "font-mono text-xs shrink-0 text-dim sm:min-w-[140px]", "2024-11-15 13:20:44" }
                                span { class: "font-mono text-xs text-warning min-w-[40px] inline-block", "WARN" }
                                span { class: "font-mono text-xs text-regular", "Rate limit hit: client app_frontend_v2 — 150 req/min" }
                            }
                        }
                    }
                }
            }
        }
    }
}
