import { test, expect } from '@playwright/test';

test('Goose MCP Extensions UI End-to-End', async ({ page }) => {
  // Navigate to the test page
  await page.goto('/goose-mcp');

  // Check the title
  await expect(page.locator('h1')).toHaveText('Goose MCP Extensions UI');

  // Verify the extension loads
  // Since our fake extension id is "sample_mcp"
  const extLocator = page.locator('#extension-sample_mcp');
  await expect(extLocator).toBeVisible({ timeout: 10000 });

  // Click execute button
  const execButton = page.locator('#execute-sample_mcp');
  await execButton.click();

  // Verify execution result output contains our expected string
  const resultLocator = page.locator('#exec-result');
  await expect(resultLocator).toContainText('hello from UI', { timeout: 10000 });
});
