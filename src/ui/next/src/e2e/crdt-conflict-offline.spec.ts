import { test, expect } from '../../../../e2e/fixtures';

test.describe('Offline-Tolerant Sync Conflict Resolution CUJ', () => {
  test('A sync conflict triggers an operations task for AI resolution', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);

    // Navigate to a page to establish a session and get a context
    await page.goto('/');

    const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'tenant-1');
    const spiffeId = `spiffe://ohc/org/${tenantId}/agent/ui`;
    const deltaId = `delta-${Date.now()}`;
    const entityId = `entity-${Date.now()}`;

    // Force a conflict by triggering an offline MCP delta sync that simulates a version/condition mismatch in the database
    // We send a delta where the `updated_at` logic causes `rows_affected == 0` during upsert because of outdated timestamps

    // First, insert a baseline delta with a very futuristic timestamp
    const resBase = await page.request.post('/api/v1/sync/mcp-deltas', {
      headers: {
        'x-spiffe-id': spiffeId
      },
      data: {
        deltas: [
          {
            id: deltaId,
            entity_id: entityId,
            data: JSON.stringify({ name: "baseline" }),
            updated_at: Date.now() + 100000 // Future timestamp
          }
        ]
      }
    });
    expect(resBase.ok()).toBeTruthy();

    // Now, simulate the offline client syncing a "stale" delta, which should trigger the conflict condition
    const resConflict = await page.request.post('/api/v1/sync/mcp-deltas', {
        headers: {
          'x-spiffe-id': spiffeId
        },
        data: {
          deltas: [
            {
              id: deltaId,
              entity_id: entityId,
              data: JSON.stringify({ name: "offline_change" }),
              updated_at: Date.now() - 100000 // Past timestamp
            }
          ]
        }
    });
    expect(resConflict.ok()).toBeTruthy();

    // The backend should have queued a task in department_tasks for the operations agent.
    // Let's verify it shows up in the Action Center.
    await page.waitForTimeout(5000); // Give the agent/queue time to process

    await page.goto('/action-center');

    // Wait for Action Center to load
    await expect(page.locator('h1', { hasText: 'Action Center' })).toBeVisible({ timeout: 10000 });

    // Verify the conflict task shows up
    await expect(page.getByText(`Sync conflict detected for CRDT delta ${deltaId} on entity ${entityId}`)).toBeVisible({ timeout: 10000 });
  });
});
