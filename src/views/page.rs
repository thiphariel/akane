use maud::{html, Markup, DOCTYPE};

fn header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                link rel="stylesheet" href="/static/bundle.css";
                link rel="icon" href="/favicon.ico";
                title { (page_title) }
            }
        }

    }
}

fn footer() -> Markup {
    html! {
        footer .flex.justify-center.my-2 {
            "So fucking 🦀"
        }
    }
}

pub fn page(title: &str, body: Markup) -> Markup {
    html! {
        (header(title))
        body .h-screen.flex.flex-col {
            section .flex-1.m-4 {
                h1 { (title) }
                (body)
            }
        }
        (footer())
    }
}
