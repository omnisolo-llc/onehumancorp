import { test, expect } from './fixtures';

test.describe('Viral Receipt Lottery Generator', () => {
  test('should load the generator and generate a lottery link', async ({ page }) => {
    // 1. We mock the backend response specifically because this is a static UI page
    // in the tauri bundle that simulates growth mechanics.
    await page.route('/api/v1/growth/referrals/generate', async route => {
      await route.fulfill({ json: { referral_link: 'http://example.com/ref/lottery-test-123' } });
    });

    // 2. Navigate to dashboard and click the new link
    await page.goto('/ui/dashboard.html');
    const link = page.locator('#viral-receipt-lottery-link');
    await expect(link).toBeVisible();
    await link.click();

    // 3. Wait for main elements
    await expect(page.locator('h1')).toHaveText('Viral Receipt Lottery 🎟');
    const generateBtn = page.locator('#generate-btn');
    await expect(generateBtn).toBeVisible();

    // 4. Click generate
    await generateBtn.click();

    // 5. Verify button state
    await expect(generateBtn).toBeDisabled();
    await expect(generateBtn).toHaveText('Generating...');

    // 6. Wait for the result to show
    const resultArea = page.locator('#result-area');
    await expect(resultArea).toBeVisible({ timeout: 5000 });

    // 7. Check share link generated correctly
    const shareLink = page.locator('#share-link');
    await expect(shareLink).toHaveValue(/win\/lottery-test-123/);

    // 8. Check that preview URL updated
    const previewUrl = page.locator('#preview-url');
    await expect(previewUrl).toHaveText('ohc.app/win/lottery-test-123');
  });
});
