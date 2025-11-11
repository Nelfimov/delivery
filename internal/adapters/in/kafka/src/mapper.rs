use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasketEventPayload {
    #[serde(default)]
    pub event_id: String,
    #[serde(default)]
    pub event_type: String,
    #[serde(default)]
    pub occurred_at: Option<String>,
    pub basket_id: String,
    pub address: Option<AddressPayload>,
    #[serde(default)]
    pub items: Vec<ItemPayload>,
    #[serde(default)]
    pub delivery_period: Option<DeliveryPeriodPayload>,
    pub volume: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressPayload {
    pub country: String,
    pub city: String,
    pub street: String,
    pub house: String,
    pub apartment: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemPayload {
    pub id: String,
    pub good_id: String,
    pub title: String,
    pub price: f64,
    pub quantity: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryPeriodPayload {
    pub from: i32,
    pub to: i32,
}
