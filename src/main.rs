#![allow(unused_imports)]
#![allow(unused)]

mod app;
mod pages;
mod router;
mod components;
pub mod data;

//use wasm_logger;

use crate::app::App;

fn main() {
  dotenvy::dotenv().ok();
  wasm_logger::init(wasm_logger::Config::default());
  yew::Renderer::<App>::new().render();
}