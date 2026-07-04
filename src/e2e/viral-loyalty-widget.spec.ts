import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';
import * as fs from 'fs';
import * as path from 'path';

test.describe('Viral Loyalty Widget', () => {
  test('should load the widget and generate a loyalty program', async ({ page }) => {
    // We mock the backend response here specifically because this is a static UI page
    // in the tauri bundle that simulates growth mechanics.
    await page.route('**/api/v1/growth/referrals/generate', async route => {
      await route.fulfill({ json: { referral_link: 'http://example.com/ref/12345' } });
    });

    const fileContent = fs.readFileSync(path.resolve(__dirname, '../ui/tauri/src/ui/viral-loyalty-widget.html'), 'utf8');
    await page.route('http://example.com/ui/viral-loyalty-widget.html', async route => {
      await route.fulfill({ contentType: 'text/html', body: fileContent });
    });

    await page.goto('http://example.com/ui/viral-loyalty-widget.html');

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