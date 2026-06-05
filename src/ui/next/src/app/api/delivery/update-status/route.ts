import { NextResponse } from "next/server";
import { headers } from "next/headers";
import { Pool } from "pg";

const pool = new Pool({
  connectionString: process.env.DATABASE_URL,
});

export async function POST(request: Request) {
  try {
    const { taskId, status } = await request.json();
    if (!taskId || !status) {
      return NextResponse.json({ error: "taskId and status are required" }, { status: 400 });
    }

    const headersList = headers();
    const tenantId = headersList.get("x-tenant-id") || "default_tenant";

    const client = await pool.connect();
    const result = await client.query(
      `UPDATE delivery_tasks
       SET status = $1, updated_at = CURRENT_TIMESTAMP
       WHERE id = $2 AND organization_id = $3
       RETURNING *`,
      [status, taskId, tenantId]
    );
    client.release();

    if (result.rowCount === 0) {
      return NextResponse.json({ error: "Task not found" }, { status: 404 });
    }

    return NextResponse.json({ task: result.rows[0] });
  } catch (error) {
    console.error("Error updating task status:", error);
    return NextResponse.json({ error: "Failed to update status" }, { status: 500 });
  }
}
