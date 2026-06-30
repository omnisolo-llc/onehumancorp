import { test, expect } from '@playwright/test';

test.describe('Agentic Onboarding', () => {
  test('Completes the conversational onboarding flow successfully', async ({ page }) => {
    await page.goto('/onboarding');

    // Check initial chat message
    await expect(page.getByText("Hi there! I'm your OHC Work Assistant. What kind of work do you do?")).toBeVisible();

    // Fill out the input field
    const input = page.getByPlaceholder('Type your response...');
    await input.fill('I bake cakes for local pickup');

    // Send the message by pressing enter (more reliable than locator)
    await input.press('Enter');

    // Wait for approval card
    await expect(page.getByText('Proposed Configuration')).toBeVisible({ timeout: 10000 });

    // Approve
    await page.getByText('Approve & Go Live').click();

    // Wait for Live screen
    await expect(page.getByText("Your business has been successfully launched.")).toBeVisible({ timeout: 10000 });
  });
});
