import { test, expect } from '@playwright/test';

test.describe('Viral Loyalty Widget', () => {
  test('should load the widget and generate a loyalty program', async ({ page }) => {
    // Start local http server to serve the page because Docker is not available in sandbox
    const fs = require('fs');
    const path = require('path');
    const tauriUiDir = path.join(process.cwd(), 'src/ui/tauri/src/ui');

    // Serve the HTML file directly through Playwright routing
    await page.route('**/*viral-loyalty-widget.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'viral-loyalty-widget.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });
    await page.route('**/api/v1/growth/referrals/generate', async route => {
      await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ referral_link: 'http://example.com/ref/12345' }) });
    });

    await page.goto('http://mock/viral-loyalty-widget.html');



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

    // Verify filled stamps
    const filledStamps = page.locator('.stamp.filled');
    await expect(filledStamps).toHaveCount(4);

    // Check share link generated correctly
    const shareLink = page.locator('#share-link');
    await expect(shareLink).toHaveValue(/loyalty\/join\?ref=12345/);
  });
});
