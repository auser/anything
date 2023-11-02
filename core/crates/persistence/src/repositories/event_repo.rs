use sqlx::Row;
use std::fmt::Debug;

use crate::{
    datastore::{Datastore, DatastoreTrait, RepoImpl},
    error::{PersistenceError, PersistenceResult},
    models::event::{CreateEvent, EventId, EventList, StoreEvent},
};

/// Guarenteed Methods available in the Trigger repo
#[async_trait::async_trait]
pub trait EventRepo {
    async fn save_event(&self, event: CreateEvent) -> PersistenceResult<EventId>;
    async fn find_by_id(&self, event_id: EventId) -> PersistenceResult<StoreEvent>;
    async fn find_events_since(
        &self,
        since_date: chrono::DateTime<chrono::Utc>,
    ) -> PersistenceResult<EventList>;
}

#[derive(Clone)]
pub struct EventRepoImpl {
    pub datastore: Datastore,
}

impl Debug for EventRepoImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EventRepoImpl  {{ /* Format your fields here */ }}")
    }
}

#[cfg(feature = "sqlite")]
#[async_trait::async_trait]
impl RepoImpl<sqlx::Sqlite> for EventRepoImpl {
    fn new_with_datastore(datastore: Datastore) -> PersistenceResult<Self> {
        Ok(EventRepoImpl { datastore })
    }

    async fn get_transaction<'a>(&self) -> PersistenceResult<sqlx::Transaction<'a, sqlx::Sqlite>> {
        let pool = self.datastore.get_pool();
        let tx = pool
            .begin()
            .await
            .map_err(|e| PersistenceError::DatabaseError(e))?;

        Ok(tx)
    }
}

#[async_trait::async_trait]
impl EventRepo for EventRepoImpl {
    async fn save_event(&self, event: CreateEvent) -> PersistenceResult<EventId> {
        let pool = self.datastore.get_pool();
        let row = sqlx::query(
            r#"
            INSERT INTO events (id, flow_id, trigger_id, name, context, started_at, ended_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            RETURNING id
            "#,
        )
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(event.flow_id)
        .bind(event.trigger_id)
        .bind(event.name)
        .bind(event.context)
        .bind(event.started_at)
        .bind(event.ended_at)
        // .bind(Utc::now())
        .fetch_one(pool)
        .await
        .map_err(|e| PersistenceError::DatabaseError(e))?;

        let id = row.get("id");

        Ok(id)
    }

    async fn find_by_id(&self, event_id: EventId) -> PersistenceResult<StoreEvent> {
        let pool = self.datastore.get_pool();
        let row = sqlx::query_as::<_, StoreEvent>("SELECT * from events WHERE id = ?1")
            .bind(event_id)
            .fetch_one(pool)
            .await
            .map_err(|e| PersistenceError::DatabaseError(e))?;

        Ok(row)
    }

    async fn find_events_since(
        &self,
        since_date: chrono::DateTime<chrono::Utc>,
    ) -> PersistenceResult<EventList> {
        let pool = self.datastore.get_pool();
        let rows = sqlx::query_as::<_, StoreEvent>(
            "SELECT * from events WHERE started_at > ?1 ORDER BY started_at ASC",
        )
        .bind(since_date)
        .fetch_all(pool)
        .await
        .map_err(|e| PersistenceError::DatabaseError(e))?;

        Ok(rows)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};

    use super::*;
    use crate::test_helper::{get_test_datastore, TestEventHelper};

    #[tokio::test]
    async fn test_save_event() {
        let datastore = get_test_datastore().await.unwrap();
        let event_repo = EventRepoImpl::new_with_datastore(datastore.clone()).unwrap();
        let test_helper = TestEventHelper::new(datastore.clone());

        let create_event = CreateEvent {
            name: "test".to_string(),
            flow_id: None,
            trigger_id: None,
            context: serde_json::json!({}),
            started_at: None,
            ended_at: None,
        };

        let res = event_repo.save_event(create_event.clone()).await;
        assert!(res.is_ok());
        let event_id = res.unwrap();

        let stored_event = test_helper.get_event_by_id(event_id).await;

        assert_eq!(stored_event.name, "test".to_string());
        assert_eq!(stored_event.flow_id, None);
    }

    #[tokio::test]
    async fn test_get_event_by_id() {
        let datastore = get_test_datastore().await.unwrap();
        let event_repo = EventRepoImpl::new_with_datastore(datastore.clone()).unwrap();
        // let test_helper = TestEventHelper::new(datastore.clone());

        let event_id = create_event(
            event_repo.clone(),
            "test".to_string(),
            "test".to_string(),
            None,
            None,
        )
        .await;

        let stored_event = event_repo.find_by_id(event_id).await.unwrap();
        assert_eq!(stored_event.name, "test".to_string());
        assert_eq!(stored_event.flow_id, Some("test".to_string()));
    }

    #[tokio::test]
    async fn test_get_events_since() {
        let datastore = get_test_datastore().await.unwrap();
        let event_repo = EventRepoImpl::new_with_datastore(datastore.clone()).unwrap();
        // let test_helper = TestEventHelper::new(datastore.clone());

        let event_id = create_event(
            event_repo.clone(),
            "test".to_string(),
            "test".to_string(),
            None,
            None,
        )
        .await;
        let _event_id2 = create_event(
            event_repo.clone(),
            "earlier".to_string(),
            "test".to_string(),
            None,
            Some(Utc::now() - Duration::days(31)),
        )
        .await;
        let event_id3 = create_event(
            event_repo.clone(),
            "test".to_string(),
            "test".to_string(),
            None,
            Some(Utc::now() - Duration::days(20)),
        )
        .await;

        let stored_event = event_repo
            .find_events_since(Utc::now() - Duration::days(21))
            .await
            .unwrap();
        assert!(stored_event.len() == 2);
        assert!(stored_event.iter().any(|e| e.id == event_id));
        assert!(stored_event.iter().any(|e| e.id == event_id3));
    }

    async fn create_event(
        event_repo: EventRepoImpl,
        event_name: String,
        flow_id: String,
        context: Option<serde_json::Value>,
        started_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> EventId {
        let create_event = CreateEvent {
            name: event_name,
            flow_id: Some(flow_id),
            trigger_id: None,
            context: context.unwrap_or(serde_json::json!({"test": "test"})),
            started_at: Some(started_at.unwrap_or(chrono::offset::Utc::now())),
            ended_at: None,
        };

        let res = event_repo.save_event(create_event.clone()).await;
        assert!(res.is_ok());
        let event_id = res.unwrap();
        event_id
    }
}
