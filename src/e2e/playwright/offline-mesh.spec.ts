import { test, expect } from '@playwright/test';

test.describe('Distributed Offline-First AI Sync Mesh for Field Service Operations', () => {
  test('simulates offline mode, generates mutations, comes back online, syncs, and triggers AI agent', async ({ page, context }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button[type="submit"]');
    await page.waitForURL('/dashboard');

    await page.goto('/field-ops/jobs');

    await context.setOffline(true);

    const firstJobCard = page.locator('.p-5.bg-white').first();
    await expect(firstJobCard).toBeVisible();

    await firstJobCard.locator('textarea').fill('Replaced the valve');

    const jobDoneButton = firstJobCard.locator('button:has-text("Job Done")');
    if (await jobDoneButton.isVisible()) {
      await jobDoneButton.click();
    } else {
      const headingToJobButton = firstJobCard.locator('button:has-text("Heading to Job")');
      if (await headingToJobButton.isVisible()) {
        await headingToJobButton.click();
        const startWorkButton = firstJobCard.locator('button:has-text("Start Work")');
        await startWorkButton.click();
        await firstJobCard.locator('button:has-text("Job Done")').click();
      }
    }

    await expect(firstJobCard.locator('p:has-text("Replaced the valve")')).toBeVisible();

    await context.setOffline(false);

    await page.waitForTimeout(6000);

    await page.goto('/dashboard');

    await expect(page.locator('text=Invoice drafted for Field Job').first()).toBeVisible({ timeout: 15000 });
  });
});
