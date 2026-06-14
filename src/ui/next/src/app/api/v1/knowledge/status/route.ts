import { NextResponse } from 'next/server';
import { Pool } from 'pg';

const pool = new Pool({
  connectionString: process.env.OHC_DATABASE_URL || 'postgres://postgres:postgres@localhost:5432/ohc',
});

export async function GET(req: Request) {
  const { searchParams } = new URL(req.url);
  const ids = searchParams.get('ids')?.split(',') || [];

  if (ids.length === 0) {
    return NextResponse.json({ statuses: {} });
  }

  const query = `
    SELECT memory_id, sync_status
    FROM swarm_truth_embeddings
    WHERE memory_id = ANY($1::text[])
  `;

  const { rows } = await pool.query(query, [ids]);

  const statuses: Record<string, string> = {};
  rows.forEach(row => {
    statuses[row.memory_id] = row.sync_status;
  });

  return NextResponse.json({ statuses });
}
