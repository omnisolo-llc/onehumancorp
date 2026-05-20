import { test, expect } from './fixtures';

test.describe('Dashboard Mock Data Audit', () => {
  test('mock fallback data does not appear on dashboard', async ({ memberPage }) => {
    const page = memberPage;
    await page.goto('/dashboard');

    // Wait for the amount of time the mock interval used to take, plus a buffer
    await page.waitForTimeout(5000);

    // Assert that none of the 5 hardcoded mock actions appear on the page
    const mockActions = [
      "Reviewing customer inquiry",
      "Generating weekly report",
      "Optimizing website layout",
      "Responding to support ticket",
      "Updating product inventory"
    ];

    for (const mockAction of mockActions) {
      await expect(page.getByText(mockAction)).toHaveCount(0);
    }
  });
});
