
import { test, expect } from '@playwright/test';

test.describe('Local Stateful Execution Proxy Integration Flow', () => {
  test('E2E integration configuration and activation flow', async ({ page }) => {
    // 1. Start from the home page after login via the UI
    await page.goto('/');

    // 2. Navigate to integrations
    // We mock the network response to ensure the integration shows up since it's just in-memory catalog
    await page.route('**/api/integrations', route => {
        route.fulfill({
            status: 200,
            contentType: 'application/json',
            body: JSON.stringify([{
                id: 'local-stateful-execution-proxy',
                name: 'Local Stateful Execution Proxy',
                type: 'local_stateful_execution_proxy',
                category: 'system',
                description: 'A bridge that allows cloud agents to execute commands locally on a user\'s machine.',
                status: 'CONNECTED'
            }])
        });
    });

    await page.goto('/settings/integrations');

    const proxyIntegrationCard = page.locator('text=Local Stateful Execution Proxy').first();
    await expect(proxyIntegrationCard).toBeVisible();
    await proxyIntegrationCard.click();

    // Verify it shows connected
    const statusText = page.locator('text=CONNECTED').first();
    await expect(statusText).toBeVisible();
    await expect(page.locator('text=A bridge that allows cloud agents to execute commands locally')).toBeVisible();
  });
});
