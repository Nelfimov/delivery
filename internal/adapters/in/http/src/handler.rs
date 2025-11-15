use application::errors::command_errors::CommandError;
use application::errors::query_errors::QueryError;
use application::usecases::CommandHandler;
use application::usecases::EventBus;
use application::usecases::commands::create_courier_command::CreateCourierCommand;
use application::usecases::commands::create_courier_handler::CreateCourierHandler;
use application::usecases::commands::create_order_command::CreateOrderCommand;
use application::usecases::commands::create_order_handler::CreateOrderHandler;
use application::usecases::queries::get_all_couriers_handler::GetAllCouriersHandler;
use application::usecases::queries::get_all_couriers_query::GetAllCouriers;
use application::usecases::queries::get_all_incomplete_orders_handler::GetAllIncompleteOrdersHandler;
use application::usecases::queries::get_all_incomplete_orders_query::GetAllIncompleteOrders;
use async_trait::async_trait;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use axum_extra::extract::Host;
use domain::model::courier::courier_aggregate::CourierName;
use domain::model::courier::courier_aggregate::CourierSpeed;
use openapi::apis::ErrorHandler;
use openapi::apis::default::CreateCourierResponse;
use openapi::apis::default::CreateOrderResponse;
use openapi::apis::default::Default as DefaultApi;
use openapi::apis::default::GetCouriersResponse;
use openapi::apis::default::GetOrdersResponse;
use openapi::models;
use ports::courier_repository_port::CourierRepositoryPort;
use ports::geo_service_port::GeoServicePort;
use ports::order_repository_port::OrderRepositoryPort;
use ports::unit_of_work_port::UnitOfWorkPort;
use std::fmt::Debug;
use std::sync::Arc;
use uuid::Uuid;

use crate::state::AppState;

pub struct ServerImpl<CR, OR, UOW, GS, EB>
where
    CR: CourierRepositoryPort + Send + 'static,
    OR: OrderRepositoryPort + Send + 'static,
    UOW: UnitOfWorkPort + Send + 'static,
    GS: GeoServicePort + Clone + Send + Sync + 'static,
    EB: EventBus + Send + 'static,
{
    state: Arc<AppState<CR, OR, UOW, GS, EB>>,
}

impl<CR, OR, UOW, GS, EB> ServerImpl<CR, OR, UOW, GS, EB>
where
    CR: CourierRepositoryPort + Send + 'static,
    OR: OrderRepositoryPort + Send + 'static,
    UOW: UnitOfWorkPort + Send + 'static,
    GS: GeoServicePort + Clone + Send + Sync + 'static,
    EB: EventBus + Send + 'static,
{
    pub fn new(state: Arc<AppState<CR, OR, UOW, GS, EB>>) -> Self {
        Self { state }
    }

    fn state(&self) -> &AppState<CR, OR, UOW, GS, EB> {
        self.state.as_ref()
    }
}

#[async_trait]
impl<CR, OR, UOW, GS, EB, E> ErrorHandler<E> for ServerImpl<CR, OR, UOW, GS, EB>
where
    CR: CourierRepositoryPort + Send + 'static,
    OR: OrderRepositoryPort + Send + 'static,
    UOW: UnitOfWorkPort + Send + 'static,
    GS: GeoServicePort + Clone + Send + Sync + 'static,
    EB: EventBus + Send + 'static,
    E: Send + Sync + Debug + 'static,
{
}

#[allow(unused_variables)]
#[async_trait]
impl<CR, OR, UOW, GS, E, EB> DefaultApi<E> for ServerImpl<CR, OR, UOW, GS, EB>
where
    CR: CourierRepositoryPort + Send + 'static,
    OR: OrderRepositoryPort + Send + 'static,
    UOW: UnitOfWorkPort + Send + 'static,
    GS: GeoServicePort + Clone + Send + Sync + 'static,
    EB: EventBus + Send + Sync + 'static,
    E: Debug + Send + Sync + 'static,
{
    async fn create_courier(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<models::NewCourier>,
    ) -> Result<CreateCourierResponse, E> {
        let repo = self.state().courier_repo();
        let mut handler = CreateCourierHandler::new(repo);

        let command =
            match CreateCourierCommand::new(CourierName("Bob".to_string()), CourierSpeed(5)) {
                Ok(cmd) => cmd,
                Err(err) => {
                    return Ok(CreateCourierResponse::Status400(models::Error {
                        message: err.to_string(),
                        code: 400,
                    }));
                }
            };

        match handler.execute(command).await {
            Ok(_) => Ok(CreateCourierResponse::Status201),
            Err(err) => {
                let code = match &err {
                    CommandError::ArgumentError(_) => 400,
                    CommandError::ExecutionError(_) => 409,
                };

                Ok(CreateCourierResponse::Status409(models::Error {
                    message: err.to_string(),
                    code,
                }))
            }
        }
    }

    async fn create_order(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<CreateOrderResponse, E> {
        let repo = self.state().order_repo();
        let geo_service = self.state().geo_service();
        let event_bus = self.state().order_event_bus();
        let mut handler = CreateOrderHandler::new(repo, geo_service, event_bus);

        let command = match CreateOrderCommand::new(Uuid::new_v4(), "Unknown street".into(), 5) {
            Ok(cmd) => cmd,
            Err(err) => {
                return Ok(CreateOrderResponse::Status0(models::Error {
                    message: err.to_string(),
                    code: 400,
                }));
            }
        };

        match handler.execute(command).await {
            Ok(_) => Ok(CreateOrderResponse::Status201),
            Err(err) => {
                let code = match &err {
                    CommandError::ArgumentError(_) => 400,
                    CommandError::ExecutionError(_) => 500,
                };

                Ok(CreateOrderResponse::Status0(models::Error {
                    message: err.to_string(),
                    code,
                }))
            }
        }
    }

    async fn get_couriers(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetCouriersResponse, E> {
        let repo = self.state().courier_repo();
        let mut handler = GetAllCouriersHandler::new(repo);

        let command = GetAllCouriers;

        match handler.execute(command).await {
            Ok(couriers) => Ok(GetCouriersResponse::Status200(
                couriers
                    .iter()
                    .map(|c| models::Courier {
                        id: c.id.0,
                        name: c.name.0.clone(),
                        location: models::Location {
                            x: c.location.x() as u32,
                            y: c.location.y() as u32,
                        },
                    })
                    .collect(),
            )),
            Err(err) => {
                let code = match &err {
                    QueryError::ArgumentError(_) => 400,
                    QueryError::ExecutionError(_) => 500,
                };

                Ok(GetCouriersResponse::Status0(models::Error {
                    message: err.to_string(),
                    code,
                }))
            }
        }
    }

    async fn get_orders(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetOrdersResponse, E> {
        let repo = self.state().order_repo();
        let mut handler = GetAllIncompleteOrdersHandler::new(repo);

        match handler.execute(GetAllIncompleteOrders).await {
            Ok(orders) => {
                let orders = orders
                    .into_iter()
                    .map(|order| models::Order {
                        id: order.id().0,
                        location: models::Location {
                            x: order.location().x() as u32,
                            y: order.location().y() as u32,
                        },
                    })
                    .collect();
                Ok(GetOrdersResponse::Status200(orders))
            }
            Err(e) => Ok(GetOrdersResponse::Status0(models::Error {
                message: e.to_string(),
                code: 500,
            })),
        }
    }
}
