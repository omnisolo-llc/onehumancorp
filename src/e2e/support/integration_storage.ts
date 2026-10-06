import { e2eDbQuery } from '../db_utils';

// Read only receipt identities and states, never provider credentials. Rejected
// verification requests must leave both existing and absent connections intact.
export async function integrationStorage(tenantId: string, providers: string[]) {
  return {
    connections: await e2eDbQuery(
      'SELECT id, name, status FROM tool_integrations WHERE tenant_id=$1 AND name=ANY($2::text[]) ORDER BY id',
      [tenantId, providers],
    ),
    credentials: await e2eDbQuery(
      'SELECT id, integration_id FROM integration_credentials WHERE tenant_id=$1 AND integration_id=ANY($2::text[]) ORDER BY id',
      [tenantId, providers],
    ),
  };
}
