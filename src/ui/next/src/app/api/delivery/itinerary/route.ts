import { NextResponse } from "next/server";
import { headers } from "next/headers";

// Assuming we would communicate with the grpc DeliveryService here,
// but for the sake of the exercise we will fetch directly from the DB like other Next.js endpoints in this repo.
import { Pool } from "pg";

const pool = new Pool({
  connectionString: process.env.DATABASE_URL,
});

export async function GET(request: Request) {
  const url = new URL(request.url);
  const date = url.searchParams.get("date") || new Date().toISOString().split('T')[0];
  const headersList = headers();
  // Simplified tenant extraction for now
  const tenantId = headersList.get("x-tenant-id") || "default_tenant";

  try {
    const client = await pool.connect();

    // In a real implementation we would call the rust API layer,
    // For this example we just query directly to test it.
    const result = await client.query(
      `SELECT dt.id, dt.organization_id, dt.order_id, dt.status,
              EXTRACT(EPOCH FROM dt.estimated_arrival)::BIGINT as estimated_arrival_unix,
              ST_Y(dt.delivery_location) as delivery_location_lat,
              ST_X(dt.delivery_location) as delivery_location_lng,
              rp.delivery_date
       FROM delivery_tasks dt
       LEFT JOIN route_plans rp ON dt.route_plan_id = rp.id
       WHERE dt.organization_id = $1 AND (rp.delivery_date = $2::DATE OR rp.delivery_date IS NULL)
       ORDER BY dt.estimated_arrival ASC`,
      [tenantId, date]
    );

    client.release();

    return NextResponse.json({ tasks: result.rows });
  } catch (error) {
    console.error("Error fetching delivery itinerary:", error);
    return NextResponse.json({ error: "Failed to fetch itinerary" }, { status: 500 });
  }
}
