import { test, expect } from '@playwright/test';

test.describe('Autonomous Service Portfolio Generator', () => {
  test('generates a portfolio case study approval card when a job completes with media', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/');

    // Send an API request to mock the backend event
    await page.evaluate(async () => {
      await fetch('/api/agents/workflows', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          action: 'trigger_event',
          event_type: 'tenant.job.completed',
          payload: {
            service_name: 'Cedar Fence Install',
            media: ['https://example.com/finished-fence.jpg']
          }
        })
      });
    });

    // Navigate to team/inbox page to see approvals
    await page.goto('/team');

    // Wait for the UI to update and verify the card is there
    await expect(page.locator('text=Drafted a new portfolio post for \'Cedar Fence Install\'')).toBeVisible({ timeout: 10000 });

    // Verify image is displayed
    const img = page.locator('img[alt="Project photo"]');
    await expect(img).toBeVisible();
    await expect(img).toHaveAttribute('src', 'https://example.com/finished-fence.jpg');

    // Verify buttons
    await expect(page.locator('button', { hasText: 'Publish to Website' })).toBeVisible();
    await expect(page.locator('button', { hasText: 'Edit' })).toBeVisible();
  });
});
