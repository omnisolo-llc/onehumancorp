import { test, expect } from './fixtures';

test('Assistant Workstation UI handles Mobile layout', async ({ page }) => {
  await page.setViewportSize({ width: 375, height: 667 });
  await page.goto('/assistant');
  const workstation = page.getByTestId('assistant-workstation');
  await expect(workstation).toBeVisible();
  await expect(page.getByRole('navigation', { name: 'Assistant section menu' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'New Task', exact: true })).toBeVisible();
  await expect.poll(() => page.evaluate(() =>
    document.documentElement.scrollWidth <= document.documentElement.clientWidth,
  )).toBe(true);
});
