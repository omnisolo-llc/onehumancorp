import { e2eDbTransaction } from '../db_utils';

// Read only receipt identities and states, never provider credentials. Rejected
// verification requests must leave both existing and absent connections intact.
export async function integrationStorage(tenantId: string, providers: string[]) {
  return e2eDbTransaction(async query => {
    await query('SET TRANSACTION ISOLATION LEVEL REPEATABLE READ, READ ONLY');
    await query("SELECT set_config('app.current_tenant',$1,true)", [tenantId]);
    // The encrypted vault is initialized lazily by supported connection flows.
    // Legacy plaintext storage is absent from the maintained PostgreSQL schema.
    // Capture absence explicitly so an unexpected CREATE also changes the snapshot.
    // Do not catch query errors: permissions, schema drift and DB failures must fail.
    const [schema] = await query(
      "SELECT to_regclass('integration_credentials') IS NOT NULL AS legacy_exists, to_regclass('ohc_provider_connections') IS NOT NULL AS vault_exists",
    );
    if (typeof schema?.legacy_exists !== 'boolean' || typeof schema?.vault_exists !== 'boolean') {
      throw new Error('Integration storage schema inspection returned no valid result');
    }
    return {
      connections: await query(
        'SELECT id, name, status FROM tool_integrations WHERE tenant_id=$1 AND (id=ANY($2::text[]) OR name=ANY($2::text[])) ORDER BY id',
        [tenantId, providers],
      ),
      vaultConnections: {
        exists: schema.vault_exists,
        rows: schema.vault_exists ? await query(
          'SELECT provider, revision, state, verified_at FROM ohc_provider_connections WHERE tenant_id=$1 AND provider=ANY($2::text[]) ORDER BY provider',
          [tenantId, providers],
        ) : [],
      },
      legacyCredentials: {
        exists: schema.legacy_exists,
        rows: schema.legacy_exists ? await query(
          'SELECT id, integration_id, updated_at FROM integration_credentials WHERE tenant_id=$1 AND integration_id=ANY($2::text[]) ORDER BY id',
          [tenantId, providers],
        ) : [],
      },
    };
  });
}
