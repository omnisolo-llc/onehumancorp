import { test, expect } from '@playwright/test';

test.describe('AI Agent Department Architecture', () => {
  test('should display approval inbox and activity feed', async ({ page }) => {
    // Navigate to agents page
    await page.goto('http://localhost:3000/agents');

    // Ensure "My Team" tab is visible
    await expect(page.locator('text=My Team')).toBeVisible();
    await expect(page.locator('text=The Manager')).toBeVisible();

    // Navigate to "Activity Feed" tab
    await page.locator('text=Activity Feed').click();
    await expect(page.locator('text=Fetching feed...').or(page.locator('text=No activity yet.'))).toBeVisible();

    // Navigate to "Needs Approval" tab
    await page.locator('text=Needs Approval').click();
    await expect(page.locator('text=Fetching approvals...').or(page.locator('text=All Caught Up!'))).toBeVisible();
  });
});
