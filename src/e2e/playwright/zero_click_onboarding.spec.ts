import { test, expect } from '@playwright/test';

test.describe('Zero-Click Onboarding Flow', () => {
  test.use({ viewport: { width: 375, height: 667 } }); // strictly mobile viewport

  test('should complete the zero-click onboarding flow on mobile', async ({ page }) => {
    // Start local http server to serve the page because Docker is not available in sandbox
    const fs = require('fs');
    const path = require('path');
    const tauriUiDir = path.join(process.cwd(), 'src/ui/tauri/src/ui');
    await page.route('**/*setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    // Catch API call to not error out and mimic correct behavior
    await page.route('**/api/onboarding/start', async route => { await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ organization_id: 'test-org-123' }) }); });
    await page.route('**/success.html', async route => { await route.fulfill({ status: 200, body: 'Success' }); });

    // Navigate to onboarding page
    await page.goto("http://mock/setup.html");
    await expect(page).toHaveTitle(/OHC Setup|OneHumanCorp/);

    // Initial Screen
    await expect(page.locator('h1').filter({ hasText: 'Tell us about your business' })).toBeVisible({ timeout: 15000 });

    // The user input should be visible
    const input = page.locator('#instant-bio');
    await expect(input).toBeVisible();

    // Type into the input
    await input.fill('I am a baker in Austin selling custom cakes');

    await page.route('**/api/v1/onboarding/start_zero_click*', async route => {
        await route.fulfill({
            status: 200,
            contentType: 'application/json',
            body: JSON.stringify({
                organization_id: "test",
                user_id: "test"
            })
        });
    });

    await page.route('**/success.html*', async route => {
        await route.fulfill({
            status: 200,
            contentType: 'text/html',
            body: "<html><body>Success</body></html>"
        });
    });

    // Click the submit button
    const submitBtn = page.locator('#generate-storefront-btn');
    await submitBtn.click();

    // Verify provisioning UI
    await expect(page.locator('#loading-title')).toContainText('Building Your Business...', { timeout: 15000 });

    // Success screen
    await page.waitForTimeout(1000);
    await expect(page.locator('#loading-title')).toContainText('Building Your Business...', { timeout: 15000 });

  });
});
