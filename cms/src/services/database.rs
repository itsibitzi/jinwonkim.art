use sqlx::SqlitePool;

use crate::model::{category::Category, error::Error, image::Image, session::Session};

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new() -> Result<Database, sqlx::Error> {
        let pool = SqlitePool::connect("jinwonkim.db").await?;

        Ok(Database { pool })
    }

    pub async fn migrate(&self) -> Result<(), Error> {
        Ok(sqlx::migrate!().run(&self.pool).await?)
    }

    pub async fn create_category(&self, name: &str) -> Result<(), Error> {
        let mut conn = self.pool.acquire().await?;

        let name_valid = name.chars().all(|c| c.is_ascii_alphabetic() || c == ' ');

        if name_valid {
            let id = name.to_lowercase().replace(" ", "_");
            sqlx::query!(
                r#"
                INSERT INTO categories (id, name) VALUES (?1, ?2)
                "#,
                id,
                name
            )
            .execute(&mut conn)
            .await?;

            Ok(())
        } else {
            Err(Error::IllegalStateError(
                "Must use English characters only in name",
            ))
        }
    }

    pub async fn delete_category(&self, id: &str) -> Result<(), Error> {
        let mut conn = self.pool.acquire().await?;

        sqlx::query!(
            r#"
                DELETE FROM categories WHERE id = ?1
            "#,
            id
        )
        .execute(&mut conn)
        .await?;

        Ok(())
    }

    pub async fn list_categories(&self) -> Result<Vec<Category>, Error> {
        let categories = sqlx::query_as!(
            Category,
            r#"
            SELECT id, name FROM categories
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(categories)
    }

    pub async fn create_image(&self, name: &str, filename: &str) -> Result<(), Error> {
        let mut conn = self.pool.acquire().await?;

        sqlx::query!(
            r#"
            INSERT INTO images (name, filename) VALUES (?1, ?2)
            "#,
            name,
            filename
        )
        .execute(&mut conn)
        .await?;

        Ok(())
    }

    pub async fn list_images(&self) -> Result<Vec<Image>, Error> {
        let rows: Vec<_> = sqlx::query!(
            r#"
            SELECT 
              images.id       AS image_id,
              images.name     AS image_name,
              images.filename AS image_filename, 
              categories.id   AS category_id,
              categories.name AS category_name
            FROM images
            LEFT OUTER JOIN category_images ON category_images.image_id = images.id
            LEFT OUTER JOIN categories ON category_images.category_id = categories.id
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        // I feel there must be an easier way of doing this...
        let images = rows
            .group_by(|a, b| a.image_id == b.image_id)
            .map(|group| {
                let first = &group[0];
                Image {
                    id: first.image_id,
                    name: first.image_name.clone(),
                    filename: first.image_filename.clone(),
                    categories: group
                        .iter()
                        .filter_map(|row| {
                            match (row.category_id.clone(), row.category_name.clone()) {
                                (Some(id), Some(name)) => Some(Category { id, name }),
                                _ => None,
                            }
                        })
                        .collect(),
                }
            })
            .collect();

        Ok(images)
    }

    pub async fn list_images_for_category(&self, category: &str) -> Result<Vec<Image>, Error> {
        unimplemented!()
    }

    pub async fn get_session(&self, id: &str) -> Result<Session, Error> {
        let session = sqlx::query_as!(
            Session,
            r#"
            SELECT id, expires FROM sessions WHERE id = ?1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(session)
    }
}
