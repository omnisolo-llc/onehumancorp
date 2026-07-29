import { test, expect } from '@playwright/test';

test.describe('Omni Inbox Triage', () => {
  test('Omni Inbox Triage test', async ({ page }) => {
    await page.goto(`/triage`);
  });
});
