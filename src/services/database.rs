use sqlx::SqlitePool;

use crate::model::{
    about::About, category::Category, db::IdAndPosition, error::Error, faq::Faq,
    forms::faq::CreateFaq, image::Image, user::User,
};

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

        let name_valid = name.chars().all(|c| c.is_ascii_alphabetic() || c == ' ')
            && name != "faq"
            && name != "home"
            && name != "about";

        if name_valid {
            let id = name.to_lowercase().replace(" ", "-");
            sqlx::query!(
                "INSERT INTO categories (id, name) VALUES (?1, ?2)",
                id,
                name
            )
            .execute(&mut conn)
            .await?;

            Ok(())
        } else {
            Err(Error::IllegalStateError(
                "Must use English alphabetic characters only in name",
            ))
        }
    }

    pub async fn delete_category(&self, id: &str) -> Result<(), Error> {
        let mut conn = self.pool.acquire().await?;

        sqlx::query!("DELETE FROM categories WHERE id = ?1", id)
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    pub async fn list_categories(&self) -> Result<Vec<Category>, Error> {
        let categories = sqlx::query_as!(
            Category,
            "SELECT id, name FROM categories ORDER BY name ASC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(categories)
    }

    pub async fn create_image(
        &self,
        name: String,
        description: String,
        filename: String,
        categories: Vec<String>,
    ) -> Result<(), Error> {
        let mut tx = self.pool.begin().await?;

        let name = name.trim();
        let description = description.trim();
        let filename = filename.trim();

        let image_id = sqlx::query!(
            "INSERT INTO images (name, description, filename) VALUES (?1, ?2, ?3)",
            name,
            description,
            filename
        )
        .execute(&mut tx)
        .await?
        .last_insert_rowid();

        for category_id in categories {
            sqlx::query!(
                "INSERT INTO category_images (category_id, image_id) VALUES (?1, ?2)",
                category_id,
                image_id
            )
            .execute(&mut tx)
            .await?;
        }

        tx.commit().await?;

        Ok(())
    }

    pub async fn update_image(
        &self,
        image_id: i64,
        name: String,
        description: String,
        categories: Vec<String>,
    ) -> Result<(), Error> {
        let mut tx = self.pool.begin().await?;

        let name = name.trim();
        let description = description.trim();

        sqlx::query!(
            r#"
            UPDATE images SET name = ?1, description = ?2 WHERE id = ?3
            "#,
            name,
            description,
            image_id
        )
        .execute(&mut tx)
        .await?;

        // Clear out old categorisations
        sqlx::query!("DELETE FROM category_images WHERE image_id = ?1", image_id)
            .execute(&mut tx)
            .await?;

        for category_id in categories {
            sqlx::query!(
                r#"
                INSERT INTO category_images (category_id, image_id) VALUES (?1, ?2)
            "#,
                category_id,
                image_id
            )
            .execute(&mut tx)
            .await?;
        }

        tx.commit().await?;

        Ok(())
    }

    pub async fn list_images(&self) -> Result<Vec<Image>, Error> {
        let rows: Vec<_> = sqlx::query!(
            r#"
            SELECT 
              images.id          AS image_id,
              images.name        AS image_name,
              images.description AS image_description,
              images.filename    AS image_filename, 
              categories.id      AS category_id,
              categories.name    AS category_name
            FROM images
            LEFT OUTER JOIN category_images ON category_images.image_id = images.id
            LEFT OUTER JOIN categories ON category_images.category_id = categories.id
            ORDER BY position ASC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let images = rows
            .group_by(|a, b| a.image_id == b.image_id)
            .map(|group| {
                let first = &group[0];
                Image {
                    id: first.image_id,
                    name: first.image_name.clone(),
                    description: first.image_description.clone(),
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
        let rows: Vec<_> = sqlx::query!(
            r#"
            SELECT 
              images.id          AS image_id,
              images.name        AS image_name,
              images.description AS image_description,
              images.filename    AS image_filename, 
              categories.id      AS category_id,
              categories.name    AS category_name
            FROM images
            LEFT OUTER JOIN category_images ON category_images.image_id = images.id
            LEFT OUTER JOIN categories ON category_images.category_id = categories.id
            WHERE categories.id = ?1
            ORDER BY position ASC
            "#,
            category
        )
        .fetch_all(&self.pool)
        .await?;

        let images = rows
            .group_by(|a, b| a.image_id == b.image_id)
            .map(|group| {
                let first = &group[0];
                Image {
                    id: first.image_id,
                    name: first.image_name.clone(),
                    description: first.image_description.clone(),
                    filename: first.image_filename.clone(),
                    categories: group
                        .iter()
                        .map(|row| Category {
                            id: row.category_id.clone(),
                            name: row.category_name.clone(),
                        })
                        .collect(),
                }
            })
            .collect();

        Ok(images)
    }

    pub async fn get_image_by_id(&self, image_id: i64) -> Result<Option<Image>, Error> {
        let rows: Vec<_> = sqlx::query!(
            r#"
            SELECT 
              images.id          AS image_id,
              images.name        AS image_name,
              images.description AS image_description,
              images.filename    AS image_filename, 
              categories.id      AS category_id,
              categories.name    AS category_name
            FROM images
            LEFT OUTER JOIN category_images ON category_images.image_id = images.id
            LEFT OUTER JOIN categories ON category_images.category_id = categories.id
            WHERE images.id = ?1
            "#,
            image_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut images: Vec<Image> = rows
            .group_by(|a, b| a.image_id == b.image_id)
            .map(|group| {
                let first = &group[0];
                Image {
                    id: first.image_id,
                    name: first.image_name.clone(),
                    description: first.image_description.clone(),
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

        if images.len() > 0 {
            Ok(Some(images.remove(0)))
        } else {
            Ok(None)
        }
    }

    pub async fn insert_about(&self, text: String) -> Result<(), Error> {
        let text = text.trim();
        sqlx::query!(
            r#"
            INSERT INTO about (id, about_text) VALUES ('about', ?1)
            ON CONFLICT DO UPDATE SET about_text = excluded.about_text
            "#,
            text
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn select_about(&self) -> Result<String, Error> {
        let about = sqlx::query_as!(
            About,
            r#"
            SELECT about_text FROM about
            "#
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(about
            .map(|a| a.about_text)
            .unwrap_or("About page coming soon".into()))
    }

    pub async fn get_user(&self, username: &str) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT username, password_hash FROM users WHERE username = ?1
            "#,
            username
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn create_faq(&self, faq: CreateFaq) -> Result<(), Error> {
        let question = faq.question.trim();
        let answer = faq.answer.trim();

        sqlx::query!(
            "INSERT INTO faqs (question, answer) VALUES (?1, ?2)",
            question,
            answer
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn list_faqs(&self) -> Result<Vec<Faq>, Error> {
        let faqs = sqlx::query_as!(
            Faq,
            r#"
            SELECT id, question, answer 
            FROM faqs
            ORDER BY position ASC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(faqs)
    }

    pub async fn move_image(&self, id: i64, up: bool) -> Result<(), Error> {
        let mut tx = self.pool.begin().await?;

        let image = sqlx::query_as!(
            IdAndPosition,
            r#"SELECT id, position AS "position!" FROM images WHERE id = ?1"#,
            id
        )
        .fetch_one(&mut tx)
        .await?;

        let swap_image = if up {
            sqlx::query_as!(
                IdAndPosition,
                r#"SELECT 
                    id, 
                    position AS "position!"
                FROM images 
                WHERE position < ?1 
                ORDER BY position DESC"#,
                image.position
            )
            .fetch_one(&mut tx)
            .await?
        } else {
            sqlx::query_as!(
                IdAndPosition,
                r#"SELECT 
                    id, 
                    position AS "position!"
                FROM images 
                WHERE position > ?1 
                ORDER BY position ASC"#,
                image.position
            )
            .fetch_one(&mut tx)
            .await?
        };

        sqlx::query!(
            "UPDATE images SET position = ?1 WHERE id = ?2",
            image.position,
            swap_image.id
        )
        .execute(&mut tx)
        .await?;
        sqlx::query!(
            "UPDATE images SET position = ?1 WHERE id = ?2",
            swap_image.position,
            image.id
        )
        .execute(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn move_faq(&self, id: i64, up: bool) -> Result<(), Error> {
        let mut tx = self.pool.begin().await?;

        let faq = sqlx::query_as!(
            IdAndPosition,
            r#"SELECT id, position AS "position!" FROM faqs WHERE id = ?1"#,
            id
        )
        .fetch_one(&mut tx)
        .await?;

        let swap_faq = if up {
            sqlx::query_as!(
                IdAndPosition,
                r#"SELECT 
                    id, 
                    position AS "position!"
                FROM faqs 
                WHERE position < ?1 
                ORDER BY position DESC"#,
                faq.position
            )
            .fetch_one(&mut tx)
            .await?
        } else {
            sqlx::query_as!(
                IdAndPosition,
                r#"SELECT 
                    id, 
                    position AS "position!"
                FROM faqs 
                WHERE position > ?1 
                ORDER BY position ASC"#,
                faq.position
            )
            .fetch_one(&mut tx)
            .await?
        };

        sqlx::query!(
            "UPDATE faqs SET position = ?1 WHERE id = ?2",
            faq.position,
            swap_faq.id
        )
        .execute(&mut tx)
        .await?;
        sqlx::query!(
            "UPDATE faqs SET position = ?1 WHERE id = ?2",
            swap_faq.position,
            faq.id
        )
        .execute(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn delete_image(&self, id: i64) -> Result<(), Error> {
        let mut conn = self.pool.acquire().await?;

        sqlx::query!("DELETE FROM images WHERE id = ?1", id)
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    pub async fn delete_faq(&self, id: i64) -> Result<(), Error> {
        let mut conn = self.pool.acquire().await?;

        sqlx::query!(
            r#"
            DELETE FROM faqs WHERE id = ?1
        "#,
            id
        )
        .execute(&mut conn)
        .await?;

        Ok(())
    }
}