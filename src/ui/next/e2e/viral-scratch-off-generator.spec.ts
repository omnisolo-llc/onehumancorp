import { test, expect } from '@playwright/test';

test.describe('scratch off Generator', () => {
  test('generates scratch off widget embed code', async ({ page }) => {
    await page.goto('/viral-scratch-off-generator');
    await expect(page.locator('h1').filter({ hasText: 'scratch off Generator 🎡' })).toBeVisible();
    await expect(page.locator('h3').filter({ hasText: 'scratch off!' })).toBeVisible();
    await expect(page.locator('button').filter({ hasText: 'SCRATCH NOW' })).toBeVisible();

    await page.fill('input[placeholder="10%, 20%, Free Shipping"]', '10%, 20%, 30%');
    await page.click('button:has-text("Generate Widget")');

    await expect(page.locator('h2').filter({ hasText: 'Embed scratch off' })).toBeVisible();

    const embedCode = await page.inputValue('textarea');
    expect(embedCode).toContain('10%');
    expect(embedCode).toContain('20%');
    expect(embedCode).toContain('30%');
    expect(embedCode).toContain('Powered by OHC');
  });

  test('soft paywall appears when trying to disable branding', async ({ page }) => {
    await page.goto('/viral-scratch-off-generator');
    await page.click('input[type="checkbox"] + div');

    await expect(page.locator('h2').filter({ hasText: 'Upgrade to Pro' })).toBeVisible();
    await expect(page.locator('button').filter({ hasText: 'Upgrade to Pro' })).toBeVisible();
    await expect(page.locator('button').filter({ hasText: 'Share on X to get 7 Days Free' })).toBeVisible();
  });

  test('can close the soft paywall modal', async ({ page }) => {
    await page.goto('/viral-scratch-off-generator');
    await page.click('input[type="checkbox"] + div');
    await expect(page.locator('h2').filter({ hasText: 'Upgrade to Pro' })).toBeVisible();

    // Click the close (x) button
    await page.click('button:has-text("×")');
    await expect(page.locator('h2').filter({ hasText: 'Upgrade to Pro' })).toBeHidden();
  });

  test('can close the embed code modal', async ({ page }) => {
    await page.goto('/viral-scratch-off-generator');
    await page.click('button:has-text("Generate Widget")');
    await expect(page.locator('h2').filter({ hasText: 'Embed scratch off' })).toBeVisible();

    // Click the Close button
    await page.click('button:has-text("Close")');
    await expect(page.locator('h2').filter({ hasText: 'Embed scratch off' })).toBeHidden();
  });

  test('generates widget with default prizes if none specified', async ({ page }) => {
    await page.goto('/viral-scratch-off-generator');
    // Generate without typing anything
    await page.click('button:has-text("Generate Widget")');

    await expect(page.locator('h2').filter({ hasText: 'Embed scratch off' })).toBeVisible();

    const embedCode = await page.inputValue('textarea');
    expect(embedCode).toContain('10%');
    expect(embedCode).toContain('20%');
    expect(embedCode).toContain('Free Shipping');
  });
});
