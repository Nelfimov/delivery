use std::sync::Arc;
use std::sync::Mutex;

use domain::model::courier::courier_aggregate::Courier;
use domain::model::courier::courier_aggregate::CourierId;
use domain::model::order::order_aggregate::Order;
use domain::model::order::order_aggregate::OrderId;
use ports::courier_repository_port::CourierRepositoryPort;
use ports::courier_repository_port::GetAllCouriersResponse;
use ports::errors::RepositoryError;
use ports::geo_service_port::GeoServicePort;
use ports::order_repository_port::OrderRepositoryPort;
use ports::unit_of_work_port::UnitOfWorkPort;

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

    fn with_lock<R, F>(&mut self, context: &'static str, op: F) -> Result<R, RepositoryError>
    where
        F: FnOnce(&mut T) -> Result<R, RepositoryError>,
    {
        let mut guard = self
            .inner
            .lock()
            .map_err(|_| RepositoryError::DatabaseError(format!("{context} lock poisoned")))?;
        op(&mut guard)
    }
}

impl<CR> CourierRepositoryPort for Shared<CR>
where
    CR: CourierRepositoryPort,
{
    fn add(&mut self, courier: Courier) -> Result<(), RepositoryError> {
        self.with_lock("courier repository", |inner| inner.add(courier))
    }

    fn update(&mut self, courier: Courier) -> Result<(), RepositoryError> {
        self.with_lock("courier repository", |inner| inner.update(courier))
    }

    fn get_by_id(&mut self, id: CourierId) -> Result<Courier, RepositoryError> {
        self.with_lock("courier repository", |inner| inner.get_by_id(id))
    }

    fn get_all_free(&mut self) -> Result<Vec<Courier>, RepositoryError> {
        self.with_lock("courier repository", |inner| inner.get_all_free())
    }

    fn get_all_couriers(&mut self) -> Result<Vec<GetAllCouriersResponse>, RepositoryError> {
        self.with_lock("courier repository", |inner| inner.get_all_couriers())
    }
}

impl<OR> OrderRepositoryPort for Shared<OR>
where
    OR: OrderRepositoryPort,
{
    fn add(&mut self, order: &Order) -> Result<(), RepositoryError> {
        self.with_lock("order repository", |inner| inner.add(order))
    }

    fn update(&mut self, order: &Order) -> Result<(), RepositoryError> {
        self.with_lock("order repository", |inner| inner.update(order))
    }

    fn get_by_id(&mut self, id: OrderId) -> Result<Order, RepositoryError> {
        self.with_lock("order repository", |inner| inner.get_by_id(id))
    }

    fn get_any_new(&mut self) -> Result<Order, RepositoryError> {
        self.with_lock("order repository", |inner| inner.get_any_new())
    }

    fn get_all_assigned(&mut self) -> Result<Vec<Order>, RepositoryError> {
        self.with_lock("order repository", |inner| inner.get_all_assigned())
    }

    fn raw(&mut self, query: String) -> Result<Vec<Order>, RepositoryError> {
        self.with_lock("order repository", |inner| inner.raw(query))
    }
}

pub struct AppState<CR, OR, UOW, GS>
where
    CR: CourierRepositoryPort + Send + 'static,
    OR: OrderRepositoryPort + Send + 'static,
    UOW: UnitOfWorkPort + Send + 'static,
    GS: GeoServicePort + Clone + Send + Sync + 'static,
{
    courier_repo: Shared<CR>,
    order_repo: Shared<OR>,
    uow: Arc<Mutex<UOW>>,
    geo_service: GS,
}

impl<CR, OR, UOW, GS> AppState<CR, OR, UOW, GS>
where
    CR: CourierRepositoryPort + Send + 'static,
    OR: OrderRepositoryPort + Send + 'static,
    UOW: UnitOfWorkPort + Send + 'static,
    GS: GeoServicePort + Clone + Send + Sync + 'static,
{
    pub fn new(courier_repo: CR, order_repo: OR, uow: UOW, geo_service: GS) -> Self {
        Self {
            courier_repo: Shared::new(courier_repo),
            order_repo: Shared::new(order_repo),
            uow: Arc::new(Mutex::new(uow)),
            geo_service,
        }
    }

    pub fn courier_repo(&self) -> Shared<CR> {
        self.courier_repo.clone()
    }

    pub fn order_repo(&self) -> Shared<OR> {
        self.order_repo.clone()
    }

    pub fn unit_of_work(&self) -> Arc<Mutex<UOW>> {
        Arc::clone(&self.uow)
    }

    pub fn geo_service(&self) -> GS {
        self.geo_service.clone()
    }
}
