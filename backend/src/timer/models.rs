use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[typeshare]
pub struct Event {}
