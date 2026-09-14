use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
enum FsNode {
    Dir {
        name: String,
        children: Vec<FsNode>,
    },
    File {
        name: String,
        size: String,
        kind: FileKind,
        modified: String,
    },
}

#[derive(Clone, Debug, PartialEq)]
enum FileKind {
    Video,
    Audio,
    Image,
    Document,
    Archive,
    Unknown,
}

impl FsNode {
    fn dir(name: &str, children: Vec<FsNode>) -> Self {
        FsNode::Dir {
            name: name.to_string(),
            children,
        }
    }

    fn file(name: &str, size: &str, kind: FileKind, modified: &str) -> Self {
        FsNode::File {
            name: name.to_string(),
            size: size.to_string(),
            kind,
            modified: modified.to_string(),
        }
    }

    fn name(&self) -> &str {
        match self {
            FsNode::Dir { name, .. } => name,
            FsNode::File { name, .. } => name,
        }
    }

    fn is_dir(&self) -> bool {
        matches!(self, FsNode::Dir { .. })
    }

    fn children(&self) -> &[FsNode] {
        match self {
            FsNode::Dir { children, .. } => children,
            FsNode::File { .. } => &[],
        }
    }
}

fn build_fs() -> FsNode {
    FsNode::dir("/root", vec![
        FsNode::dir("media", vec![
            FsNode::dir("videos", vec![
                FsNode::file("demo-reel-v3.mp4", "2.1 GB", FileKind::Video, "2024-11-15"),
                FsNode::file("product-tour.webm", "480 MB", FileKind::Video, "2024-11-10"),
                FsNode::file("interview-raw.mov", "8.7 GB", FileKind::Video, "2024-11-08"),
            ]),
            FsNode::dir("audio", vec![
                FsNode::file("podcast-ep42.mp3", "68 MB", FileKind::Audio, "2024-11-14"),
                FsNode::file("bg-music-loop.ogg", "12 MB", FileKind::Audio, "2024-10-20"),
                FsNode::file("voiceover-v2.wav", "245 MB", FileKind::Audio, "2024-11-12"),
            ]),
            FsNode::dir("images", vec![
                FsNode::file("product-banner.jpg", "4.2 MB", FileKind::Image, "2024-11-13"),
                FsNode::file("team-photo.png", "8.1 MB", FileKind::Image, "2024-11-01"),
                FsNode::file("logo-dark.svg", "12 KB", FileKind::Image, "2024-09-15"),
            ]),
        ]),
        FsNode::dir("documents", vec![
            FsNode::dir("reports", vec![
                FsNode::file("q3-financials.pdf", "2.4 MB", FileKind::Document, "2024-10-30"),
                FsNode::file("annual-review.docx", "1.8 MB", FileKind::Document, "2024-11-05"),
                FsNode::file("budget-2025.xlsx", "340 KB", FileKind::Document, "2024-11-10"),
            ]),
            FsNode::dir("contracts", vec![
                FsNode::file("vendor-agreement.pdf", "890 KB", FileKind::Document, "2024-08-22"),
                FsNode::file("nda-acme-corp.pdf", "420 KB", FileKind::Document, "2024-09-18"),
            ]),
            FsNode::file("readme.txt", "4 KB", FileKind::Document, "2024-11-01"),
        ]),
        FsNode::dir("data", vec![
            FsNode::dir("exports", vec![
                FsNode::file("sensor-data-20241115.csv", "156 MB", FileKind::Unknown, "2024-11-15"),
                FsNode::file("user-analytics.json", "23 MB", FileKind::Unknown, "2024-11-14"),
                FsNode::file("metrics-dump.parquet", "890 MB", FileKind::Unknown, "2024-11-13"),
            ]),
            FsNode::dir("backups", vec![
                FsNode::file("db-backup-20241115.sql.gz", "3.2 GB", FileKind::Archive, "2024-11-15"),
                FsNode::file("config-snapshot.tar", "128 KB", FileKind::Archive, "2024-11-14"),
            ]),
        ]),
        FsNode::dir("archive-2024", vec![
            FsNode::file("project-alpha.zip", "45 GB", FileKind::Archive, "2024-06-30"),
            FsNode::file("legacy-docs.tar.gz", "12 GB", FileKind::Archive, "2024-03-15"),
        ]),
    ])
}

