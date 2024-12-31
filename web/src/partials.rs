use maud::{html, Markup, DOCTYPE};

use crate::static_files::{corbado_init, lucide, main_css, stimulus_init};

/// Page template, ~100% of the time you want to use this.
pub fn page(page_title: impl AsRef<str>, content: Markup) -> Markup {
    html! {
        (head(page_title.as_ref()))
        body class="app" {
            (content)
            (footer())
        }
        // Icons
        script type="module" src={"/static/" (lucide.name)} {}
        // Monaco loader
        script src="https://cdn.jsdelivr.net/npm/monaco-editor@0.52.2/min/vs/loader.min.js" {}
    }
}

/// <head> template.
///
/// It's better to use `page`, instead of using this directly.
pub fn head(page_title: &str) -> Markup {
    head_custom_content(page_title, html! {})
}

/// <head> template with custom content.
///
/// It's better to use `page_custom_head_content`, instead of using this directly.
pub fn head_custom_content(page_title: &str, head_content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            title { (page_title) }
            link rel="stylesheet" href={"/static/" (main_css.name)};

            // Turbo
            script type="module" src="https://cdn.jsdelivr.net/npm/@hotwired/turbo@8.0.11/+esm" {}

            // Corbado
            link
                rel="stylesheet"
                href="https://unpkg.com/@corbado/web-js@2/dist/bundle/index.css"
                crossorigin="anonymous"
                referrerpolicy="no-referrer";
            script src="https://unpkg.com/@corbado/web-js@2/dist/bundle/index.js" {}
            script type="module" src={"/static/" (corbado_init.name)} {}

            // Stimulus
            script type="module" src={"/static/" (stimulus_init.name)} {}

            // Highlight.js
            link rel="stylesheet" href="https://unpkg.com/@highlightjs/cdn-assets@11.11.1/styles/vs2015.min.css" {}

            (head_content)
        }
    }
}

/// App layout template.
pub fn app_layout(content: Markup, title: &str) -> Markup {
    html! {
        div class="app" {
            nav class="nav" {
                div class="nav__container" {
                    div class="nav__brand" {
                        i data-lucide="book-open" class="nav__icon" {}
                        span class="nav__title" { (title) }
                    }
                }
            }

            main class="main" {
                (content)
            }
        }
    }
}

/// Footer template.
///
/// It's better to use `page`, instead of using this directly.
pub fn footer() -> Markup {
    let current_year = time::OffsetDateTime::now_utc().year();

    html! {
        footer class="footer" {
            p class="footer_content" {
                ("© ") (current_year) (" V")
            }
        }
    }
}
