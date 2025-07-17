use crate::models::{ContactRow, IdentifyRequest, ContactSummary};
use crate::repo;
use anyhow::Result;
use sqlx::PgPool;
use std::collections::HashSet;
use std::env;

pub async fn identify(pool: &PgPool, req: IdentifyRequest) -> Result<ContactSummary> {

    let matches = repo::fetch_matching_contacts(pool, &req).await?;

    if matches.is_empty() {

        let id = repo::create_contact(
            pool,
            req.email.as_deref(),
            req.phone_number.as_deref(),
            None,
            "primary",
        ).await?;
        let row = repo::fetch_contact(pool, id).await?;
        return Ok(build_summary(vec![row]));
    }

    let cluster = expand_group(pool, &matches).await?;

    let (mut cluster, primary_id) = normalize_group(pool, cluster).await?;

    if needs_new_secondary(&req, &cluster) {
        let full = env::var("WRITE_FULL_ROW").unwrap_or_else(|_| "false".into()) == "true";
        let (email, phone) = if full {
            (req.email.as_deref(), req.phone_number.as_deref())
        } else {
            let (ne, np) = unseen_fields(&req, &cluster);
            (ne, np)
        };
        if email.is_some() || phone.is_some() {
            let id = repo::create_contact(pool, email, phone, Some(primary_id), "secondary").await?;
            let new_row = repo::fetch_contact(pool, id).await?;
            cluster.push(new_row);
        }
    }

    Ok(build_summary(cluster))
}

async fn expand_group(pool: &PgPool, matches: &[ContactRow]) -> Result<Vec<ContactRow>> {
    let mut seen = HashSet::new();
    let mut out: Vec<ContactRow> = Vec::new();

    for r in matches {
        if !seen.insert(r.id) {
            continue;
        }
        out.push(r.clone());

        match r.link_precedence.as_str() {
            "primary" => {
                let fam = repo::fetch_contacts_by_primary(pool, r.id).await?;
                for f in fam {
                    if seen.insert(f.id) {
                        out.push(f);
                    }
                }
            }
            "secondary" => {
                if let Some(pid) = r.linked_id {
                    let fam = repo::fetch_contacts_by_primary(pool, pid).await?;
                    for f in fam {
                        if seen.insert(f.id) {
                            out.push(f);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    out.sort_by_key(|r| r.created_at);
    Ok(out)
}

async fn normalize_group(pool: &PgPool, mut rows: Vec<ContactRow>) -> Result<(Vec<ContactRow>, i64)> {
    rows.sort_by_key(|r| r.created_at);
    let primary_id = rows.first().map(|r| r.id).unwrap();

    for r in &rows {
        if r.id == primary_id {
            continue;
        }
        if r.link_precedence != "secondary" || r.linked_id != Some(primary_id) {
            repo::update_contact_to_secondary(pool, r.id, primary_id).await?;
        }
    }

    let rows = repo::fetch_contacts_by_primary(pool, primary_id).await?;
    Ok((rows, primary_id))
}

fn needs_new_secondary(req: &IdentifyRequest, rows: &[ContactRow]) -> bool {
    let mut emails = HashSet::new();
    let mut phones = HashSet::new();
    for r in rows {
        if let Some(e) = &r.email { emails.insert(e); }
        if let Some(p) = &r.phone_number { phones.insert(p); }
    }
    if let Some(e) = &req.email {
        if !emails.contains(e) { return true; }
    }
    if let Some(p) = &req.phone_number {
        if !phones.contains(p) { return true; }
    }
    false
}

fn unseen_fields<'a>(req: &'a IdentifyRequest, rows: &'a [ContactRow]) -> (Option<&'a str>, Option<&'a str>) {
    let mut emails = HashSet::new();
    let mut phones = HashSet::new();
    for r in rows {
        if let Some(e) = &r.email { emails.insert(e); }
        if let Some(p) = &r.phone_number { phones.insert(p); }
    }
    let e = req.email.as_ref().and_then(|x| if !emails.contains(x) { Some(x.as_str()) } else { None });
    let p = req.phone_number.as_ref().and_then(|x| if !phones.contains(x) { Some(x.as_str()) } else { None });
    (e, p)
}

fn build_summary(mut rows: Vec<ContactRow>) -> ContactSummary {
    rows.sort_by_key(|r| r.created_at);
    let primary_id = rows[0].id;

    let mut emails_seen = HashSet::new();
    let mut phones_seen = HashSet::new();
    let mut emails = Vec::new();
    let mut phones = Vec::new();
    let mut secondary_ids = Vec::new();

    if let Some(e) = &rows[0].email {
        if emails_seen.insert(e.clone()) { emails.push(e.clone()); }
    }
    if let Some(p) = &rows[0].phone_number {
        if phones_seen.insert(p.clone()) { phones.push(p.clone()); }
    }

    for r in rows.iter().skip(1) {
        if let Some(e) = &r.email {
            if emails_seen.insert(e.clone()) { emails.push(e.clone()); }
        }
        if let Some(p) = &r.phone_number {
            if phones_seen.insert(p.clone()) { phones.push(p.clone()); }
        }
        if r.id != primary_id {
            secondary_ids.push(r.id);
        }
    }

    ContactSummary {
        primaryContactId: primary_id,
        emails,
        phoneNumbers: phones,
        secondaryContactIds: secondary_ids,
    }
}