import { test as baseTest, expect } from '@playwright/test';
import * as fs from 'fs';
import * as path from 'path';

// We use the base playwright test since the e2e fixtures block network mocking,
// and this specific file relies on a static HTML file that is injected via routing.
baseTest.describe('Viral Loyalty Widget', () => {
  baseTest('should load the widget and generate a loyalty program', async ({ page }) => {

    await page.route('**/ui/viral-loyalty-widget.html', async route => {
      const htmlContent = fs.readFileSync(path.join(process.cwd(), 'src/ui/tauri/src/ui/viral-loyalty-widget.html'), 'utf-8');
      await route.fulfill({ contentType: 'text/html', body: htmlContent });
    });

    // Mock the backend API call because the server is not actually running when using npx playwright directly.
    await page.route('**/api/v1/growth/referrals/generate', async route => {
      await new Promise(resolve => setTimeout(resolve, 500));
      await route.fulfill({ json: { referral_link: 'https://ohc.app/ref/mock-uuid-1234' } });
    });

    // The frontend code expects localStorage to have the tenant id to pass x-spiffe-id header
    // We navigate to a mock domain to set local storage, since we're mocking the html and api anyway
    await page.route('http://mock-domain.local/setup', async route => {
        await route.fulfill({ contentType: 'text/html', body: '<html><body>setup</body></html>' });
    });
    await page.goto('http://mock-domain.local/setup');
    await page.evaluate(() => {
        localStorage.setItem('tenant_id', 'e2e-tenant');
    });

    await page.goto('http://mock-domain.local/ui/viral-loyalty-widget.html');

    // Wait for main elements
    await expect(page.locator('h1')).toHaveText('Viral Loyalty Widget Generator');
    const generateBtn = page.locator('#generate-btn');
    await expect(generateBtn).toBeVisible();

    // Check initial stamps state
    const emptyStamps = page.locator('.stamp.empty');
    await expect(emptyStamps).toHaveCount(4);

    // Click generate
    await generateBtn.click();

    // Verify animation starts
    await expect(generateBtn).toBeDisabled();
    await expect(generateBtn).toHaveText('Generating...');

    // Wait for the animation to finish and result to show
    const resultArea = page.locator('#result-area');
    await expect(resultArea).toBeVisible({ timeout: 5000 });

    // the button should be re-enabled after generation
    await expect(generateBtn).toBeEnabled();
    await expect(generateBtn).toHaveText('Generate Loyalty Program');

    // Verify filled stamps
    const filledStamps = page.locator('.stamp.filled');
    await expect(filledStamps).toHaveCount(4);

    // Check share link generated correctly.
    const shareLink = page.locator('#share-link');
    await expect(shareLink).toHaveValue(/loyalty\/join\?ref=mock-uuid-1234/);
  });
});
