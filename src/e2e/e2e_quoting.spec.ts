import { test, expect } from '@playwright/test';

test.describe('Quoting UI e2e', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('owner can navigate to quoting page, view a real quote from the backend, and approve it', async ({ page }) => {
    // Navigate to the quoting page
    await page.goto('/quoting?id=823e4567-e89b-12d3-a456-426614174000');

    await expect(page.locator('text=Quote Details')).toBeVisible({ timeout: 15000 });

    const approveBtn = page.getByRole('button', { name: 'Pay Deposit with Pay' });
    await expect(approveBtn).toBeVisible();

    await approveBtn.click();
  });
});
