import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Viral Loyalty Widget', () => {
  test('should load the widget and generate a loyalty program', async ({ page }) => {
    // Use the adminPage fixture to run the CUJ properly (from dashboard)
    await adminPage(page, async () => {
      // Start from dashboard and click the widget link
      await page.goto('/ui/dashboard.html');
      await page.locator('#loyalty-link').click();

      // Wait for main elements
      await expect(page.locator('h1')).toHaveText('Viral Loyalty Widget Generator');

      const cardPreview = page.locator('.card-preview');
      await expect(cardPreview).toBeVisible();
      // Assert the specific styling of card-preview matches constraints
      await expect(cardPreview).toHaveCSS('background-color', 'rgba(255, 255, 255, 0.65)');

      const generateBtn = page.locator('#generate-btn');
      await expect(generateBtn).toBeVisible();

      // Check initial stamps state
      const emptyStamps = page.locator('.stamp.empty');
      await expect(emptyStamps).toHaveCount(4);

      // Click generate (this hits the real `/api/v1/growth/referrals/generate` endpoint)
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

      // Verify filled stamps contain the '☕' character from animation
      for (let i = 0; i < 4; i++) {
         await expect(filledStamps.nth(i)).toHaveText('☕');
      }

      // Check share link generated correctly
      const shareLink = page.locator('#share-link');
      // Verify that the referral link uses the real format
      await expect(shareLink).toHaveValue(/loyalty\/join\?ref=[0-9a-fA-F-]+/);
    });
  });
});
