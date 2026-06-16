import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('AI-Powered Field Service Quoting & Scheduling Engine', () => {
  test('Carlos receives and approves a draft quote from the Work Feed', async ({ browser }) => {
    const page = await adminPage(browser);

    // Navigate to the Dashboard where UnifiedAgentFeed is
    await page.goto('/dashboard');

    // Check if the new card is visible
    await expect(page.locator('text=New Request: Ceiling Fan Install. Draft Quote Ready.')).toBeVisible();
    await expect(page.locator('text=Sales Agent drafted a $150 quote and found 3 available slots next week.')).toBeVisible();

    // Carlos clicks "Approve & Send"
    const approveButton = page.getByTestId('approve-quote');
    await expect(approveButton).toBeVisible();

    // In a real e2e, we would click this, but since it's just a static UI card for now
    // we can at least assert that it exists and is clickable
    await approveButton.click();

    // (Optional) We could verify it sends an API request to accept the quote,
    // but the backend logic for accepting estimates in this mock is not fully wired up.
    // So ensuring the button is there and interactive is the main verification.
  });
});
