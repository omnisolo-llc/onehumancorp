import { test, expect } from './fixtures';

test.describe('Help Center', () => {




  test('Persona: Business Owner interacts with a Tooltip', async ({ page }) => {
    await page.goto('/dashboard');
    const kairosLink = page.locator('a[href="/kairos"]');
    await expect(kairosLink).toBeVisible();
    await kairosLink.hover();
    await expect(page.locator('text=Click here to see what your AI helpers are working on and how they plan.').first()).toBeVisible();
  });

  test('Persona: Business Owner navigates to KAIROS page', async ({ page }) => {
     await page.goto('/kairos');
     // Ensure page loaded
     await expect(page.getByRole('heading', { name: 'Kairos' })).toBeVisible();
  });
});
