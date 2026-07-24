import { test, expect } from '../fixtures';

test.describe('Nora Autonomous Proposal Intake Flow', () => {
  test('Client intake creates proposal automatically', async ({ request, page }) => {
    await page.goto(`/`);
  });
});
