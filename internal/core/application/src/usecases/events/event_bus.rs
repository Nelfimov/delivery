use async_trait::async_trait;
use domain::model::order::order_events::OrderEvent;
use ports::events_producer_port::Events;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::errors::command_errors::CommandError;
use crate::usecases::Handler;

#[async_trait]
pub trait EventBus: Clone + Send + Sync {
    fn register_order_created(&mut self, subscriber: impl Handler + 'static);
    fn register_order_completed(&mut self, subscriber: impl Handler + 'static);
    async fn commit(&self, e: Events) -> Result<(), CommandError>;
}

#[derive(Default, Clone)]
pub struct EventBusImpl {
    order_created_subscribers: Vec<Arc<Mutex<dyn Handler + Send + Sync>>>,
    order_completed_subscribers: Vec<Arc<Mutex<dyn Handler + Send + Sync>>>,
}

impl EventBusImpl {
    pub fn new() -> Self {
        Self {
            order_created_subscribers: Vec::new(),
            order_completed_subscribers: Vec::new(),
        }
    }
}

#[async_trait]
impl EventBus for EventBusImpl {
    fn register_order_created(&mut self, subscriber: impl Handler + 'static) {
        self.order_created_subscribers
            .push(Arc::new(Mutex::new(subscriber)));
    }

    fn register_order_completed(&mut self, subscriber: impl Handler + 'static) {
        self.order_completed_subscribers
            .push(Arc::new(Mutex::new(subscriber)));
    }

    async fn commit(&self, event: Events) -> Result<(), CommandError> {
        match event {
            Events::Order(order_event) => match &order_event {
                OrderEvent::Created { .. } => {
                    for subscriber in &self.order_created_subscribers {
                        let mut s = subscriber.lock().await;
                        s.execute(order_event.clone()).await?;
                    }
                }
                OrderEvent::Completed { .. } => {
                    for subscriber in &self.order_completed_subscribers {
                        let mut s = subscriber.lock().await;
                        s.execute(order_event.clone()).await?;
                    }
                }
            },
        };

        Ok(())
    }
}
