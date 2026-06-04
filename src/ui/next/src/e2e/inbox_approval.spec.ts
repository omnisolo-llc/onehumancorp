import { test, expect } from '@playwright/test';

test.describe('Inbox Approval Flow', () => {
  test('should allow a business owner to approve a drafted reply', async ({ page }) => {
    // Navigate to the inbox page
    await page.goto('/inbox');

    // Due to lack of real backend data in E2E, mock the API response for the test
    await page.route('**/api/ui/inbox/messages*', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify([
          {
            id: 'msg_test_123',
            source: 'instagram',
            content: 'Do you have vegan options?',
            draft_reply: 'Yes we do! We have 3 left for this Saturday.',
            status: 'pending',
            created_at: '2023-10-27 10:00:00'
          }
        ])
      });
    });

    await page.route('**/api/ui/inbox/messages/msg_test_123/approve', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ success: true })
      });
    });

    await page.reload();

    // Wait for messages to load (checking for at least one message item)
    await page.waitForSelector('.app-list-item');

    // Select the first message
    const firstMessage = page.locator('.app-list-item').first();
    await firstMessage.click();

    // Ensure the Draft Reply section is visible
    await expect(page.locator('text=Draft Reply')).toBeVisible();

    // Verify the "Approve & Send" button is present
    const approveButton = page.locator('button', { hasText: 'Approve & Send' });
    await expect(approveButton).toBeVisible();

    // Setup an intercept to verify that the approve endpoint was called
    const approveRequestPromise = page.waitForRequest(request => request.url().includes('/api/ui/inbox/messages/msg_test_123/approve') && request.method() === 'POST');

    // Click to approve the message
    await approveButton.click();

    // Verify request was sent
    const approveRequest = await approveRequestPromise;
    expect(approveRequest.method()).toBe('POST');
  });
});
