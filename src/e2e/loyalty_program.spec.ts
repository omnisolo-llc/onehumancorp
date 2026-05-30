import { test, expect } from '@playwright/test';
import { adminPage as pageFixture } from './fixtures';

test.describe('Customer Loyalty Program Generator', () => {
  test('should open loyalty program modal, generate offer, and show copy button', async ({ page }) => {
    // 1. Navigate to dashboard
    await page.goto('/dashboard');
    await expect(page.locator('h1').filter({ hasText: 'Overview' })).toBeVisible({ timeout: 10000 });

    // 2. Find and click the "Create Loyalty Program" button
    const createButton = page.locator('button:has-text("Create Loyalty Program")');
    await expect(createButton).toBeVisible();
    await createButton.click();

    // 3. Verify the modal opens
    const modalHeading = page.locator('h2:has-text("Loyalty Program")');
    await expect(modalHeading).toBeVisible();

    // 4. Click generate
    const generateButton = page.locator('button:has-text("Generate Loyalty Program")');
    await expect(generateButton).toBeVisible();

    // We mock the API route to ensure stable tests
    await page.route('/api/v1/growth/loyalty-program/generate', async (route) => {
      const json = { result: "Join our loyalty program! Buy 5 items and get a free coffee." };
      await route.fulfill({ json });
    });

    await generateButton.click();

    // 5. Verify the generated text appears
    await expect(page.locator('p:has-text("Join our loyalty program! Buy 5 items and get a free coffee.")')).toBeVisible();

    // 6. Verify the copy button appears
    const copyButton = page.locator('button:has-text("Copy Promotional Message")');
    await expect(copyButton).toBeVisible();
  });
});
