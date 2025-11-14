mod mapper;
pub mod orders_events_producer;
mod order_event_gen {
    include!("gen/order_event.rs");
}