fn kind_icon(kind: &FileKind) -> &'static str {
    match kind {
        FileKind::Video => "🎬",
        FileKind::Audio => "🎵",
        FileKind::Image => "🖼",
        FileKind::Document => "📄",
        FileKind::Archive => "📦",
        FileKind::Unknown => "📄",
    }
}

fn kind_color(kind: &FileKind) -> &'static str {
    match kind {
        FileKind::Video => "text-success-light",
        FileKind::Audio => "text-primary",
        FileKind::Image => "text-warning",
        FileKind::Document => "text-foreground",
        FileKind::Archive => "text-dim",
        FileKind::Unknown => "text-dim",
    }
}

#[derive(Clone, Copy, PartialEq)]
enum MobileView {
    Tree,
    Contents,
    Preview,
}

#[component]
pub fn FileExplorer() -> Element {
    let fs = use_memo(|| build_fs());
    let expanded = use_signal(|| {
        let mut set = std::collections::HashSet::new();
        set.insert("/root".to_string());
        set.insert("/root/media".to_string());
        set
    });
    let selected_folder: Signal<Option<String>> = use_signal(|| Some("/root".to_string()));
    let mut selected_file: Signal<Option<FsNode>> = use_signal(|| None);
    let mut mobile_view = use_signal(|| MobileView::Tree);

    let current_folder = use_memo(move || {
        let path = selected_folder()?;
        find_node(&fs(), &path)
    });

    rsx! {
        div { class: "flex-1 flex flex-col overflow-hidden",
            div { class: "h-12 border-b border-border flex items-center px-4 gap-3 shrink-0 bg-background lg:hidden",
                crate::components::SidebarToggle {}
                span { class: "font-mono text-xs text-dim", "ppdrive" }
                span { class: "font-mono text-xs text-border", "›" }
                span { class: "font-mono text-xs text-muted-foreground", "file explorer" }
                div { class: "flex-1" }
                span { class: "font-mono text-xs px-2 py-1 bg-success/10 text-success border border-success/20 rounded", "● live" }
            }
            div { class: "hidden lg:flex h-12 border-b border-border items-center px-5 gap-2 shrink-0 bg-background",
                span { class: "font-mono text-xs text-dim", "ppdrive" }
                span { class: "font-mono text-xs text-border", "›" }
                span { class: "font-mono text-xs text-muted-foreground", "file explorer" }
                div { class: "flex-1" }
                div { class: "flex items-center gap-3",
                    span { class: "font-mono text-xs px-2 py-1 bg-success/10 text-success border border-success/20 rounded", "● live" }
                    span { class: "font-mono text-xs text-dim", "21:19:46" }
                }
            }
            div { class: "flex-1 overflow-hidden",
                div { class: "hidden lg:flex h-full overflow-hidden",
                    div { class: "w-56 lg:w-60 border-r border-border bg-card flex flex-col overflow-hidden shrink-0",
                        div { class: "px-3 py-2.5 text-xs font-semibold uppercase text-muted-foreground border-b border-border tracking-widest", "Buckets & Folders" }
                        div { class: "overflow-y-auto flex-1 py-1",
                            TreeView {
                                node: fs(),
                                path: "/root".to_string(),
                                depth: 0,
                                expanded,
                                selected_folder,
                                selected_file,
                            }
                        }
                        div { class: "border-t border-border py-2 px-3",
                            div { class: "font-mono text-xs text-dim", "4 buckets · 94,271 objects" }
                        }
                    }
                    div { class: "flex-1 flex flex-col overflow-hidden border-r border-border min-w-0",
                        div { class: "border-b border-border py-2 px-3 flex items-center gap-2.5 bg-background shrink-0",
                            span { class: "font-mono text-xs text-dim truncate",
                                {selected_folder().as_deref().unwrap_or("/root")}
                            }
                            div { class: "flex-1" }
                            input {
                                placeholder: "Search files...",
                                class: "bg-card border border-border rounded text-foreground text-xs font-mono py-[5px] px-2.5 w-[200px] outline-none",
                                value: "",
                            }
                            button { class: "bg-primary text-background border-none rounded py-[5px] px-3 text-xs font-semibold cursor-pointer", "Upload" }
                        }
                        div { class: "grid grid-cols-[1fr_100px_120px_80px] py-1.5 px-3 border-b border-border bg-card shrink-0",
                            span { class: "text-xs font-medium text-dim", "Name" }
                            span { class: "text-xs font-medium text-dim", "Size" }
                            span { class: "text-xs font-medium text-dim", "Modified" }
                            span { class: "text-xs font-medium text-dim", "Type" }
                        }
                        div { class: "overflow-y-auto flex-1",
                            if let Some(folder) = current_folder() {
                                for child in folder.children() {
                                    FileRow {
                                        node: child.clone(),
                                        selected_file,
                                        expanded,
                                        selected_folder,
                                    }
                                }
                            }
                        }
                    }
                    div { class: "w-[300px] xl:w-[340px] bg-card flex flex-col overflow-hidden shrink-0",
                        div { class: "py-2 px-3 border-b border-border flex items-center justify-between shrink-0",
                            span { class: "text-xs font-semibold uppercase text-muted-foreground tracking-widest", "Preview" }
                            if selected_file().is_some() {
                                button {
                                    class: "bg-transparent border-none text-dim cursor-pointer text-lg leading-none px-0.5",
                                    onclick: move |_| selected_file.set(None),
                                    "×"
                                }
                            }
                        }
                        div { class: "overflow-y-auto flex-1 p-3.5",
                            if let Some(file) = selected_file() {
                                {preview_for(&file)}
                            } else {
                                div { class: "flex flex-col items-center justify-center h-full gap-3 text-dim",
                                    span { class: "text-3xl", "📂" }
                                    span { class: "text-xs", "Select a file to preview" }
                                }
                            }
                        }
                    }
                }
                div { class: "flex lg:hidden flex-col h-full overflow-hidden",
                    if mobile_view() == MobileView::Tree {
                        div { class: "px-3 py-2.5 text-xs font-semibold uppercase text-muted-foreground border-b border-border tracking-widest shrink-0", "Buckets & Folders" }
                        div { class: "overflow-y-auto flex-1 py-1",
                            TreeView {
                                node: fs(),
                                path: "/root".to_string(),
                                depth: 0,
                                expanded,
                                selected_folder,
                                selected_file,
                            }
                        }
                        div { class: "border-t border-border py-2 px-3 shrink-0",
                            div { class: "font-mono text-xs text-dim", "4 buckets · 94,271 objects" }
                        }
                    } else if mobile_view() == MobileView::Contents {
                        div { class: "border-b border-border py-2 px-3 flex items-center gap-2 bg-background shrink-0",
                            button {
                                class: "bg-transparent border-none cursor-pointer text-primary text-xs flex items-center gap-1",
                                onclick: move |_| mobile_view.set(MobileView::Tree),
                                "← Back"
                            }
                            span { class: "font-mono text-xs text-dim truncate",
                                {selected_folder().as_deref().unwrap_or("/root")}
                            }
                            div { class: "flex-1" }
                            button { class: "bg-primary text-background border-none rounded py-[5px] px-3 text-xs font-semibold cursor-pointer", "Upload" }
                        }
                        div { class: "grid grid-cols-[1fr_80px_80px] py-1.5 px-3 border-b border-border bg-card shrink-0",
                            span { class: "text-xs font-medium text-dim", "Name" }
                            span { class: "text-xs font-medium text-dim", "Size" }
                            span { class: "text-xs font-medium text-dim", "Type" }
                        }
                        div { class: "overflow-y-auto flex-1",
                            if let Some(folder) = current_folder() {
                                for child in folder.children() {
                                    MobileFileRow {
                                        node: child.clone(),
                                        selected_file,
                                        expanded,
                                        selected_folder,
                                        mobile_view,
                                    }
                                }
                            }
                        }
                    } else if mobile_view() == MobileView::Preview {
                        div { class: "border-b border-border py-2 px-3 flex items-center gap-2 bg-background shrink-0",
                            button {
                                class: "bg-transparent border-none cursor-pointer text-primary text-xs flex items-center gap-1",
                                onclick: move |_| {
                                    selected_file.set(None);
                                    mobile_view.set(MobileView::Contents);
                                },
                                "← Back"
                            }
                            span { class: "text-xs font-semibold text-foreground truncate", "Preview" }
                            div { class: "flex-1" }
                        }
                        div { class: "overflow-y-auto flex-1 p-4",
                            if let Some(file) = selected_file() {
                                {preview_for(&file)}
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TreeView(
    node: FsNode,
    path: String,
    depth: i32,
    mut expanded: Signal<std::collections::HashSet<String>>,
    mut selected_folder: Signal<Option<String>>,
    mut selected_file: Signal<Option<FsNode>>,
) -> Element {
    if !node.is_dir() {
        return rsx! {};
    }

    let is_expanded = expanded().contains(&path);
    let is_selected = selected_folder() == Some(path.clone());
    let indent = depth * 12;

    let children: Vec<FsNode> = node.children().to_vec();
    let all_children: Vec<(FsNode, String)> = children
        .iter()
        .map(|c| {
            let p = format!("{}/{}", path.trim_end_matches('/'), c.name());
            (c.clone(), p)
        })
        .collect();
    let path_for_click = path.clone();

    rsx! {
        div {
            class: if is_selected {
                "flex items-center gap-1.5 cursor-pointer py-[5px] px-2 border-l-2 border-l-primary bg-primary/12"
            } else {
                "flex items-center gap-1.5 cursor-pointer py-[5px] px-2 border-l-2 border-l-transparent hover:bg-secondary/50"
            },
            style: "padding-left: {indent + 8}px;",
            onclick: move |_| {
                selected_file.set(None);
                let mut e = expanded.write();
                if e.contains(&path_for_click) {
                    e.remove(&path_for_click);
                } else {
                    e.insert(path_for_click.clone());
                }
                selected_folder.set(Some(path_for_click.clone()));
            },
            span { class: "font-mono text-xs text-dim w-3 shrink-0",
                if is_expanded { "▾" } else { "▸" }
            }
            span { class: "font-mono text-xs shrink-0", "📁" }
            span { class: "font-mono text-xs truncate text-foreground", "{node.name()}" }
        }
        if is_expanded {
            for (child, child_path) in all_children {
                if child.is_dir() {
                    TreeView {
                        node: child,
                        path: child_path,
                        depth: depth + 1,
                        expanded,
                        selected_folder,
                        selected_file,
                    }
                } else {
                    TreeFile {
                        node: child,
                        depth: depth + 1,
                        selected_file,
                    }
                }
            }
        }
    }
}

#[component]
fn TreeFile(
    node: FsNode,
    depth: i32,
    mut selected_file: Signal<Option<FsNode>>,
) -> Element {
    let indent = depth * 12;
    let (icon, color) = match &node {
        FsNode::File { kind, .. } => (kind_icon(kind), kind_color(kind)),
        _ => ("📄", "text-dim"),
    };
    let file = node.clone();
    let name = node.name().to_string();

    rsx! {
        div {
            class: "flex items-center gap-1.5 cursor-pointer py-[5px] px-2 border-l-2 border-l-transparent hover:bg-secondary/50",
            style: "padding-left: {indent + 16}px;",
            onclick: move |_| {
                selected_file.set(Some(file.clone()));
            },
            span { class: "font-mono text-xs shrink-0", "{icon}" }
            span { class: "font-mono text-xs truncate {color}", "{name}" }
        }
    }
}

#[component]
fn FileRow(
    node: FsNode,
    mut selected_file: Signal<Option<FsNode>>,
    mut expanded: Signal<std::collections::HashSet<String>>,
    mut selected_folder: Signal<Option<String>>,
) -> Element {
    let is_selected_dir = node.is_dir() && selected_folder() == Some(format!(
        "{}/{}",
        if let Some(parent) = selected_folder() {
            parent
        } else {
            "/".to_string()
        },
        node.name()
    ));

    let row_bg = if is_selected_dir {
        "bg-primary/8"
    } else {
        "bg-background"
    };

    if node.is_dir() {
        let dir_path = format!(
            "{}/{}",
            selected_folder().unwrap_or_default().trim_end_matches('/'),
            node.name()
        );
        let dir_path_clone = dir_path.clone();

        rsx! {
            div {
                class: "grid grid-cols-[1fr_100px_120px_80px] py-[7px] px-3 cursor-pointer {row_bg} border-b border-[#222630] items-center hover:bg-secondary/50",
                onclick: move |_| {
                    selected_file.set(None);
                    selected_folder.set(Some(dir_path_clone.clone()));
                    expanded.write().insert(dir_path_clone.clone());
                },
                div { class: "flex items-center gap-2 min-w-0",
                    span { class: "text-xs shrink-0", "📁" }
                    span { class: "font-mono text-xs truncate text-foreground", "{node.name()}" }
                }
                span { class: "font-mono text-xs text-dim", "—" }
                span { class: "font-mono text-xs text-dim", "—" }
                span { class: "font-mono text-xs text-dim", "folder" }
            }
        }
    } else if let FsNode::File { size, kind, modified, .. } = &node {
        let file = node.clone();
        let color_class = kind_color(kind);

        rsx! {
            div {
                class: "grid grid-cols-[1fr_100px_120px_80px] py-[7px] px-3 cursor-pointer bg-background border-b border-[#222630] items-center hover:bg-secondary/50",
                onclick: move |_| {
                    selected_file.set(Some(file.clone()));
                },
                div { class: "flex items-center gap-2 min-w-0",
                    span { class: "text-xs shrink-0", "{kind_icon(kind)}" }
                    span { class: "font-mono text-xs truncate text-regular", "{node.name()}" }
                }
                span { class: "font-mono text-xs text-dim", "{size}" }
                span { class: "font-mono text-xs text-dim", "{modified}" }
                span { class: "font-mono text-xs {color_class}", "{format_kind(kind)}" }
            }
        }
    } else {
        rsx! {}
    }
}

#[component]
fn MobileFileRow(
    node: FsNode,
    mut selected_file: Signal<Option<FsNode>>,
    mut expanded: Signal<std::collections::HashSet<String>>,
    mut selected_folder: Signal<Option<String>>,
    mut mobile_view: Signal<MobileView>,
) -> Element {
    if node.is_dir() {
        let dir_path = format!(
            "{}/{}",
            selected_folder().unwrap_or_default().trim_end_matches('/'),
            node.name()
        );
        let dir_path_clone = dir_path.clone();

        rsx! {
            div {
                class: "flex items-center gap-2 py-3 px-3 cursor-pointer bg-background border-b border-[#222630] hover:bg-secondary/50",
                onclick: move |_| {
                    selected_file.set(None);
                    selected_folder.set(Some(dir_path_clone.clone()));
                    expanded.write().insert(dir_path_clone.clone());
                    mobile_view.set(MobileView::Contents);
                },
                span { class: "text-sm shrink-0", "📁" }
                div { class: "flex-1 min-w-0",
                    span { class: "font-mono text-sm truncate text-foreground", "{node.name()}" }
                }
                span { class: "font-mono text-xs text-dim shrink-0", "›" }
            }
        }
    } else if let FsNode::File { size, kind, .. } = &node {
        let file = node.clone();
        let color_class = kind_color(kind);

        rsx! {
            div {
                class: "flex items-center gap-2 py-3 px-3 cursor-pointer bg-background border-b border-[#222630] hover:bg-secondary/50",
                onclick: move |_| {
                    selected_file.set(Some(file.clone()));
                    mobile_view.set(MobileView::Preview);
                },
                span { class: "text-sm shrink-0", "{kind_icon(kind)}" }
                div { class: "flex-1 min-w-0",
                    span { class: "font-mono text-sm truncate text-regular", "{node.name()}" }
                }
                span { class: "font-mono text-xs {color_class} shrink-0", "{size}" }
            }
        }
    } else {
        rsx! {}
    }
}

fn format_kind(kind: &FileKind) -> &'static str {
    match kind {
        FileKind::Video => "video",
        FileKind::Audio => "audio",
        FileKind::Image => "image",
        FileKind::Document => "doc",
        FileKind::Archive => "archive",
        FileKind::Unknown => "file",
    }
}

fn find_node(node: &FsNode, path: &str) -> Option<FsNode> {
    if path == "/root" || path == "/" || path.is_empty() {
        return Some(node.clone());
    }
    let parts: Vec<&str> = path.trim_matches('/').split('/').collect();
    let parts = if parts.first() == Some(&"root") {
        &parts[1..]
    } else {
        &parts
    };
    let mut current = node;
    for part in parts {
        let child = current.children().iter().find(|c| c.name() == *part)?;
        current = child;
    }
    Some(current.clone())
}

fn preview_for(file: &FsNode) -> Element {
    match file {
        FsNode::File { name, size, kind, modified } => {
            let (title, body) = match kind {
                FileKind::Video => (
                    "Video Player".to_string(),
                    rsx! {
                        div { class: "flex flex-col items-center gap-4 py-6",
                            div { class: "w-20 h-20 rounded-full bg-card border-2 border-success-light flex items-center justify-center",
                                span { class: "text-3xl", "▶" }
                            }
                            span { class: "font-mono text-sm text-foreground", "{name}" }
                            div { class: "w-full bg-background rounded h-1.5",
                                div { class: "bg-success-light rounded h-full w-1/3" }
                            }
                            span { class: "font-mono text-xs text-dim", "0:00 / 12:34" }
                        }
                    }
                ),
                FileKind::Audio => (
                    "Audio Player".to_string(),
                    rsx! {
                        div { class: "flex flex-col items-center gap-4 py-6",
                            div { class: "w-20 h-20 rounded-full bg-card border-2 border-primary flex items-center justify-center",
                                span { class: "text-3xl", "♫" }
                            }
                            span { class: "font-mono text-sm text-foreground", "{name}" }
                            div { class: "w-full bg-background rounded h-1.5",
                                div { class: "bg-primary rounded h-full w-2/3" }
                            }
                            span { class: "font-mono text-xs text-dim", "3:42 / 5:18" }
                        }
                    }
                ),
                FileKind::Image => (
                    "Image Preview".to_string(),
                    rsx! {
                        div { class: "flex flex-col items-center gap-4 py-6",
                            div { class: "w-full aspect-video bg-background border border-border rounded flex items-center justify-center",
                                span { class: "text-4xl", "🖼" }
                            }
                            span { class: "font-mono text-sm text-foreground", "{name}" }
                        }
                    }
                ),
                FileKind::Document => (
                    "Document Preview".to_string(),
                    rsx! {
                        div { class: "flex flex-col gap-3 py-4",
                            div { class: "bg-background border border-border rounded p-4",
                                div { class: "h-2 bg-border rounded w-full mb-2" }
                                div { class: "h-2 bg-border rounded w-5/6 mb-2" }
                                div { class: "h-2 bg-border rounded w-4/6 mb-2" }
                                div { class: "h-2 bg-border rounded w-full mb-2" }
                                div { class: "h-2 bg-border rounded w-3/6 mb-2" }
                                div { class: "h-2 bg-border rounded w-5/6 mb-2" }
                                div { class: "h-2 bg-border rounded w-2/6" }
                            }
                            span { class: "font-mono text-sm text-foreground", "{name}" }
                        }
                    }
                ),
                FileKind::Archive => (
                    "Archive Contents".to_string(),
                    rsx! {
                        div { class: "flex flex-col gap-2 py-4",
                            div { class: "font-mono text-xs text-dim mb-1", "Contents:" }
                            div { class: "bg-background border border-border rounded p-3 font-mono text-xs text-muted-foreground",
                                div { "src/" }
                                div { "  main.rs" }
                                div { "  lib.rs" }
                                div { "Cargo.toml" }
                                div { "README.md" }
                            }
                            span { class: "font-mono text-sm text-foreground mt-2", "{name}" }
                        }
                    }
                ),
                FileKind::Unknown => (
                    "File Info".to_string(),
                    rsx! {
                        div { class: "flex flex-col items-center gap-4 py-6",
                            div { class: "w-20 h-20 rounded bg-card border border-border flex items-center justify-center",
                                span { class: "text-3xl", "📄" }
                            }
                            span { class: "font-mono text-sm text-foreground", "{name}" }
                        }
                    }
                ),
            };

            rsx! {
                div { class: "flex flex-col",
                    div { class: "font-mono text-xs text-dim mb-3", "{title} — {name}" }
                    {body}
                    div { class: "mt-4 flex flex-col gap-1.5 border-t border-border pt-3",
                        div { class: "flex items-center gap-3",
                            span { class: "text-xs text-dim min-w-16", "Path" }
                            span { class: "font-mono text-xs text-muted-foreground", "{name}" }
                        }
                        div { class: "flex items-center gap-3",
                            span { class: "text-xs text-dim min-w-16", "Size" }
                            span { class: "font-mono text-xs text-muted-foreground", "{size}" }
                        }
                        div { class: "flex items-center gap-3",
                            span { class: "text-xs text-dim min-w-16", "Modified" }
                            span { class: "font-mono text-xs text-muted-foreground", "{modified}" }
                        }
                        div { class: "flex items-center gap-3",
                            span { class: "text-xs text-dim min-w-16", "Type" }
                            span { class: "font-mono text-xs text-muted-foreground", "{format_kind(kind)}" }
                        }
                    }
                    div { class: "mt-3 flex gap-2",
                        button { class: "flex-1 bg-primary text-background border-none rounded py-1.5 text-xs font-semibold cursor-pointer", "Download" }
                        button { class: "flex-1 bg-transparent text-foreground border border-border rounded py-1.5 text-xs cursor-pointer", "Share Link" }
                        button { class: "bg-transparent text-danger border border-danger/30 rounded py-1.5 px-2.5 text-xs cursor-pointer", "Delete" }
                    }
                }
            }
        }
        _ => rsx! {},
    }
}
