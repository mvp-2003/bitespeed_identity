use crate::models::{ContactRow, IdentifyRequest};
use sqlx::{PgPool, Row};
use anyhow::Result;

pub async fn fetch_matching_contacts(pool: &PgPool, req: &IdentifyRequest) -> Result<Vec<ContactRow>> {
    if req.email.is_none() && req.phone_number.is_none() {
        return Ok(vec![]);
    }
    let rows = sqlx::query_as::<_, ContactRow>(r#"
        SELECT * FROM contacts
        WHERE ($1::text IS NOT NULL AND email = $1)
           OR ($2::text IS NOT NULL AND phone_number = $2)
        ORDER BY created_at ASC
    "#)
        .bind(req.email.as_deref())
        .bind(req.phone_number.as_deref())
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

pub async fn fetch_contact(pool: &PgPool, id: i64) -> Result<ContactRow> {
    let row = sqlx::query_as::<_, ContactRow>(
        "SELECT * FROM contacts WHERE id = $1"
    )
    .bind(id)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn fetch_contacts_by_primary(pool: &PgPool, primary_id: i64) -> Result<Vec<ContactRow>> {
    let rows = sqlx::query_as::<_, ContactRow>(r#"
        SELECT * FROM contacts
        WHERE id = $1 OR linked_id = $1
        ORDER BY created_at ASC
    "#)
        .bind(primary_id)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

pub async fn update_contact_to_secondary(pool: &PgPool, id: i64, primary_id: i64) -> Result<()> {
    sqlx::query(r#"
        UPDATE contacts
        SET link_precedence = 'secondary',
            linked_id = $2,
            updated_at = NOW()
        WHERE id = $1
    "#)
        .bind(id)
        .bind(primary_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_contact_primary(pool: &PgPool, id: i64) -> Result<()> {
    // used in rare cases to promote (shouldn't be needed in normal flow)
    sqlx::query(r#"
        UPDATE contacts
        SET link_precedence = 'primary',
            linked_id = NULL,
            updated_at = NOW()
        WHERE id = $1
    "#)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn create_contact(
    pool: &PgPool,
    email: Option<&str>,
    phone_number: Option<&str>,
    linked_id: Option<i64>,
    link_precedence: &str,
) -> Result<i64> {
    let rec = sqlx::query(r#"
        INSERT INTO contacts (email, phone_number, linked_id, link_precedence)
        VALUES ($1, $2, $3, $4)
        RETURNING id
    "#)
        .bind(email)
        .bind(phone_number)
        .bind(linked_id)
        .bind(link_precedence)
        .fetch_one(pool)
        .await?;
    Ok(rec.get::<i64, _>("id"))
}