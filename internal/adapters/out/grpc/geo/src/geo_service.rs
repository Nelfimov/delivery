use async_trait::async_trait;
use domain::model::kernel::location::Location;
use ports::errors::GeoClientError;
use ports::geo_service_port::GeoServicePort;
use tonic::transport::Channel;

use crate::api::GetGeolocationRequest;
use crate::api::geo_client::GeoClient;
use crate::errors::GeoClientGrpcError;

#[derive(Clone)]
pub struct GeoService {
    client: GeoClient<Channel>,
}
impl GeoService {
    pub async fn new(address: String) -> Result<Self, GeoClientError> {
        let client = GeoClient::connect(address)
            .await
            .map_err(GeoClientGrpcError::from)
            .map_err(GeoClientError::from)?;

        Ok(Self { client })
    }
}

#[async_trait]
impl GeoServicePort for GeoService {
    async fn get_location(&mut self, address: String) -> Result<Location, GeoClientError> {
        let result = self
            .client
            .get_geolocation(GetGeolocationRequest { street: address })
            .await
            .map_err(GeoClientGrpcError::from)
            .map_err(GeoClientError::from)?;

        match result.into_inner().location {
            Some(location) => Ok(Location::try_from(location)?),
            None => Err(GeoClientError::ExecutionError(
                "no location for such address".to_string(),
            )),
        }
    }
}
