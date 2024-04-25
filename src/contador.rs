use sycamore::prelude::*;

#[component]
fn Counter<G: Html>(cx: Scope) -> View<G> {
    let count = create_signal(cx, 0);
    view! { cx,
        button(on:click=move |_| count.set(*count.get() + 1)) {
            "Incrementar"
        }
        p {
            "Contagem: "
            (count.get())
        }
    }
}
