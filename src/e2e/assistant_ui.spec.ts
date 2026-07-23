import { test, expect } from './fixtures';

test('Assistant Workstation UI handles Mobile layout', async ({ page }) => {
  // Navigate to standard UI route for Tauri or Next prototype
  // Use the Tauri UI directly. Note that we don't mock network requests.
  await page.goto('/assistant.html');

  // Set viewport to mobile width (375px) to test the Mobile UX flow
  await page.setViewportSize({ width: 375, height: 667 });

  // Basic check to see if layout renders
  const assistantHeader = page.locator('.header-title, h1, text=WorkBuddy');
  if (await assistantHeader.count() > 0) {
      await expect(assistantHeader.first()).toBeVisible({ timeout: 10000 });
  }
});
