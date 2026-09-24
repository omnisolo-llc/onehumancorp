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

export async function executeSql(sql: string, values?: unknown[]) {
  return e2eDbQuery(sql, values);
}

export async function setupTestTenant(name: string): Promise<string> {
  const tenantId = `tenant-${name}-${Date.now()}`;
  await e2eDbQuery(
    `INSERT INTO tenants (id, name, industry, tier, plan_tier, has_claimed_trial_extension)
     VALUES ($1, $2, 'Food and beverage', 'Free', 'Free', false)
     ON CONFLICT (id) DO NOTHING`,
    [tenantId, name]
  );
  return tenantId;
}

export async function cleanupTestTenant(tenantId: string): Promise<void> {
  if (!tenantId) return;
  try {
    await e2eDbQuery(`DELETE FROM ohc_job_queue WHERE tenant_id = $1`, [tenantId]);
    await e2eDbQuery(`DELETE FROM subscribers WHERE tenant_id = $1`, [tenantId]);
    await e2eDbQuery(`DELETE FROM subscription_plans WHERE tenant_id = $1`, [tenantId]);
    await e2eDbQuery(`DELETE FROM agent_action_requests WHERE tenant_id = $1`, [tenantId]);
    await e2eDbQuery(`DELETE FROM tenants WHERE id = $1`, [tenantId]);
  } catch {
    // Ignore cleanup error
  }
}
