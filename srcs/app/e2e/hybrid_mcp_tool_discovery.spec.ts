import { test, expect } from '@playwright/test';

test.use({ baseURL: 'http://localhost:8000' });

test.describe('Hybrid MCP Tool Discovery E2E', () => {
  test('Local tool synchronizes, displays, and triggers invocation UI', async ({ page }) => {
    // 1. Login
    await page.goto('/');
    await page.fill('input[name="username"]', 'admin');
    await page.fill('input[name="password"]', 'admin');
    await page.click('button:has-text("Login")');

    // 2. Wait for login and navigate to Integrations
    await page.waitForTimeout(1000); // Wait for Flutter canvas to render
    await page.goto('/#/integrations');

    // 3. Verify that the Integrations screen is loaded
    await expect(page.locator('text=Integrations & MCP Tools')).toBeVisible();

    // 4. Verify that the synchronized local MCP tool (from discovery mock/sqlite) is listed
    await expect(page.locator('text=local-calculator')).toBeVisible();

    // 5. Verify the description for the local tool is present
    await expect(page.locator('text=A local calculator tool')).toBeVisible();

    // 6. Test Remote Execution presence by clicking Invoke inside the ListTile
    // The Integrations UI builds a GlassCard with a ListTile trailing OutlinedButton containing 'Invoke'
    const invokeBtn = page.locator('text=local-calculator').locator('..').locator('button:has-text("Invoke")');
    await expect(invokeBtn).toBeVisible();
    await invokeBtn.click();

    // Check that there isn't an execution error presented
    await expect(page.locator('text=error')).not.toBeVisible();
  });
});
