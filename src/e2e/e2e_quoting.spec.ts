import { test, expect } from '@playwright/test';

test.describe('Quoting UI e2e', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('owner can navigate to quoting page, view a real quote from the backend, and approve it', async ({ page }) => {
    // Navigate to the quoting page
    await page.goto('/ui/quote.html?mode=owner&id=823e4567-e89b-12d3-a456-426614174000');

    await expect(page.locator('text=Review Estimate')).toBeVisible({ timeout: 15000 });

    // The component has a title "Review Estimate" and subtitle with Quote #
    await expect(page.locator('text=Quote #823e4567')).toBeVisible();

    const approveBtn = page.getByRole('button', { name: 'Approve & Send to Customer' });
    await expect(approveBtn).toBeVisible();

    await approveBtn.click();

    // wait for approving to finish
    await expect(approveBtn).not.toHaveText('Approving...');
  });
});
