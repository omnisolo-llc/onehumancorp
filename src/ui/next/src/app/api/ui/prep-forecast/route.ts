export const dynamic = 'force-dynamic';
import { NextResponse } from "next/server";
import { Pool } from "pg";

const pool = new Pool({
  connectionString: process.env.DATABASE_URL || "postgres://postgres:postgres@localhost:5432/ohc",
});

export async function GET(request: Request) {
  try {
    const { searchParams } = new URL(request.url);
    const tenant_id = searchParams.get("tenant_id") || "default";

    const client = await pool.connect();
    try {
      await client.query("SELECT set_config('app.current_tenant', $1, false)", [tenant_id]);

      // Join with products table to get product name if available
      const result = await client.query(`
        SELECT p.*, pr.name as product_name
        FROM inventory_predictions p
        LEFT JOIN products pr ON p.product_id = pr.id AND p.tenant_id = pr.tenant_id
        WHERE p.tenant_id = $1
        ORDER BY p.predicted_stockout_date ASC
      `, [tenant_id]);

      return NextResponse.json({ predictions: result.rows });
    } finally {
      client.release();
    }
  } catch (error) {
    console.error("Prep forecast API error:", error);
    return NextResponse.json({ error: "Internal Server Error" }, { status: 500 });
  }
}
