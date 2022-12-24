use dioxus::prelude::*;

fn main() {
    dioxus_tui::launch(app);
}

fn app(cx: Scope) -> Element {
    let enabled = use_state(&cx, || false);  

    cx.render(rsx! {
        div {
            width: "90%",
            height: "85%",
            onclick: move |_| {
                println!("clicked");
                enabled.set(!enabled.get());
            },
            vec![0,0,0,0,0,0,0,0].iter().enumerate().map(move |(i, _)| {
                println!("{}", i);                       
                rsx! {
                    div {
                        key: "{i}",
                        p {
                            "N: {i} "
                        }
                    }
                }
            })
        }
    })
}