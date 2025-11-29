use std::error::Error;
use std::fmt;

use domain::model::kernel::message::Message;
use domain::model::order::order_events::OrderEvent;

pub enum Events {
    Order(OrderEvent),
}

impl std::fmt::Display for Events {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = &self::Events::to_string(self);
        write!(f, "{}", str)
    }
}

impl From<OrderEvent> for Events {
    fn from(v: OrderEvent) -> Self {
        Events::Order(v)
    }
}

impl TryFrom<&Message> for Events {
    type Error = Box<dyn Error>;

    fn try_from(v: &Message) -> Result<Self, Self::Error> {
        let event = match v.name.as_str() {
            "created" => serde_json::from_str(&v.payload)?,
            "completed" => serde_json::from_str(&v.payload)?,
            _ => {
                return Err(Box::new(UnsupportedEventName(v.name.clone())));
            }
        };

        Ok(Self::Order(event))
    }
}

#[derive(Debug)]
struct UnsupportedEventName(String);

impl fmt::Display for UnsupportedEventName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unsupported event name: {}", self.0)
    }
}

impl Error for UnsupportedEventName {}

pub trait EventsProducerPort {
    fn publish(&self, e: Events);
}
