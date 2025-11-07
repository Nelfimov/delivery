use std::error::Error;
use tonic::transport::Channel;

use super::api::geo_client::GeoClient;

pub async fn connect_geo(address: String) -> Result<GeoClient<Channel>, Box<dyn Error>> {
    let client = GeoClient::connect(address).await?;

    Ok(client)
}
