use domain::model::order::order_events::OrderEvent;
use ports::events_producer_port::Events;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::errors::command_errors::CommandError;
use crate::usecases::EventBus;
use crate::usecases::Handler;

#[derive(Default)]
pub struct OrdersEventBus {
    order_created_subscribers: Vec<Arc<Mutex<dyn Handler + Send + Sync>>>,
    order_completed_subscribers: Vec<Arc<Mutex<dyn Handler + Send + Sync>>>,
}

impl OrdersEventBus {
    pub fn new() -> Self {
        Self {
            order_created_subscribers: Vec::new(),
            order_completed_subscribers: Vec::new(),
        }
    }
}

impl EventBus for OrdersEventBus {
    fn register_order_created(&mut self, subscriber: impl Handler + Send + Sync + 'static) {
        self.order_created_subscribers
            .push(Arc::new(Mutex::new(subscriber)));
    }

    fn register_order_completed(&mut self, subscriber: impl Handler + Send + Sync + 'static) {
        self.order_completed_subscribers
            .push(Arc::new(Mutex::new(subscriber)));
    }

    async fn commit(&mut self, event: Events) -> Result<(), CommandError> {
        match event {
            Events::Order(order_event) => match &order_event {
                OrderEvent::Created { .. } => {
                    for subscriber in &self.order_created_subscribers {
                        let mut handler = subscriber.lock().await;
                        handler.execute(order_event.clone()).await?;
                    }
                }
                OrderEvent::Completed { .. } => {
                    for subscriber in &self.order_completed_subscribers {
                        let mut handler = subscriber.lock().await;
                        handler.execute(order_event.clone()).await?;
                    }
                }
            },
        };

        Ok(())
    }
}
