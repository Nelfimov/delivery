use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum CreateCourierResponse {
    /// Успешный ответ
    Status201
    ,
    /// Ошибка валидации
    Status400
    (models::Error)
    ,
    /// Ошибка выполнения бизнес логики
    Status409
    (models::Error)
    ,
    /// Ошибка
    Status0
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum CreateOrderResponse {
    /// Успешный ответ
    Status201
    ,
    /// Ошибка
    Status0
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetCouriersResponse {
    /// Успешный ответ
    Status200
    (Vec<models::Courier>)
    ,
    /// Ошибка
    Status0
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetOrdersResponse {
    /// Успешный ответ
    Status200
    (Vec<models::Order>)
    ,
    /// Ошибка
    Status0
    (models::Error)
}




/// Default
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Default<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// Добавить курьера.
    ///
    /// CreateCourier - POST /api/v1/couriers
    async fn create_courier(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
            body: &Option<models::NewCourier>,
    ) -> Result<CreateCourierResponse, E>;

    /// Создать заказ.
    ///
    /// CreateOrder - POST /api/v1/orders
    async fn create_order(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
    ) -> Result<CreateOrderResponse, E>;

    /// Получить всех курьеров.
    ///
    /// GetCouriers - GET /api/v1/couriers
    async fn get_couriers(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
    ) -> Result<GetCouriersResponse, E>;

    /// Получить все незавершенные заказы.
    ///
    /// GetOrders - GET /api/v1/orders/active
    async fn get_orders(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
    ) -> Result<GetOrdersResponse, E>;
}
