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

  test('Persona: Business Owner uses the Help Widget tabs', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/api/ui/dashboard.html');

    const helpBtn = page.locator('#ohc-floating-help-btn');
    await helpBtn.waitFor({ state: 'visible' });
    await helpBtn.click();

    // Check Ask anything
    const askAnythingTab = page.locator('button', { hasText: 'Ask anything' }).first();
    await expect(askAnythingTab).toBeVisible();
    await askAnythingTab.click();
    await expect(page.locator('input[placeholder="Ask anything..."]')).toBeVisible();

    // Check Videos
    const videosTab = page.locator('button', { hasText: 'Videos' }).first();
    await expect(videosTab).toBeVisible();
    await videosTab.click();
    await expect(page.locator('h3', { hasText: 'Tutorials' })).toBeVisible();

    // Check New
    const newTab = page.locator('button', { hasText: 'New' }).first();
    await expect(newTab).toBeVisible();
    await newTab.click();
    await expect(page.locator('h3', { hasText: "What's New" })).toBeVisible();
  });
});
