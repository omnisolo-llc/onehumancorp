import os
with open("src/server/lib.rs", "r") as f:
    c = f.read()

if "work_triage" not in c:
    c = c.replace('use crate::api::unified_inbox_webhook::{', 'use crate::api::work_triage::{get_daily_work_handler, approve_daily_work_handler, simulate_inbound_signal_handler};\nuse crate::api::unified_inbox_webhook::{')

    c = c.replace('.route("/api/dev/simulate-triage-item", axum::routing::post(simulate_ui_triage_item_handler).with_state(db.clone()))', '.route("/api/triage/simulate", axum::routing::post(simulate_inbound_signal_handler).with_state(db.clone()))\n        .route("/api/triage/items", axum::routing::get(get_daily_work_handler).with_state(db.clone()))\n        .route("/api/triage/items/:id/approve", axum::routing::post(approve_daily_work_handler).with_state(db.clone()))\n        .route("/api/dev/simulate-triage-item", axum::routing::post(simulate_ui_triage_item_handler).with_state(db.clone()))')
    with open("src/server/lib.rs", "w") as f:
        f.write(c)
