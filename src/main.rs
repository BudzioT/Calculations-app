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

/// Company page
#[component]
fn Company() -> Element {
    let mut services = use_signal(|| Vec::<String>::new());
    let mut service_name = use_signal(|| String::new());

    let add_service = move |_| {
        if !service_name().is_empty() {
            services.push(service_name());
            service_name.set(String::new());
        }
    };

    rsx! {
        div {
            id: "hero",
            h1 {
                class: "text-3xl mb-10",
                "Dane firmy"
            }

            div {
                h1 {
                    class: "text-xl font-bold",
                    "Aktualne usługi"
                }
                ul {
                    services.iter().map(|service| rsx! {
                        li { "{service}"}
                    })
                }
            }

            div {
                id: "inputs",
                class: "",
                div {
                    class: "mb-5 text-lg",
                    h3 {
                        class: "font-bold",
                        "Nazwa usługi"
                    }
                    input {
                        class: "text-sm text-black px-1",
                        type: "text",
                        oninput: move |e| service_name.set(e.value()),
                        placeholder: "Nazwa usługi"
                    }
                }
                div {
                    class: "mb-5 text-lg",
                    h3 {
                        class: "font-bold",
                        "Cena netto za dzień usługi (zł)"
                    }
                    input {
                        class: "text-sm text-black px-1",
                        type: "number",
                        placeholder: "Cena netto (zł)"
                    }
                }
                div {
                    class: "mb-5 text-lg",
                    h3 {
                        class: "font-bold",
                        "Kwota podatku"
                    }
                    input {
                        class: "text-sm text-black px-1",
                        type: "number",
                        placeholder: "Kwota podatku (zł)"
                    }
                }
                div {
                    class: "mb-5 text-lg",
                    h3 {
                        class: "font-bold",
                        "Koszt dojazdu"
                    }
                    input {
                        class: "text-sm text-black px-1",
                        type: "number",
                        placeholder: "Cena dojazdu (zł/km)"
                    }
                }

                div {
                    class: "mb-5 text-lg",
                    h3 {
                        class: "font-bold",
                        "Koszty logistyki ze względu na odległość"
                    }
                    h4 { "Próg odległości" }
                    input {
                        class: "text-sm text-black px-1",
                        type: "number",
                        placeholder: "Próg (km)"
                    }
                    h4 { "Dodatkowa opłata" }
                    input {
                        class: "text-sm text-black px-1",
                        type: "number",
                        placeholder: "Dodatkowa opłata"
                    }
                }

                div {
                    class: "mb-10 text-xl",
                    h3 {
                        class: "font-bold",
                        "Ilość dni usług w zalezności od progu ilości pracowników"
                    }
                    h4 {
                        class: "text-lg",
                        "Próg pracowników"
                    }
                    input {
                        class: "text-sm text-black px-1",
                        type: "number",
                        placeholder: "Próg (liczba osób)"
                    }
                    h4 {
                        class: "text-lg",
                        "Ilość dni usług"
                    }
                    input {
                        class: "text-sm text-black px-1",
                        type: "number",
                        placeholder: "Ilość dni"
                    }
                }

                button {
                    class: "bg-gray-500 p-2 rounded-xl text-sm",
                    onclick: add_service,
                    "Dodaj usługę"
                }
            }
        }
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
#[component]
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
