import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Viral Powered by OHC Widget Flow', () => {
  test('Owner can navigate to the widget page and generate embed code', async ({ page }) => {
    await adminPage(page, async () => {
      // Navigate to the dashboard
      await page.goto('/ui/dashboard.html');

      // Click the "Powered by OHC Widget" link
      await page.click('#powered-by-ohc-link');

      // Verify we are on the widget generator page
      await expect(page).toHaveURL(/.*viral-powered-by-ohc-widget.html/);
      await expect(page.locator('h1')).toHaveText(/Embed Footer Badge|Grow your business with OHC/);

      // Click the generate button
      await page.click('#generate-widget-btn, #generate-btn');

      // Verify the output text area is visible and contains the expected code
      const textarea = page.locator('#embed-code-textarea, #embed-code');
      await expect(textarea).toBeVisible();
      await expect(textarea).toContainText(/ohc-widget|OHC Referral Footer Badge/);
    });
  });
});
