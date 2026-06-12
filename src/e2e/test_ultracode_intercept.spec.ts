import { test, expect } from '@playwright/test';

test.describe('Ultracode Intercept E2E', () => {
  test('should intercept ultracode trigger and generate dynamic workflow script', async ({ page }) => {
    // Navigate to the assistant chat
    await page.goto('/assistant');

    // Wait for chat input
    const chatInput = page.locator('textarea[placeholder="Ask assistant..."]');
    await expect(chatInput).toBeVisible();

    // Type the ultracode trigger
    await chatInput.fill('Please use a workflow to audit all API endpoints');

    // Submit
    await page.locator('button[aria-label="Send message"]').click();

    // The backend should intercept "use a workflow" and generate a script response
    await expect(page.locator('text=Generated dynamic workflow script')).toBeVisible({ timeout: 15000 });
    await expect(page.locator('text=Workflow executed successfully for: Please use a workflow to audit all API endpoints')).toBeVisible({ timeout: 15000 });
  });

  test('should intercept literal ultracode keyword', async ({ page }) => {
    // Navigate to the assistant chat
    await page.goto('/assistant');

    // Wait for chat input
    const chatInput = page.locator('textarea[placeholder="Ask assistant..."]');
    await expect(chatInput).toBeVisible();

    // Type the ultracode trigger
    await chatInput.fill('ultracode: migrate the database');

    // Submit
    await page.locator('button[aria-label="Send message"]').click();

    // The backend should intercept "ultracode" and generate a script response
    await expect(page.locator('text=Generated dynamic workflow script')).toBeVisible({ timeout: 15000 });
    await expect(page.locator('text=Workflow executed successfully for: ultracode: migrate the database')).toBeVisible({ timeout: 15000 });
  });
});
