use serde::{Serialize, Deserialize};
use sqlx::{sqlite::{SqlitePool, SqliteRow}, query, Row};
use std::fmt::{self, Display};
use chrono::{DateTime, NaiveDate, Utc, NaiveDateTime};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Email{
    id: i64,
    from: i64,
    to: String,
    subject: String,
    body: String,
    date: DateTime<Utc>,
    inbox: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewEmail{
    from: i64,
    to: String,
    #[serde(default = "get_default_empty")]
    subject: String,
    #[serde(default = "get_default_empty")]
    body: String,
    date: String,
    #[serde(default = "get_default_true")]
    inbox: bool,
}

fn get_default_empty() -> String{
    "".to_string()
}

fn get_default_true() -> bool{
    true
}

fn get_default_date() -> DateTime<Utc>{
    NaiveDateTime::timestamp_nanos
    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc)
}

impl Email {
    fn from_row(row: SqliteRow) -> Self{
        Self{
            id: row.get("id"),
            from: row.get("from"),
            to: row.get("to"),
            subject: row.get("subject"),
            body: row.get("body"),
            date: row.get("date"),
            inbox: row.get("inbox"),
        }
    }

    pub fn get_id(&self) -> i64{
        self.id
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_email(&self) -> &str{
        &self.email
    }

    pub async fn create(pool: &SqlitePool, new_user: NewEmail)
            -> Result<Email, sqlx::Error>{
        tracing::info!("Data: {:?}", new_user);
        let sql = "INSERT INTO users (name, email) 
                   VALUES ($1, $2) RETURNING *;";
        query(sql)
            .bind(new_user.name)
            .bind(new_user.email)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn read(pool: &SqlitePool, id: i64)
            -> Result<Option<Email>, sqlx::Error>{
        let sql = "SELECT * FROM users WHERE id = $1";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_optional(pool)
            .await
    }

    pub async fn read_all(pool: &SqlitePool)
            -> Result<Vec<Email>, sqlx::Error>{
        let sql = "SELECT * FROM users";
        query(sql)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
    }

    pub async fn update(pool: &SqlitePool, user: Email) -> Result<Option<Email>, sqlx::Error>{
        let sql = "UPDATE users SET name = $2, email =$3
                   FROM users WHERE id = $1 RETURNING *;";
        query(sql)
            .bind(user.id)
            .bind(user.name)
            .bind(user.email)
            .map(Self::from_row)
            .fetch_optional(pool)
            .await
    }

    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<Option<Email>, sqlx::Error>{
        let sql = "DELETE FROM users WHERE id = $1 RETURNIN *;";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_optional(pool)
            .await
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}. Name: {}. Email: {}", self.id, self.name, self.email)
    }
}
