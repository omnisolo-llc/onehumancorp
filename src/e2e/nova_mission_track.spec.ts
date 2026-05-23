import { test, expect } from '@playwright/test';

test.describe('Nova Mission Track', () => {
  test('should display the Nova Mission Track dashboard and filter correctly', async ({ page }) => {
    await page.goto('/nova-mission-track');

    // Check header
    await expect(page.locator('h1')).toContainText('Mission Control');
    await expect(page.locator('text=Nova Track')).toBeVisible();

    // Check all missions are visible initially
    await expect(page.locator('text=Dynamic Organization Generation')).toBeVisible();
    await expect(page.locator('text=Apply Design Tokens')).toBeVisible();
    await expect(page.locator('text=Multimodal LLM Endpoints')).toBeVisible();

    // Filter by Active
    await page.click('button:has-text("active")');
    await expect(page.locator('text=Multimodal LLM Endpoints')).toBeVisible();
    await expect(page.locator('text=Dynamic Organization Generation')).not.toBeVisible();

    // Filter by Completed
    await page.click('button:has-text("completed")');
    await expect(page.locator('text=Dynamic Organization Generation')).toBeVisible();
    await expect(page.locator('text=Multimodal LLM Endpoints')).not.toBeVisible();
  });
});
