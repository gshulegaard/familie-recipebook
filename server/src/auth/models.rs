use sqlx::types::chrono::{DateTime, Local};
use crate::sql::Constraint;

#[derive(sqlx::FromRow)]
pub struct EmailRecord {
    pub id: String,
    pub address: String,
    pub verified: bool,
    pub created: DateTime<Local>,
    pub updated: DateTime<Local>,
    pub deleted: bool,
}

impl EmailRecord {
    pub async fn load(db_pool: sqlx::PgPool, constraints: &[Constraint])
            -> Result<Vec<EmailRecord>, sqlx::Error> {
        // Start with a generic select all query...
        let mut query_string = "SELECT * FROM email".to_string();

        // ... if there are constraints, add them to the query...
        if !constraints.is_empty() {
            query_string.push_str(" WHERE");
            for i in 0..constraints.len() {
                if i != 0 {
                    query_string.push_str(" AND");
                }
                let constraint_string = constraints[i].to_sql();
                query_string.push_str(&constraint_string);
            }
        }

        // ...now execute the constructed query and return the results.
        let emails = sqlx::query_as::<_, EmailRecord>(
                &query_string
            )
            .fetch_all(&db_pool)
            .await;

        return emails;
    }
}