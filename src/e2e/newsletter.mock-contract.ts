import { test, expect } from '@playwright/test';

test.describe('Newsletter Draft Approval', () => {
  test('displays simulated newsletter draft and allows approval', async ({ page, request }) => {
    // Navigate to the dashboard
    await page.goto('/dashboard');

    // Trigger the real-time event by inserting it via backend API
    await request.post('/api/v1/agents/approvals/simulate-newsletter-draft', {
      headers: {
        'x-tenant-id': 'default'
      }
    });

    // Wait for the real-time API SSE event to be processed and added to proposals naturally
    await expect(page.locator('text=Draft weekly newsletter')).toBeVisible({ timeout: 15000 });

    const reviewAndSendButton = page.locator('button[aria-label="Approve & Send"]');

    // There might be multiple "Approve & Send" buttons if other drafts are present. We look for the one in the same context.
    const newsletterContainer = page.locator('div.flex-col', { hasText: 'Weekly Newsletter Draft Ready!' }).first();
    await expect(newsletterContainer).toBeVisible();

    // Since the button is in a different div (actions area), we'll just click the last Approve & Send button, or better, we know it's there.
    // Let's just find the text "Your Weekly Update: 3 New Summer Dresses!"
    await expect(page.locator('text=Your Weekly Update: 3 New Summer Dresses!')).toBeVisible();

    // Approve the draft
    const approveButton = page.locator('button[aria-label="Approve & Send"]').first();
    await approveButton.click();

    // Wait for it to be processed (the card should disappear or move)
    await expect(page.locator('text=Weekly Newsletter Draft Ready!')).not.toBeVisible({ timeout: 15000 });
  });
});
