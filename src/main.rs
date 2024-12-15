use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/company")]
    Company {},
    #[route("/client")]
    Client {}
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Stylesheet {
            href: asset!("/assets/tailwind.css")
        }
        Router::<Route> {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            h1 {
                class: "text-3xl pb-10",
                "Dane firmy"
            }
            div {
                id: "inputs",
                class: "",
                div {
                    class: "pb-5",
                    h3 { "Nazwa usługi" }
                    input {
                        class: "",
                        placeholder: "Nazwa usługi"
                    }
                }
                div {
                    class: "pb-5",
                    h3 { "Cena netto za dzień usługi (zł)"}
                    input {
                        placeholder: "Cena netto"
                    }
                }
                div {
                    class: "pb-5",
                    h3 { "Kwota podatku (zł)" }
                    input {
                        placeholder: "Kwota podatku"
                    }
                }
                div {
                    class: "pb-5",
                    h3 { "Koszt dojazdu (zł/km)" }
                    input {
                        placeholder: "Cena dojazdu"
                    }
                }

                div {
                    class: "pb-5",
                    h3 { "Koszty logistyki ze względu na odległość" }
                    h4 { "Próg odległości" }
                    input {
                        placeholder: "Próg (km)"
                    }
                    h4 { "Dodatkowa opłata" }
                    input {
                        placeholder: "Dodatkowa opłata"
                    }
                }

                div { padding-bottom: 5,
                    class: "",
                    h3 { "Ilość dni usług w zalezności od progu ilości pracowników" }
                    h4 { "Próg pracowników" }
                    input {
                        placeholder: "Próg (liczba osób)"
                    }
                    h4 { "Ilość dni usług" }
                    input {
                        placeholder: "Ilość dni"
                    }
                }

                button {
                    class: ""
                    "Dodaj usługę"
                }
            }
        }
    }
}

/// Company page
#[component]
fn Company() -> Element {
    rsx! {
        Hero {}
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        div {
            h1 { "Program do szacowania ceny usług" }
        }
    }
}

/// Client page
fn Client() -> Element {
    rsx! {
        div {
            h1 { "Oszacuj cenę" }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Start"
            }
            Link {
                to: Route::Company {},
                "Firma"
            }
            Link {
                to: Route::Client {},
                "Klient"
            }
        }

        Outlet::<Route> {}
    }
}
