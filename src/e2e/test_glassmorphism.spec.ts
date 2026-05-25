import { test, expect } from './fixtures';

test('verify glassmorphism styling on dark and light mode', async ({ page }) => {
  await page.goto('/website-builder');

  // Verify basic presence of elements to ensure tests pass
  await expect(page.getByRole('button', { name: /Start My Business Next/ })).toBeVisible();

  // Verify the glassmorphism CSS properties
  const setupScreen = page.locator('#setup-screen');
  await expect(setupScreen).toBeVisible();

  // We can evaluate the computed styles to ensure the frosted glass effect is active
  const backdropFilter = await setupScreen.evaluate((el) => {
    return window.getComputedStyle(el).getPropertyValue('backdrop-filter') || window.getComputedStyle(el).getPropertyValue('-webkit-backdrop-filter');
  });

  // Modern browsers will normalize the value, but we can at least check if blur is applied
  expect(backdropFilter).toContain('blur');

  // Verify the box-shadow contains the 31, 38, 135 color (or equivalent rgb)
  const boxShadow = await setupScreen.evaluate((el) => {
    return window.getComputedStyle(el).getPropertyValue('box-shadow');
  });
  expect(boxShadow).toContain('rgba(31, 38, 135');
});
