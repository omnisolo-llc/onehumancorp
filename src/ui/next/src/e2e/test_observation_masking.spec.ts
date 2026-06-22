
import { test, expect } from '@playwright/test';

test.describe('Observation Masking UI Settings', () => {
  test('Owner can navigate to system settings', async ({ page }) => {
    await page.goto('/assistant?panel=system');
    await expect(page.locator('h2', { hasText: 'System & Safety' })).toBeVisible();
  });

  test('Owner can open advanced settings to see observation masking', async ({ page }) => {
    await page.goto('/assistant?panel=system');
    await page.locator('button', { hasText: 'Show Advanced Settings' }).click();
    await expect(page.locator('.cardTitle', { hasText: 'Observation Masking' })).toBeVisible();
  });

  test('Owner can toggle observation masking off', async ({ page }) => {
    await page.goto('/assistant?panel=system');
    await page.locator('button', { hasText: 'Show Advanced Settings' }).click();
    const checkbox = page.locator('input[type="checkbox"]');
    await checkbox.uncheck();
    expect(await checkbox.isChecked()).toBe(false);
  });

  test('Owner can toggle observation masking on', async ({ page }) => {
    await page.goto('/assistant?panel=system');
    await page.locator('button', { hasText: 'Show Advanced Settings' }).click();
    const checkbox = page.locator('input[type="checkbox"]');
    await checkbox.check();
    expect(await checkbox.isChecked()).toBe(true);
  });

  test('Owner can toggle observation masking and save', async ({ page }) => {
    await page.goto('/assistant?panel=system');
    await expect(page.locator('h2', { hasText: 'System & Safety' })).toBeVisible();

    await page.locator('button', { hasText: 'Show Advanced Settings' }).click();
    await expect(page.locator('.cardTitle', { hasText: 'Observation Masking' })).toBeVisible();

    const checkbox = page.locator('input[type="checkbox"]');
    await checkbox.uncheck();

    const uiButton = page.locator('button', { hasText: 'Save UI Settings' });
    await expect(uiButton).toBeVisible();
    await uiButton.click();

    const toast = page.locator('div', { hasText: 'UI settings saved' }).last();
    await expect(toast).toBeVisible();
  });
});
