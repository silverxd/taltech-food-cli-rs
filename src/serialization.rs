use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

pub type Diners = Vec<DinerWrapper>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DinerWrapper {
    pub diner: Diner,
    pub meal_images: Vec<Value>,
    pub meals: Vec<Meal>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Diner {
    pub id: i64,
    pub name: String,
    pub address: Option<String>,
    pub county: String,
    pub district: Option<String>,
    pub homepage: String,
    pub menu: Option<String>,
    pub contact_email: Option<String>,
    pub phone: Option<String>,
    pub coordinates: Option<Coordinates>,
    pub day_votes: i64,
    pub up_votes: i64,
    pub photos: Vec<Photo>,
    pub settings: Option<Settings>,
    pub created: String,
    pub status: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub awards: Vec<Value>,
    pub parking_type: Option<String>,
    pub parking_info: Option<String>,
    pub postal_code: Option<i64>,
    pub offer_week_days: Vec<String>,
    pub auto_repeat_until: Option<String>,
    pub active: bool,
    pub has_food_carrier_bolt: bool,
    pub has_food_carrier_fudy: bool,
    pub has_food_carrier_self_provided: bool,
    pub has_food_carrier_wolt: bool,
    pub name_as_url: String,
    pub one_photo: Option<OnePhoto>,
    pub public: bool,
    pub unaltered_name: String,
    pub verified: bool,
    pub year_votes: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coordinates {
    pub h: String,
    pub l: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Photo {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub cleaning_facebook_posts: bool,
    pub diner_id: i64,
    pub facebook_default_header: String,
    pub facebook_layout: Option<FacebookLayout>,
    pub files_premade_day_designs: Vec<String>,
    pub offers_end: Option<String>,
    pub offers_start: Option<String>,
    pub web_layout: Option<WebLayout>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FacebookLayout {
    pub header: String,
    pub footer: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebLayout {
    pub header: String,
    pub footer: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnePhoto {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meal {
    pub amount: Value,
    pub description: String,
    pub description_eng: Option<String>,
    pub price: Option<String>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub tags: Vec<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

/// Deserializes a null value as the default instantiation of itself instead.
///
/// Useful when the API might return null for empty arrays.
///
/// Taken from https://github.com/MathiasPius/hrobot-rs/blob/a0525777eeb2be1368de3ff5feb8176c0d1992b4/src/conversion.rs
pub fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}
