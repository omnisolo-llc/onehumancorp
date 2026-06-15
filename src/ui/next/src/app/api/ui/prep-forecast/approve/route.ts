import { NextResponse } from "next/server";
import { Pool } from "pg";
import { v4 as uuidv4 } from "uuid";

const pool = new Pool({
  connectionString: process.env.DATABASE_URL || "postgres://postgres:postgres@localhost:5432/ohc",
});

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const { tenant_id, prediction_id, product_id, quantity } = body;

    if (!tenant_id || !prediction_id || !product_id) {
      return NextResponse.json({ error: "Missing required fields" }, { status: 400 });
    }

    const client = await pool.connect();
    try {
      await client.query("BEGIN");
      await client.query("SELECT set_config('app.current_tenant', $1, false)", [tenant_id]);

      // Remove the prediction from the view once approved
      await client.query(
        "DELETE FROM inventory_predictions WHERE id = $1 AND tenant_id = $2",
        [prediction_id, tenant_id]
      );

      // In a real system, this would queue a job for staff tasks or supply order
      // For now, let's update inventory to reflect prep or just create a task
      // We'll queue a task for the team
      const jobId = uuidv4();
      await client.query(
        `INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status)
         VALUES ($1, $2, 'create_prep_task', $3, 'PENDING')`,
        [
          jobId,
          tenant_id,
          JSON.stringify({
            product_id,
            quantity,
            action: "approve_prep_plan"
          })
        ]
      );

      await client.query("COMMIT");

      return NextResponse.json({ success: true });
    } catch (e) {
      await client.query("ROLLBACK");
      throw e;
    } finally {
      client.release();
    }
  } catch (error) {
    console.error("Prep forecast approval API error:", error);
    return NextResponse.json({ error: "Internal Server Error" }, { status: 500 });
  }
}
