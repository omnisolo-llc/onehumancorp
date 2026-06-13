import { test, expect } from './fixtures';

test('Actor Model UI works end to end via UI', async ({ page, unlimitedAdminUser, loginAs }) => {
  // Login first to satisfy real E2E standard
  await loginAs(page, unlimitedAdminUser);

  await page.goto('/actor-model');

  await expect(page.locator('h1')).toContainText('Actor-Model Message Passing');

  await page.fill('textarea[id="message"]', 'Test Actor Model Task');

  await page.click('text=Send Message to Swarm');

  await expect(page.getByTestId('success-message')).toBeVisible({ timeout: 60000 });
});
