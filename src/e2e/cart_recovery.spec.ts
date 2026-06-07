import { test, expect } from './fixtures';

test.describe('Autonomous Abandoned Cart Recovery Agent', () => {
  test('should display abandoned cart recovery draft in unified feed and allow owner approval', async ({ page, context }) => {
    // 1. Navigate to the dashboard where the Unified Agent Feed is displayed
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    // 2. Look for the approval card representing the abandoned cart
    await expect(page.getByText('Abandoned Cart Detected')).toBeVisible({ timeout: 15000 });

    // We can also just look for the text of the description
    await expect(page.getByText('Sarah left a $45 Vegan Chocolate Cake in her cart.')).toBeVisible({ timeout: 15000 });

    // 4. Click the "Approve & Send" button
    // The UI renders buttons as "Approve" (with a checkmark icon)
    const card = page.locator('div').filter({ hasText: 'Abandoned Cart Detected' }).first().locator('..').locator('..').locator('..');
    const approveButton = card.getByRole('button', { name: 'Approve' }).first();

    await approveButton.click();

    // 5. Verify the card is removed or marked approved
    await expect(page.getByText('Abandoned Cart Detected')).toBeHidden({ timeout: 15000 });
  });
});
