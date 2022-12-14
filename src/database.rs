use crate::types::*;

pub async fn insert(pool: &sqlx::SqlitePool, site: &SiteData) -> Result<(), sqlx::Error> {
    // Add top level searched domain
    let domain = sqlx::query!(
        r#"
INSERT INTO DOMAINS (rowid, base_url)
VALUES (NULL, ?1) RETURNING id
        "#,
        site.root
    )
    .fetch_one(pool)
    .await?;

    // Record all top level domain local links
    for link in site.local.iter() {
        println!("content: {:?}", link.url);
        sqlx::query!(
            r#"
INSERT INTO DOMAIN_LOCAL_LINKS (domain_id, link_url, title, body)
VALUES (?1, ?2, ?3, ?4)
            "#,
            domain.id,
            link.url,
            link.title,
            link.body
        )
        .execute(pool)
        .await?;
    }

    // Record any new external links from top level domain
    // Record their association
    for link in site.remote.iter() {
        sqlx::query!(
            r#"
INSERT OR IGNORE INTO DOMAINS (rowid, base_url)
VALUES (NULL, ?1) RETURNING id
            "#,
            link
        )
        .fetch_one(pool)
        .await?;
    }

    Ok(())
}
