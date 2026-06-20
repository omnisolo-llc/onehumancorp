import { test, expect } from '@playwright/test';

test.describe('Lead Magnet Viral Loop', () => {
  test('generator works and produces embed code', async ({ page }) => {
    await page.goto('/lead-magnet-generator');

    await expect(page.locator('h1').filter({ hasText: 'Lead Magnet Generator' })).toBeVisible();

    // Check default input
    await expect(page.locator('input[value="Unlock the Ultimate Business Checklist"]')).toBeVisible();

    // Verify preview contains the title
    await expect(page.locator('h3').filter({ hasText: 'Unlock the Ultimate Business Checklist' })).toBeVisible();

    // Viral loop footer should exist in the live preview
    const loopLink = page.locator('a', { hasText: '⚡ Powered by OHC' }).first();
    await expect(loopLink).toBeVisible();

    const href = await loopLink.getAttribute('href');
    expect(href).toContain('api/v1/growth/referrals/click?target=/onboarding&ref=');

    // Test changing title
    const titleInput = page.locator('label:has-text("Headline") + input');
    await titleInput.fill('Get My New Book');
    await expect(page.locator('h3').filter({ hasText: 'Get My New Book' })).toBeVisible();
  });
});
