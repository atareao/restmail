use serde::{Serialize, Deserialize};
use sqlx::{sqlite::{SqlitePool, SqliteRow}, query, Row};
use std::fmt::{self, Display};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User{
    id: i64,
    name: String,
    email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewUser{
    #[serde(default = "get_default_name")]
    name: String,
    email: String,
}

fn get_default_name() -> String{
    "".to_string()
}

impl User {
    fn from_row(row: SqliteRow) -> Self{
        Self{
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
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

    pub async fn create(pool: &SqlitePool, new_user: NewUser)
            -> Result<User, sqlx::Error>{
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
            -> Result<Option<User>, sqlx::Error>{
        let sql = "SELECT * FROM users WHERE id = $1";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_optional(pool)
            .await
    }

    pub async fn read_all(pool: &SqlitePool)
            -> Result<Vec<User>, sqlx::Error>{
        let sql = "SELECT * FROM users";
        query(sql)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
    }

    pub async fn update(pool: &SqlitePool, user: User) -> Result<Option<User>, sqlx::Error>{
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

    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<Option<User>, sqlx::Error>{
        let sql = "DELETE FROM users WHERE id = $1 RETURNIN *;";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_optional(pool)
            .await
    }
}

impl Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}. Name: {}. Email: {}", self.id, self.name, self.email)
    }
}
