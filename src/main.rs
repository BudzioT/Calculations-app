use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/company")]
    Company {},
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

#[derive(Debug, Clone, Default)]
struct Service {
    name: String,
    price_per_day: f32,
    tax: f32,
    travel_price: f32,
    travel_limit: f32,
    travel_additional_price: f32,
    employees_limit: i32,
    additional_days: i32,
}

impl Service {
    fn new(name: String, price_per_day: f32, tax: f32, travel_price: f32, travel_limit: f32,
           travel_additional_price: f32, employees_limit: i32, additional_days: i32) -> Self {
        Self {
            name,
            price_per_day,
            tax,
            travel_price,
            travel_limit,
            travel_additional_price,
            employees_limit,
            additional_days,
        }
    }

    fn calculate_price(&self, days: i32, distance: f32, employees: i32) -> f32 {
        let mut total_price = self.price_per_day * days as f32;
        total_price += self.tax;
        if distance > self.travel_limit {
            total_price += self.travel_additional_price * (distance - self.travel_limit);
        }
        if employees > self.employees_limit {
            total_price += self.additional_days as f32 * self.price_per_day;
        }
        total_price
    }
}

/// Company page
#[component]
fn Company() -> Element {
    let mut services = use_signal(|| Vec::<Service>::new());
    let mut service = use_signal(|| Service::new(String::new(), 0.0, 0.0, 0.0, 0.0, 0.0, 0, 0));
    let mut selected_service = use_signal(|| None::<Service>);
    let mut days = use_signal(|| 0);
    let mut distance = use_signal(|| 0.0);
    let mut employees = use_signal(|| 0);
    let mut total_price = use_signal(|| 0.0);
    let mut total_price_no_tax = use_signal(|| 0.0);
    let mut tax = use_signal(|| 0.0);

    let add_service = move |_| {
        if !service().name.is_empty() {
            services.push(service());
            service.set(Service::default());
        }
    };
    let calculate_price = move |_| {
        if let Some(selected_service) = services().first() {
            total_price.set(selected_service.calculate_price(days(), distance(), employees()));
            tax.set(total_price() * 0.23 + service().tax);
            total_price_no_tax.set(total_price() - tax());
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
                    {
                        services().iter().enumerate().map(|(index, service)| rsx! {
                            li {
                                class: "mb-2",
                                "{service.name} - {service.price_per_day} zł/dzień"
                                button {
                                    class: "ml-2 text-red-500",
                                    onclick: move |_| { services.remove(index); },
                                    "Usuń"
                                }
                            }
                        })
                    }
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
                        value: service().name.clone(),
                        oninput: move |e| {
                            let mut current_service = service();
                            current_service.name = e.value();
                            service.set(current_service);
                        },
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
                        value: service().price_per_day.to_string(),
                        oninput: move |e| {
                            if let Ok(price) = e.value().parse::<f32>() {
                                let mut current_service = service();
                                current_service.price_per_day = price;
                                service.set(current_service);
                            }
                        },
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
                        value: service().tax.to_string(),
                        oninput: move |e| {
                            if let Ok(tax) = e.value().parse::<f32>() {
                                let mut current_service = service();
                                current_service.tax = tax;
                                service.set(current_service);
                            }
                        },
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
                        value: service().travel_price.to_string(),
                        oninput: move |e| {
                            if let Ok(travel_price) = e.value().parse::<f32>() {
                                let mut current_service = service();
                                current_service.travel_price = travel_price;
                                service.set(current_service);
                            }
                        },
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
                        value: service().travel_limit.to_string(),
                        oninput: move |e| {
                            if let Ok(travel_limit) = e.value().parse::<f32>() {
                                let mut current_service = service();
                                current_service.travel_limit = travel_limit;
                                service.set(current_service);
                            }
                        },
                        placeholder: "Próg (km)"
                    }
                    h4 { "Dodatkowa opłata" }
                    input {
                        class: "text-sm text-black px-1",
                        type: "number",
                        value: service().travel_additional_price.to_string(),
                        oninput: move |e| {
                            if let Ok(additional_price) = e.value().parse::<f32>() {
                                let mut current_service = service();
                                current_service.travel_additional_price = additional_price;
                                service.set(current_service);
                            }
                        },
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
                        value: service().employees_limit.to_string(),
                        oninput: move |e| {
                            if let Ok(employees_limit) = e.value().parse::<i32>() {
                                let mut current_service = service();
                                current_service.employees_limit = employees_limit;
                                service.set(current_service);
                            }
                        },
                        placeholder: "Próg (liczba osób)"
                    }
                    h4 {
                        class: "text-lg",
                        "Ilość dni usług"
                    }
                    input {
                        class: "text-sm text-black px-1",
                        type: "number",
                        value: service().additional_days.to_string(),
                        oninput: move |e| {
                            if let Ok(additional_days) = e.value().parse::<i32>() {
                                let mut current_service = service();
                                current_service.additional_days = additional_days;
                                service.set(current_service);
                            }
                        },
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

        div {
            div {
                class: "flex flex flex-col",

                h1 { class: "text-2xl text-center mt-10 mb-5 font-bold","Oszacuj cenę" }
                div {
                    class: "mb-5 text-lg gap-x-10 flex flex-wrap justify-center",
                    div {
                        h3 {
                            class: "font-bold",
                            "Wybierz usługę"
                        }
                        select {
                            class: "text-sm text-black px-1",
                            onchange: move |e| {
                                let selected_index = e.value().parse::<usize>().unwrap_or(0);
                                if let Some(service) = services().get(selected_index) {
                                    selected_service.set(Some(service.clone()));
                                }
                            },
                            option { value: "", "Wybierz usługę" }
                            {
                                services().iter().enumerate().map(|(index, service)| rsx! {
                                    option { value: "{index}", "{service.name}" }
                                })
                            }
                        }
                    }

                    div {
                        h3 {
                            class: "font-bold",
                            "Ilość dni"
                        }
                        input {
                            class: "text-sm text-black px-1",
                            type: "number",
                            value: days.to_string(),
                            oninput: move |e| {
                                if let Ok(d) = e.value().parse::<i32>() {
                                    days.set(d);
                                }
                            },
                            placeholder: "Ilość dni"
                        }
                    }

                    div {
                    class: "mb-5 text-lg",
                    h3 {
                        class: "font-bold",
                        "Odległość (km)"
                    }
                    input {
                        class: "text-sm text-black px-1",
                        type: "number",
                        value: distance.to_string(),
                        oninput: move |e| {
                            if let Ok(d) = e.value().parse::<f32>() {
                                distance.set(d);
                            }
                        },
                        placeholder: "Odległość (km)"
                        }
                    }

                    div {
                    class: "mb-5 text-lg",
                    h3 {
                        class: "font-bold",
                        "Liczba pracowników"
                    }
                    input {
                        class: "text-sm text-black px-1",
                        type: "number",
                        value: employees.to_string(),
                        oninput: move |e| {
                            if let Ok(e) = e.value().parse::<i32>() {
                                employees.set(e);
                            }
                        },
                        placeholder: "Liczba pracowników"
                        }
                    }
                }

                button {
                    class: "bg-gray-500 p-2 rounded-xl text-sm self-center w-40",
                    onclick: calculate_price,
                    "Oblicz cenę"
                }
                h2 {
                    class: "text-xl mt-5 text-center",
                    "Całkowita cena netto: {total_price_no_tax} zł"
                }
                h2 {
                    class: "text-xl mt-5 text-center",
                    "Całkowity VAT i podatki: {tax} zł"
                }
                h2 {
                    class: "text-xl mt-5 mb-10 text-center",
                    "Całkowita cena brutto: {total_price} zł"
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
            class: "text-3xl mt-20 text-center",
            h1 { "Program do szacowania ceny usług" }
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
        }

        Outlet::<Route> {}
    }
}
