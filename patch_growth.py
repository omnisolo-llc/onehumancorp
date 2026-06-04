with open('src/server/api/growth.rs', 'r') as f:
    content = f.read()

old_block = """    let active_referrals: i64 = sqlx::query_scalar("SELECT COALESCE(SUM(conversions), 0) FROM referrals")
        .fetch_one(&state.pool)
        .await
        .unwrap_or(0);

    match tracker.get_total_invites_count().await {"""

new_block = """    let (active_referrals_res, total_invites_res) = tokio::join!(
        sqlx::query_scalar::<_, i64>("SELECT COALESCE(SUM(conversions), 0) FROM referrals").fetch_one(&state.pool),
        tracker.get_total_invites_count()
    );
    let active_referrals = active_referrals_res.unwrap_or(0);

    match total_invites_res {"""

if old_block in content:
    content = content.replace(old_block, new_block)
    with open('src/server/api/growth.rs', 'w') as f:
        f.write(content)
    print("Success")
else:
    print("Block not found")
