import { test, expect } from './fixtures';

test('Assistant Workstation UI handles Mobile layout', async ({ page }) => {
  await page.setViewportSize({ width: 375, height: 667 });
  await page.goto('/assistant');

  await expect(page.getByTestId('assistant-workstation')).toBeVisible({ timeout: 10000 });
  await expect(page.getByRole('navigation', { name: 'Assistant section menu' })).toBeVisible();
});
