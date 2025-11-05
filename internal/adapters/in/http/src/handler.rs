use async_trait::async_trait;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use axum_extra::extract::Host;
use openapi::apis::ErrorHandler;
use openapi::apis::default::CreateCourierResponse;
use openapi::apis::default::CreateOrderResponse;
use openapi::apis::default::Default;
use openapi::apis::default::GetCouriersResponse;
use openapi::apis::default::GetOrdersResponse;
use openapi::models;
use std::fmt::Debug;

pub struct ServerImpl;

#[async_trait]
impl<E> ErrorHandler<E> for ServerImpl where E: Send + Sync + Debug + 'static {}

#[allow(unused_variables)]
#[async_trait]
impl<E> Default<E> for ServerImpl
where
    E: Debug + Send + Sync + 'static,
{
    async fn create_courier(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<models::NewCourier>,
    ) -> Result<CreateCourierResponse, E> {
        Ok(CreateCourierResponse::Status201)
    }

    async fn create_order(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<CreateOrderResponse, E> {
        Ok(CreateOrderResponse::Status201)
    }

    async fn get_couriers(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetCouriersResponse, E> {
        todo!()
    }

    async fn get_orders(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetOrdersResponse, E> {
        todo!()
    }
}
