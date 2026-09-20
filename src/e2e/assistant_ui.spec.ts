import { test, expect } from './fixtures';

test('Assistant Workstation UI handles Mobile layout', async ({ page }) => {
  await page.setViewportSize({ width: 375, height: 667 });
  await page.goto('/assistant');
  await expect(page.getByTestId('assistant-shell')).toBeVisible();
  await expect(page.getByRole('navigation', { name: 'Assistant section menu' })).toBeVisible();
  const dimensions = await page.evaluate(() => ({
    documentWidth: document.documentElement.scrollWidth,
    viewportWidth: window.innerWidth,
  }));
  expect(dimensions.documentWidth - dimensions.viewportWidth).toBeLessThanOrEqual(1);
});
