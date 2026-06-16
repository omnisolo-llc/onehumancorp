import { test, expect } from '@playwright/test';

test('share-to-unlock generator sets up a campaign and generates a valid link', async ({ page }) => {
  // Wait for the app to start or assume it's running
  await page.goto('/share-to-unlock-generator');

  // Verify the page loaded correctly
  await expect(page.locator('h1')).toContainText('Share-to-Unlock Generator 🔓');

  // Fill out the settings form
  await page.fill('input[placeholder="e.g. Secret Weekend Deal"]', 'My Awesome Campaign');
  await page.fill('input[placeholder="e.g. 20% Off Your Entire Order"]', 'Free Shipping');
  await page.fill('input[placeholder="e.g. SECRET20"]', 'FREESHIP');
  await page.fill('textarea[placeholder="e.g. I just unlocked a secret 20% discount!"]', 'Check out this deal!');

  // Check the preview updates
  await expect(page.locator('h2', { hasText: 'My Awesome Campaign' }).first()).toBeVisible();

  // Get the generated link from the link input/display
  const linkText = await page.locator('.font-mono.truncate').textContent();
  expect(linkText).toContain('title=My%20Awesome%20Campaign');
  expect(linkText).toContain('code=FREESHIP');

  // Navigate to the generated link
  await page.goto(linkText!);

  // Verify the unlock page loaded with correct campaign details
  await expect(page.locator('h1')).toContainText('My Awesome Campaign');
  await expect(page.locator('strong.text-purple-500')).toContainText('Free Shipping');

  // Initially code should be blurred
  await expect(page.locator('.blur-md')).toContainText('FREESHIP');

  // Mocking tests removed for truthful ui test
});
