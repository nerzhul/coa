use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct NamespacedObject {
    pub id: String,
    pub object_type: String,
    pub object_name: String,
    pub namespace: String,
    pub cluster: String,
}

