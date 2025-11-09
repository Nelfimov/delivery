use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

use domain::model::order::order_aggregate::Order;
use domain::model::order::order_aggregate::OrderId;
use ports::errors::RepositoryError;
use ports::order_repository_port::OrderRepositoryPort;

pub struct Shared<T> {
    inner: Arc<Mutex<T>>,
}

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T> Shared<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }

    fn lock(&self) -> Result<MutexGuard<'_, T>, RepositoryError> {
        self.inner
            .lock()
            .map_err(|_| RepositoryError::DatabaseError("repository lock poisoned".into()))
    }
}

impl<OR> OrderRepositoryPort for Shared<OR>
where
    OR: OrderRepositoryPort,
{
    fn add(&mut self, order: &Order) -> Result<(), RepositoryError> {
        let mut repo = self.lock()?;
        repo.add(order)
    }

    fn update(&mut self, order: &Order) -> Result<(), RepositoryError> {
        let mut repo = self.lock()?;
        repo.update(order)
    }

    fn get_by_id(&mut self, id: OrderId) -> Result<Order, RepositoryError> {
        let mut repo = self.lock()?;
        repo.get_by_id(id)
    }

    fn get_any_new(&mut self) -> Result<Order, RepositoryError> {
        let mut repo = self.lock()?;
        repo.get_any_new()
    }

    fn get_all_assigned(&mut self) -> Result<Vec<Order>, RepositoryError> {
        let mut repo = self.lock()?;
        repo.get_all_assigned()
    }

    fn raw(&mut self, query: String) -> Result<Vec<Order>, RepositoryError> {
        let mut repo = self.lock()?;
        repo.raw(query)
    }
}
