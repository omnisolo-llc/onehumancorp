import { test, expect } from '@playwright/test';

test('Staff Manager handles data format correctly without crashing', async ({ page }) => {
  // Navigate to the team page which contains the StaffManager component
  await page.goto('/team');

  // Wait for the staff manager header to be visible
  await expect(page.getByText('Your Human Staff')).toBeVisible();

  // Verify the staff member loaded and rendered successfully
  await expect(page.getByText('Test Staff')).toBeVisible();
  await expect(page.getByText('Cashier • 123-456-7890')).toBeVisible();
});
