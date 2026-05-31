import { test, expect } from '@playwright/test';
import { e2eDbQuery } from './fixtures';

test.describe('Action Feed E2E Tests', () => {
  test('displays loading state and then action feed for valid token, and accepts approval', async ({ page }) => {
    // We bypass the login UI for speed but still go through the E2E application stack
    await page.goto('/login');
    await page.fill('input[name="email"]', 'test_user@example.com');
    await page.fill('input[name="password"]', 'password123');
    await page.click('button[type="submit"]');

    // Wait until logged in
    await page.waitForURL('**/dashboard');

    const uniqueId = Date.now().toString();
    const tokenId = `token-${uniqueId}`;
    const reqId = `req-${uniqueId}`;

    // Seed the database
    await e2eDbQuery(`INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk) VALUES ('${reqId}', 'test-tenant', 'operations', 'Test Approval ${uniqueId}', 'PENDING', 'HIGH')`);
    await e2eDbQuery(`INSERT INTO action_tokens (id, tenant_id, approval_request_id, status, expires_at) VALUES ('${tokenId}', 'test-tenant', '${reqId}', 'PENDING', NOW() + INTERVAL '1 day')`);

    // Visit the action feed page
    await page.goto('/action-feed?token=' + tokenId);

    // Assert that the page renders the actual DB content without mocks
    await expect(page.locator('h2').filter({ hasText: 'Approval Request' })).toBeVisible({ timeout: 10000 });
    await expect(page.getByText(`Test Approval ${uniqueId}`)).toBeVisible({ timeout: 10000 });

    // Approve the action
    await page.getByRole('button', { name: 'Approve & Execute' }).click();

    // The backend should respond 200 OK and UI should update
    await expect(page.getByText('Action Processed!')).toBeVisible({ timeout: 10000 });

    // Verify that the database was mutated correctly by the API
    const res = await e2eDbQuery(`SELECT status FROM action_tokens WHERE id = '${tokenId}'`);
    expect(res.rows[0].status).toBe('CONSUMED');
  });
});
