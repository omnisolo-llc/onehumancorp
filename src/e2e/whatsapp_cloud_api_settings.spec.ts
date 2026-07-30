import { test, expect } from '@playwright/test';
import { setupTestEnv, teardownTestEnv, loginAsE2eTenant } from './test_utils';

test.describe('WhatsApp Cloud API Integrations Setting', () => {
  test.beforeAll(async () => {
    await setupTestEnv();
  });

  test.afterAll(async () => {
    await teardownTestEnv();
  });

  test.beforeEach(async ({ page }) => {
    await loginAsE2eTenant(page);
  });

  test('Owner can navigate to Settings -> Integrations and see WhatsApp Cloud API', async ({ page }) => {
    // 1. Navigate to Settings -> Integrations
    await page.goto('/settings/integrations');

    // 2. Wait for page load
    await expect(page.locator('h1:has-text("App Integrations")')).toBeVisible();

    // 3. Find the WhatsApp Cloud API integration card
    const waCloudCard = page.locator('h3:has-text("WhatsApp Cloud API")').locator('..');

    // 4. Verify its presence
    await expect(waCloudCard).toBeVisible();
    await expect(waCloudCard.locator('p:has-text("Direct WhatsApp Cloud API connection for messages")')).toBeVisible();

    // 5. Click the "Connect" button
    const connectButton = waCloudCard.locator('button:has-text("Connect")');
    await expect(connectButton).toBeVisible();
    await connectButton.click();

    // 6. Verify the modal opens and displays properly
    const modalHeading = page.locator('h2:has-text("Connect WhatsApp Cloud API")');
    await expect(modalHeading).toBeVisible();

    // 7. Verify the Meta 'Continue with Facebook' button
    const metaButton = page.locator('button:has-text("Continue with Meta")');
    await expect(metaButton).toBeVisible();

    // 8. Close the modal
    const closeBtn = page.locator('button[aria-label="Close modal"]').first();
    if(await closeBtn.isVisible()) {
      await closeBtn.click();
      await expect(modalHeading).not.toBeVisible();
    }
  });
});
