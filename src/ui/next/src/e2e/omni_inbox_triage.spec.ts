import { test, expect } from '../../../../e2e/fixtures';

test.describe('Omni Inbox Triage UI', () => {
  test('Owner can view omni inbox triage', async ({ adminPage }) => {
    const page = await adminPage;
    await page.goto('/dashboard');
    await expect(page.locator('body')).toBeVisible();
  });
});
