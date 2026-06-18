import { test, expect } from '@playwright/test';

test.describe('Carlos Field Ops Notification', () => {
  test('Carlos receives and approves an agentic notification for a new inquiry', async ({ page }) => {
    // Navigate to dashboard


    // Ensure notification appears
    await expect(page.getByText('New plumbing inquiry from Carlos')).toBeVisible({ timeout: 10000 });
    await expect(page.getByText('Send quote for $150 and propose Tuesday')).toBeVisible();
    await expect(page.getByText(/"Hi Carlos.*Estimated price is \$150."/)).toBeVisible();

    // Click Approve & Send
    await page.getByRole('button', { name: 'Approve & Send' }).click();

    // Ensure it disappears
    await expect(page.getByText('New plumbing inquiry from Carlos')).toBeHidden();
  });
});
