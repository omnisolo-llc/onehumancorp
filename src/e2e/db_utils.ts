import { Pool } from 'pg';

if (!process.env.DATABASE_URL) {
    throw new Error('DATABASE_URL is not set in the environment. Tests must run with a valid database.');
}

const pool = new Pool({
  connectionString: process.env.DATABASE_URL,
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

export const db = {
  query: e2eDbQuery,
};
