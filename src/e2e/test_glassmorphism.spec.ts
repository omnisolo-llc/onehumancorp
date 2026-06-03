import { test, expect } from '@playwright/test';

test('Translucent Glass styling is applied', async ({ page }) => {
  await page.goto('/dashboard');

  const panel = page.locator('.ohc-hybrid-panel').first();
  await expect(panel).toBeVisible();

  const css = await panel.evaluate((el) => {
    const computed = window.getComputedStyle(el);
    return {
      backdropFilter: computed.backdropFilter,
    };
  });

  expect(css.backdropFilter).toContain('blur(30px)');
  expect(css.backdropFilter).toContain('saturate(210%)');
});
