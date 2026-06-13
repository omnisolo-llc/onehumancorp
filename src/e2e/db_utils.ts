import { Pool } from 'pg';

const pool = new Pool({
  connectionString: process.env.DATABASE_URL || 'postgres://ohc:ohc@localhost:5432/ohc',
});

export async function e2eDbQuery(query: string, values?: any[]) {
  const client = await pool.connect();
  try {
    const res = await client.query(query, values);
    return res.rows;
  } finally {
    client.release();
  }
}
