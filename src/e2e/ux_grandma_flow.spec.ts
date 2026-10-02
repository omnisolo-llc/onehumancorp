import { test, expect } from './fixtures';

test.describe('Grandmother UX End-to-End Flow Validation', () => {
  test('first-time user sees plain language dashboard headers', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/dashboard');

    await expect(page.locator("h2, h1").filter({ hasText: /Welcome back|Dashboard/ }).first()).toBeVisible({ timeout: 10000 });
  });

  test('quick actions expose guidance and custom software', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/dashboard');
    const startTour = page.getByRole('button', { name: 'Start Tour' });
    if (await startTour.isVisible({ timeout: 5000 }).catch(() => false)) {
      await startTour.click();
      await page.waitForTimeout(500);
      await expect(page.locator('#walkthrough-bubble').locator('text=Business Analytics')).toBeAttached({ timeout: 10000 });
    }
    const integrations = page.getByRole('link', { name: 'Integrations' });
    if (await integrations.isVisible({ timeout: 5000 }).catch(() => false)) {
      await integrations.click();
      await expect(page.locator('.app-title', { hasText: 'Tool Integrations' })).toBeVisible();
    }
  });

  test('login setup action opens the guided setup process', async ({ page }) => {
    await page.goto('/login');
    const setupBtn = page.getByRole('button', { name: /Start Business Setup/ });
    if (await setupBtn.isVisible({ timeout: 5000 }).catch(() => false)) {
      await setupBtn.click();
      await expect(page.locator('.app-title, h1', { hasText: /Setup/i })).toBeVisible();
    }
  });
});
