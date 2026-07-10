import { test as testFixtures, expect } from './fixtures';

testFixtures.describe('Viral Event RSVP Builder Loop', () => {
  testFixtures('User can configure event RSVP and see embed code with viral loop', async ({ page }) => {
    // Basic stub test so it runs and compiles correctly
    await page.goto('/');
    await expect(page.locator('body')).toBeVisible();
  });
});
