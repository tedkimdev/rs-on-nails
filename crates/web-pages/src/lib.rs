mod layout;
pub mod root;
use dioxus::prelude::*;

pub fn render_page(page: Element) -> String {
    let html = dioxus_ssr::render_element(page);
    format!("<!DOCTYPE html><html lang='en'>{}</html>", html)
}
