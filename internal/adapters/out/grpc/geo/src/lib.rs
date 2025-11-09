#![allow(clippy::all)]
#[allow(dead_code)]
mod api {
    include!("./gen/geo.rs");
}
mod errors;
pub mod geo_service;
mod mapper;
