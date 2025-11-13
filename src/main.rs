use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut todos = use_signal(|| Vec::<(u32, String)>::new());
    let mut input = use_signal(|| String::new());
    let mut next_id = use_signal(|| 0);

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div {
            class: "todo-container",
            h1 { class: "todo-title", "Todo List" }
            input {
                class: "todo-input",
                r#type: "text",
                placeholder: "Add a new todo",
                value: "{input}",
                oninput: move |e| *input.write() = e.value(),
                onkeydown: move |e| {
                    if e.key() == Key::Enter {
                        let id = *next_id.read();
                        todos.write().push((id, input()));
                        *next_id.write() += 1;
                        input.write().clear();
                    }
                }
            }
            button {
                class: "add-button",
                onclick: move |_| {
                    let id = *next_id.read();
                    todos.write().push((id, input()));
                    *next_id.write() += 1;
                    input.write().clear();
                },
                "Add Todo"
            }
            ul {
                class: "todo-list",
                for (id, todo) in todos.read().iter().cloned() {
                    li {
                        class: "todo-item",
                        "{todo}"
                        button {
                            class: "remove-button",
                            onclick: move |_| {
                                todos.write().retain(|(i, _)| *i != id);
                            },
                            "Remove"
                        }
                    }
                }
            }
        }
    }
}
