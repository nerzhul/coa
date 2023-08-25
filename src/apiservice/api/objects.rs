use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct NamespacedObject {
    pub id: Uuid,
    pub object_type: String,
    pub object_name: String,
    pub namespace: String,
    pub cluster: String,
}

