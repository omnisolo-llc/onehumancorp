import { Pool } from "pg";

const databaseUrl = process.env.DATABASE_URL;
if (!databaseUrl) {
  throw new Error("DATABASE_URL is required for database-backed E2E tests.");
}

const pool = new Pool({ connectionString: databaseUrl });

export async function e2eDbQuery(query: string, values?: unknown[]) {
  const client = await pool.connect();
  try {
    const result = await client.query(query, values);
    return result.rows;
  } finally {
    client.release();
  }
}

export const db = {
  query: e2eDbQuery,
};
