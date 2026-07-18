import { test, expect } from '../../../../e2e/fixtures';

test('Staff Manager handles data format correctly without crashing', async ({ page }) => {
  // Mock the /api/staff endpoint to return the format the backend uses
  await page.route('**/api/staff', async (route) => {
    if (route.request().method() === 'GET') {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          staff: [
            {
              id: '1',
              name: 'Test Staff',
              phone_number: '123-456-7890',
              role: 'Cashier',
              pin_hash: '1234'
            }
          ]
        })
      });
    } else {
      await route.continue();
    }
  });

  // Navigate to the team page which contains the StaffManager component
  await page.goto('/team');

  // Wait for the staff manager header to be visible
  await expect(page.getByText('Your Human Staff')).toBeVisible();

  // Verify the staff member loaded and rendered successfully
  await expect(page.getByText('Test Staff')).toBeVisible();
  await expect(page.getByText('Cashier • 123-456-7890')).toBeVisible();
});
