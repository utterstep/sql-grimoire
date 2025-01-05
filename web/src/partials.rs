use maud::{html, Markup, DOCTYPE};

use crate::static_files;

/// Page template, ~100% of the time you want to use this.
pub fn page(page_title: impl AsRef<str>, content: Markup) -> Markup {
    html! {
        (head(page_title.as_ref()))
        body class="app" {
            (content)
            (footer())
        }
        // Icons
        script type="module" src={"/static/" (static_files::lucide.name)} {}
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
            link rel="stylesheet" href={"/static/" (static_files::reset_css.name)};
            link rel="stylesheet" href={"/static/" (static_files::main_css.name)};

            // Turbo
            script type="module" src="https://cdn.jsdelivr.net/npm/@hotwired/turbo@8.0.11/+esm" {}

            // Corbado
            link
                rel="stylesheet"
                href="https://unpkg.com/@corbado/web-js@2/dist/bundle/index.css"
                crossorigin="anonymous"
                referrerpolicy="no-referrer";
            script src="https://unpkg.com/@corbado/web-js@2/dist/bundle/index.js" {}
            script type="module" src={"/static/" (static_files::corbado_init.name)} {}

            // Stimulus
            script type="module" src={"/static/" (static_files::stimulus_init.name)} {}

            // Highlight.js
            link rel="stylesheet" href="https://unpkg.com/@highlightjs/cdn-assets@11.11.1/styles/vs2015.min.css" {}

            // Monaco loader
            script type="module" src={"/static/" (static_files::monaco_loader.name)} {}

            (head_content)
        }
    }
}

/// App layout template.
pub fn app_layout(content: Markup, title: &str, is_admin: bool) -> Markup {
    html! {
        div class="app" {
            nav class="nav" {
                div class="nav__container" {
                    a href="/" class="nav__brand" {
                        i data-lucide="book-open" class="nav__icon" {}
                        span class="nav__title" { (title) }
                    }
                    div class="nav__menu" {
                        @if is_admin {
                            a href="/admin/exercise/schemas/" class="nav__link" {
                                i data-lucide="database" class="nav__link-icon" {}
                                span { "Schemas" }
                            }
                        }
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
            div class="footer__container" {
                div class="footer__content" {
                    p class="footer__text" {
                        "Made with "
                        i data-lucide="heart" class="footer__icon footer__icon--heart" {}
                        " by V, " (current_year)
                    }
                }
            }
        }
    }
}
