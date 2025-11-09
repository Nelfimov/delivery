pub mod consumers;
pub mod shared;
mod messages {
    include!("gen/basket_event.rs");
}
