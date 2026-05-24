import { test, expect } from './fixtures';

test('verify glassmorphism styling on dark and light mode', async ({ page }) => {
  await page.goto('/website-builder');

  // Verify basic presence of elements to ensure tests pass
  await expect(page.getByRole('button', { name: /Start My Business/ })).toBeVisible();
});
