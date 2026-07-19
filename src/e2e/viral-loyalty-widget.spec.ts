import { test, expect } from '@playwright/test';
import * as path from 'path';
import * as fs from 'fs';

test.describe('Viral Loyalty Widget', () => {
  test.beforeEach(async ({ page }) => {
    // Clear local storage to ensure fresh state
    await page.addInitScript(() => {
      window.localStorage.clear();
    });

    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR || process.cwd(), process.env.TEST_WORKSPACE)
        : process.cwd();
    const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');

    // Serve the static HTML file
    await page.route('**/viral-loyalty-widget.html', async route => {
        const fileContent = fs.readFileSync(path.join(tauriUiDir, 'viral-loyalty-widget.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: fileContent });
    });
  });

  test('should load the widget and generate a loyalty program', async ({ page }) => {
    // We mock the backend response here specifically because this is a static UI page
    // in the tauri bundle that simulates growth mechanics.
    await page.route('/api/v1/growth/referrals/generate', async route => {
      await route.fulfill({ json: { referral_link: 'http://example.com/ref/12345' } });
    });

    await page.goto('/viral-loyalty-widget.html');

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
