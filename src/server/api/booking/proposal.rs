use axum::{
    extract::Path,
    response::{IntoResponse, Html},
    routing::get,
    Router,
};
use std::sync::Arc;
use crate::db::DB;

pub fn router<S>(db: Arc<DB>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/p/:proposal_id", get(handle_view_proposal))
        .with_state(db)
}

// In a full implementation, this would read from the DB.
// For the research task, we'll return a hardcoded HTML response representing the mock Stripe payment flow.
async fn handle_view_proposal(
    Path(_proposal_id): Path<String>,
) -> impl IntoResponse {
    let html = format!(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no">
    <title>Booking Proposal</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
            margin: 0;
            padding: 20px;
            background-color: #f9fafb;
            color: #333;
        }}
        .card {{
            background: white;
            border-radius: 12px;
            padding: 20px;
            box-shadow: 0 4px 6px rgba(0,0,0,0.05);
            max-width: 400px;
            margin: 40px auto;
        }}
        h1 {{ margin-top: 0; font-size: 20px; }}
        p {{ line-height: 1.5; color: #555; }}
        .amount {{
            font-size: 24px;
            font-weight: bold;
            color: #000;
            margin: 20px 0;
            text-align: center;
        }}
        .btn {{
            display: block;
            width: 100%;
            padding: 14px;
            background-color: #0f172a;
            color: white;
            text-align: center;
            border: none;
            border-radius: 8px;
            font-size: 16px;
            font-weight: bold;
            cursor: pointer;
            text-decoration: none;
        }}
        .btn:active {{ background-color: #1e293b; }}
    </style>
</head>
<body>
    <div class="card">
        <h1>Confirm Your Booking</h1>
        <p>Please review the details below and pay the deposit to confirm your appointment.</p>

        <div style="background: #f1f5f9; padding: 12px; border-radius: 6px; margin: 15px 0;">
            <p style="margin:0; font-size: 14px;"><strong>Service:</strong> Sink Repair Diagnostic</p>
            <p style="margin:5px 0 0 0; font-size: 14px;"><strong>Time:</strong> Tuesday, 10:00 AM</p>
        </div>

        <div class="amount">Deposit: $50.00</div>

        <button class="btn" onclick="alert('Simulated Stripe Checkout Successful. Appointment Confirmed!')">Pay with Stripe</button>
    </div>
</body>
</html>
        "#,
    );

    Html(html)
}
