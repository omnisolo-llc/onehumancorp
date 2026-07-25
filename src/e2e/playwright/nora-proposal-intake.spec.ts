import { test, expect } from '@playwright/test';

test.describe('Nora Autonomous Proposal Intake Flow', () => {

  test('Client intake creates proposal automatically', async ({ request, page }) => {
    // navigate to the standard real page and use the real page
    await page.goto(`/proposals/customer-view`);
  });
});
