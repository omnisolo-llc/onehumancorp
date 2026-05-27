use std::fs;

fn main() {
    let path = "src/server/services/booking.rs";
    let content = fs::read_to_string(path).unwrap();

    let old_insert = r#"        sqlx::query(
            "INSERT INTO bookings (id, tenant_id, customer_id, product_id, start_time, end_time, status) \
             VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(&booking.id)
        .bind(&booking.tenant_id)
        .bind(&booking.customer_id)
        .bind(&booking.product_id)
        .bind(booking.start_time)
        .bind(booking.end_time)
        .bind(&booking.status)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;"#;

    let new_insert = r#"        sqlx::query(
            "INSERT INTO bookings (id, tenant_id, customer_id, product_id, start_time, end_time, status) \
             VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(&booking.id)
        .bind(&booking.tenant_id)
        .bind(&booking.customer_id)
        .bind(&booking.product_id)
        .bind(booking.start_time)
        .bind(booking.end_time)
        .bind(&booking.status)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        // Phase 1 Dual-Write to new unified schema (Transaction table)
        let amount_cents = 0; // Default amount since it's not present in BookingRecord
        let _ = sqlx::query(
            "INSERT INTO transactions (id, tenant_id, offering_id, customer_id, type, status, amount_cents) \
             VALUES ($1, $2, $3, $4, 'booking', $5, $6) \
             ON CONFLICT (id) DO NOTHING"
        )
        .bind(&booking.id)
        .bind(&booking.tenant_id)
        .bind(&booking.product_id)
        .bind(&booking.customer_id)
        .bind(&booking.status)
        .bind(amount_cents)
        .execute(&mut *tx)
        .await;"#;

    let modified = content.replace(old_insert, new_insert);

    let old_upsert_service = r#"        sqlx::query(
            "INSERT INTO products (id, tenant_id, title, description, price_cents, type) \
             VALUES ($1, $2, $3, $4, $5, 'booking') \
             ON CONFLICT (id) DO UPDATE SET \
             title = EXCLUDED.title, \
             description = EXCLUDED.description, \
             price_cents = EXCLUDED.price_cents, \
             updated_at = CURRENT_TIMESTAMP"
        )
        .bind(&service.id)
        .bind(&service.tenant_id)
        .bind(&service.title)
        .bind(&service.description)
        .bind(service.price_cents)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;"#;

    let new_upsert_service = r#"        sqlx::query(
            "INSERT INTO products (id, tenant_id, title, description, price_cents, type) \
             VALUES ($1, $2, $3, $4, $5, 'booking') \
             ON CONFLICT (id) DO UPDATE SET \
             title = EXCLUDED.title, \
             description = EXCLUDED.description, \
             price_cents = EXCLUDED.price_cents, \
             updated_at = CURRENT_TIMESTAMP"
        )
        .bind(&service.id)
        .bind(&service.tenant_id)
        .bind(&service.title)
        .bind(&service.description)
        .bind(service.price_cents)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        // Phase 1 Dual-Write to new unified schema (Offerings table)
        let metadata = "{}"; // Empty JSONB for service
        let _ = sqlx::query(
            "INSERT INTO offerings (id, tenant_id, type, title, description, price_cents, metadata) \
             VALUES ($1, $2, 'service', $3, $4, $5, $6) \
             ON CONFLICT (id) DO UPDATE SET \
             title = EXCLUDED.title, \
             description = EXCLUDED.description, \
             price_cents = EXCLUDED.price_cents, \
             metadata = EXCLUDED.metadata"
        )
        .bind(&service.id)
        .bind(&service.tenant_id)
        .bind(&service.title)
        .bind(&service.description)
        .bind(service.price_cents)
        .bind(metadata)
        .execute(&mut *tx)
        .await;"#;

    let modified = modified.replace(old_upsert_service, new_upsert_service);

    fs::write(path, modified).unwrap();
    println!("Updated booking.rs");
}
