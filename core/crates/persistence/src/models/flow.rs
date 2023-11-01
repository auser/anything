use crate::models::model_types::default_bool;
use anything_graph::Flow;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteRow, Column, FromRow, Row};

pub type FlowId = String;
pub type FlowVersionId = String;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StoredFlow {
    pub flow_id: String,
    pub flow_name: String,
    pub latest_version_id: FlowVersionId,
    pub active: bool,
    pub updated_at: DateTime<Utc>,
    pub versions: Vec<FlowVersion>,
}

// SQLITE handling
impl FromRow<'_, SqliteRow> for StoredFlow {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
        let flow_id = row.get::<'_, String, &str>("flow_id");
        let flow_name = row.get::<'_, String, &str>("flow_name");
        let latest_version_id = row.get::<'_, String, &str>("latest_version_id");
        let active = row.get::<'_, bool, &str>("active");
        let updated_at = row.get::<'_, DateTime<Utc>, &str>("updated_at");

        let mut versions = Vec::default();
        let column_names = row
            .columns()
            .iter()
            .map(|c| c.name().to_owned())
            .collect::<Vec<String>>();

        if column_names.contains(&"fv_flow_definition".to_string()) {
            let flow_def = row.get::<'_, String, &str>("fv_flow_definition");
            let flow_version = FlowVersion {
                flow_id: flow_id.clone(),
                flow_version: row.get::<'_, String, &str>("fv_flow_version"),
                description: row.get::<'_, Option<String>, &str>("fv_description"),
                flow_definition: serde_json::from_str(&flow_def).unwrap(),
                checksum: row.get::<'_, String, &str>("fv_checksum"),
                published: row.get::<'_, bool, &str>("fv_published"),
                updated_at: row.get::<'_, Option<DateTime<Utc>>, &str>("fv_updated_at"),
            };
            versions.push(flow_version);
        }

        // let flow = anything_coordinator::Manager::default().get_flow(&flow_name).unwrap();

        Ok(Self {
            flow_id,
            flow_name,
            latest_version_id,
            active,
            updated_at,
            versions,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct CreateFlow {
    pub name: String,
    #[serde(default = "default_bool::<false>")]
    pub active: bool,
    pub version: Option<String>,
}

#[derive(FromRow, Debug, Serialize, Deserialize, Clone, Default)]
pub struct FlowVersion {
    pub flow_id: FlowId,
    pub flow_version: String,
    pub description: Option<String>,
    pub flow_definition: serde_json::Value,
    pub checksum: String,
    pub published: bool,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateFlowVersion {
    pub flow_id: FlowId,
    pub version: Option<String>,
    pub flow_definition: serde_json::Value,
    pub published: Option<bool>,
    pub description: Option<String>,
}

impl Default for CreateFlowVersion {
    fn default() -> Self {
        Self {
            flow_id: "".to_string(),
            version: Some("0.0.1".to_string()),
            flow_definition: serde_json::json!("{}"),
            published: Some(false),
            description: None,
        }
    }
}

impl Into<CreateFlowVersion> for CreateFlow {
    fn into(self) -> CreateFlowVersion {
        CreateFlowVersion {
            flow_id: self.name.clone(),
            version: Some("v0.0.0".to_string()),
            flow_definition: serde_json::json!("{}"),
            published: Some(false),
            description: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateFlow {
    pub flow_name: String,
    pub version: Option<String>,
}

impl UpdateFlow {
    pub fn new(flow_name: String) -> Self {
        Self {
            flow_name,
            version: None,
        }
    }
}

impl From<anything_graph::Flow> for UpdateFlow {
    fn from(value: anything_graph::Flow) -> Self {
        Self {
            flow_name: value.name,
            version: Some(value.version),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateFlowVersion {
    pub version: Option<String>,
    pub flow_definition: Option<String>,
    pub published: Option<bool>,
    pub description: Option<String>,
}
