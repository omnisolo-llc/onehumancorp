import { test, expect } from './fixtures';

test.describe('Dashboard Plain-Language Briefing Agent', () => {
  test('CUJ 1: displays the briefing correctly on desktop', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Your Daily Briefing' })).toBeVisible();
    await expect(page.getByText('Good morning!')).toBeVisible();
    await expect(page.getByText('Vegan Celebration Cake')).toBeVisible();
  });

  test('CUJ 2: displays the briefing correctly on mobile (375px)', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Your Daily Briefing' })).toBeVisible();
    const briefingText = page.locator('text=Vegan Celebration Cake');
    await expect(briefingText).toBeVisible();
  });

  test('CUJ 3: briefing acknowledgment works', async ({ page }) => {
    await page.goto('/dashboard');
    page.on('dialog', dialog => dialog.accept());
    await page.getByRole('button', { name: 'Got it' }).click();
  });

  test('CUJ 4: promotional email drafting trigger works', async ({ page }) => {
    await page.goto('/dashboard');
    page.on('dialog', dialog => dialog.accept());
    await page.getByRole('button', { name: 'Draft Weekend Promo' }).click();
  });

  test('CUJ 5: briefing reflects updated dynamic values', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByText('pending orders')).toBeVisible();
    await expect(page.getByText('active customers')).toBeVisible();
  });
});
