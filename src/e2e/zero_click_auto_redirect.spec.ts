import { test, expect } from './fixtures';

test.describe('Zero Click Builder Auto Redirect', () => {
  test('should auto redirect to dashboard feed skipping intermediate UI', async ({ page, request, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/zero-click-builder');
    await page.setViewportSize({ width: 375, height: 812 });

    const generateBtn = page.getByRole('button', { name: /Generate My Business/i });
    await page.fill('textarea[id="prompt"]', 'I am a local coffee roaster in Seattle needing a storefront.');
    await generateBtn.click();

    // The user should transition to the Unified Agent Feed directly
    await page.waitForURL('**/dashboard', { timeout: 20000 });
    await expect(page.locator('text=Your store is ready. Review and Publish.')).toBeVisible({ timeout: 15000 });
  });
});
