import { test, expect } from './fixtures';

test.describe('Instagram Triage', () => {
  test('Dashboard loads', async ({ page }) => {
    await page.goto(`/`);
  });
});
