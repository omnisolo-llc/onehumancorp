import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard Flow', () => {


  test.beforeEach(async ({ page }) => {
    const fs = require('fs');
    const path = require('path');
    const tauriUiDir = path.join(process.cwd(), 'src/ui/tauri/src/ui');
    await page.route('**/setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });
    await page.goto('http://mock/setup.html');
  });

  test('successfully completes the wizard with drafting and instant image url', async ({ page }) => {
    await expect(page.locator('body')).toContainText('10-Minute Setup Wizard');

    // Sometimes there might be a loading transition, wait for the Instant Build button to be ready
    await page.waitForTimeout(2000);

    // There are multiple ways to click Instant Build, we added ID "instant-build-btn-text"
    await page.getByText('Instant Build').click();

    const bioInput = page.locator('#instant-bio');
    await expect(bioInput).toBeVisible();

    await bioInput.fill('My E2E Bakery');

    const imageUrlInput = page.locator('#instant-image-url');
    await expect(imageUrlInput).toBeVisible();
    await imageUrlInput.fill('https://example.com/bakery.png');

    await page.locator('#generate-storefront-btn').click();

    await expect(page.locator('#loading-title')).toContainText('Building Your Business...', { timeout: 15000 });
  });
});
