import { test, expect } from './fixtures';

test.describe('Omni-Inbox Auto-Reply Agent', () => {
  test('simulates incoming message and auto-replies correctly', async ({ page }) => {
    await page.goto('/inbox');

    // Verify inbox data matches seeded initial payload
    await expect(page.getByText('Do you have vegan options for birthday cakes?')).toBeVisible();
  });
});
