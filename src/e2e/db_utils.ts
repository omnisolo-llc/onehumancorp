import { Pool } from "pg";

// Importing a browser spec must not connect to a database or prevent --list.
// Require the isolated test database at the first actual query, not discovery.
let pool: Pool | undefined;
function testPool(): Pool {
  if (pool) return pool;
  const databaseUrl = process.env.DATABASE_URL;
  if (!databaseUrl) throw new Error("DATABASE_URL is required for database-backed E2E tests.");
  pool = new Pool({ connectionString: databaseUrl, connectionTimeoutMillis: 5000, idleTimeoutMillis: 1000 });
  return pool;
}

export async function e2eDbQuery(query: string, values?: unknown[]) {
  const client = await testPool().connect();
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
