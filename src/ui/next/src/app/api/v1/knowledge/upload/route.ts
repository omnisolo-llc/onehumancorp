import { NextResponse } from 'next/server';
import { v4 as uuidv4 } from 'uuid';
import { Pool } from 'pg';

const pool = new Pool({
  connectionString: process.env.OHC_DATABASE_URL || 'postgres://postgres:postgres@localhost:5432/ohc',
});

export async function POST(req: Request) {
  // Respect real multitenancy isolation via x-tenant-id header (falling back to a default for local dev if needed)
  const tenant_id = req.headers.get('x-tenant-id') || 'tenant_test_1';

  const formData = await req.formData();
  const files = formData.getAll('files') as File[];

  const documents = [];

  for (const file of files) {
    const memory_id = uuidv4();
    const context = file.name; // Simulating parsed text context

    // Create record in swarm_truth_embeddings
    await pool.query(
      `INSERT INTO swarm_truth_embeddings (memory_id, tenant_id, context, sync_status, last_sync_at)
       VALUES ($1, $2, $3, 'pending', NULL)`,
      [memory_id, tenant_id, context]
    );

    // Enqueue job in ohc_job_queue
    const job_id = uuidv4();
    const payload = JSON.stringify({ document_id: memory_id });
    await pool.query(
      `INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, retry_count, next_retry_at, created_at, updated_at)
       VALUES ($1, $2, 'rag_sync', $3, 'PENDING', 0, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)`,
      [job_id, tenant_id, payload]
    );

    documents.push({
      id: memory_id,
      name: file.name,
      sync_status: 'pending',
    });
  }

  return NextResponse.json({ documents });
}
