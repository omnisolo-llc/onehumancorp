import { test, expect } from '../fixtures';

test.describe('Nora Proposal Intake', () => {
  test('Nora Proposal Intake test', async ({ page }) => {
    await page.goto(`/proposals`);
  });
});
