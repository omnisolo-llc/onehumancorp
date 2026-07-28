import { test, expect } from '@playwright/test';
import { setupTestEnv, teardownTestEnv, loginAsE2eTenant } from './fixtures';

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
    await page.goto('/settings/integrations');
    await expect(page.locator('h1:has-text("App Integrations")')).toBeVisible();

    const waCloudCard = page.locator('h3:has-text("WhatsApp Cloud API")').locator('..');
    await expect(waCloudCard).toBeVisible();
    await expect(waCloudCard.locator('p:has-text("Direct WhatsApp Cloud API connection for messages")')).toBeVisible();

    const connectButton = waCloudCard.locator('button:has-text("Connect")');
    await expect(connectButton).toBeVisible();
    await connectButton.click();

    const modalHeading = page.locator('h2:has-text("Connect WhatsApp Cloud API")');
    await expect(modalHeading).toBeVisible();

    const metaButton = page.locator('button:has-text("Continue with Meta")');
    await expect(metaButton).toBeVisible();

    const closeBtn = page.locator('button[aria-label="Close modal"]').first();
    if(await closeBtn.isVisible()) {
      await closeBtn.click();
      await expect(modalHeading).not.toBeVisible();
    }
  });
});
