import { test, expect } from '@playwright/test';

test.describe('Autonomous Service Quoting Agent', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate directly to the Inbox page
    await page.goto('http://localhost:3000/inbox');
  });

  test('owner can view a customer request, see the drafted quote, and approve it hands-free', async ({ page }) => {
    // 1) Verify the page has loaded
    await expect(page.getByRole('heading', { name: 'Customer Inbox' })).toBeVisible();

    // 2) Look for the specific message from Sarah (SMS) asking for a quote
    await expect(page.getByText('Hi Carlos, my pipe is leaking under the kitchen sink. Can you fix it?')).toBeVisible();

    // 3) Verify the AI Quote Draft component is visible with details
    await expect(page.getByText('AI Quote Draft')).toBeVisible();
    await expect(page.getByText('Kitchen Sink Pipe Repair')).toBeVisible();
    await expect(page.getByText('$150')).toBeVisible();

    // 4) Click the "Approve & Send" button
    await page.getByRole('button', { name: 'Approve & Send' }).click();

    // 5) Verify that the drafted quote has been approved and sent to the chat feed
    // It should now appear as a message from "Me"
    const sentMessageText = 'Quote Attached: Kitchen Sink Pipe Repair - $150. Please click here to pay deposit and book.';
    await expect(page.getByText(sentMessageText)).toBeVisible();

    // The draft component should no longer be visible for that message
    await expect(page.getByText('AI Quote Draft')).not.toBeVisible();
  });
});
