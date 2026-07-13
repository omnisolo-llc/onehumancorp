import { test, expect } from '@playwright/test';

test.describe('Zero-Click Onboarding Flow', () => {
  test.use({ viewport: { width: 375, height: 667 } }); // strictly mobile viewport

  test.beforeEach(async ({ page }) => {
    const fs = require('fs');
    const path = require('path');
    const tauriUiDir = path.join(process.cwd(), 'src/ui/tauri/src/ui');
    await page.route('**/setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });
  });

  test('should complete the zero-click onboarding flow on mobile', async ({ page }) => {
    // Navigate to the real local server
    await page.goto('http://mock/setup.html');
    await expect(page).toHaveTitle(/OneHumanCorp|OHC/);

    // Initial Screen
    await expect(page.locator('h1', { hasText: 'Tell us about your business' })).toBeVisible({ timeout: 15000 });

    // Check if the input loaded
    await expect(page.locator('#instant-bio')).toBeVisible();

    // Type into the input
    await page.locator('#instant-bio').fill('I am a baker in Austin selling custom cakes');

    // Ensure we can see the generate storefront button
    await expect(page.locator('#generate-storefront-btn')).toBeVisible({ timeout: 15000 });
  });
});
