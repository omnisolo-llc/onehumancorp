import { test, expect } from './fixtures';

test.describe('Help Center and Contextual Help (Tauri UI)', () => {

  test('Persona: Business Owner views the Changelog', async ({ page }) => {
    await page.goto('/api/ui/changelog.html');
    await expect(page.locator('text=Release Notes & Changelog').first()).toBeVisible();
    await expect(page.locator('text=v0.4.48 (Cloud)').first()).toBeVisible();
    await expect(page.locator('text=Cloud Scaling Improvements').first()).toBeVisible();
  });

  test('Persona: Developer views the API documentation', async ({ page }) => {
    await page.goto('/api/ui/api-docs.html');
    await expect(page.locator('text=Advanced:').first()).toBeVisible();
    await expect(page.locator('text=OHC Advanced API Reference').first()).toBeVisible();
  });

  test('Persona: Business Owner interacts with a Tooltip', async ({ page }) => {
    await page.goto('/api/ui/dashboard.html?test_walkthrough=true');
    const shareLink = page.locator('#generate-link-btn');
    await expect(shareLink).toBeVisible();
    await shareLink.hover();
    await expect(page.locator('text=Click here to share access with a team member.').first()).toBeVisible();
  });
});
