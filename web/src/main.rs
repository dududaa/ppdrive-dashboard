mod components;
mod pages;

use dioxus::prelude::*;
use pages::{
    AppClientsPage, AuthPage, DashboardLayout, DashboardOverviewPage, FileExplorer, SettingsPage,
};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[route("/")]
    Auth {},
    #[layout(DashboardLayout)]
        #[nest("/dashboard")]
        #[route("")]
        Dashboard {},
        #[route("/explorer")]
        Explorer {},
        #[route("/clients")]
        Clients {},
        #[route("/settings")]
        Settings {},
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
fn Auth() -> Element {
    rsx! { AuthPage {} }
}

#[component]
fn Dashboard() -> Element {
    rsx! { DashboardOverviewPage {} }
}

#[component]
fn Explorer() -> Element {
    rsx! { FileExplorer {} }
}

#[component]
fn Clients() -> Element {
    rsx! { AppClientsPage {} }
}

#[component]
fn Settings() -> Element {
    rsx! { SettingsPage {} }
}
