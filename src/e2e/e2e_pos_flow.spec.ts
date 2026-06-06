import { test, expect } from './fixtures';

test.describe('In-Person Payment (POS) Flow', () => {
  test('should authenticate from root and navigate to pos terminal', async ({ page }) => {
    // 1. Navigate to login and perform actual login
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await page.getByRole('button', { name: 'Log In' }).click();

    // 2. Ensure we are successfully on Dashboard
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

    // 3. Seed offline staff (required to unlock terminal) in context of the application
    await page.evaluate(() => {
      localStorage.setItem('ohc_offline_staff', JSON.stringify([
        {
          id: 'mock_staff_1',
          name: 'Carlos Handyman',
          role: 'Manager',
          pin_hash: '1111'
        }
      ]));
    });

    // 4. Navigate to the POS Terminal page
    await page.goto('/pos/terminal');

    // 5. Verify we are on the lock screen
    await expect(page.getByRole('heading', { name: 'Terminal Locked' })).toBeVisible();

    // 6. Click the "1" digit button 4 times to enter PIN "1111"
    const digit1 = page.getByRole('button', { name: '1' });
    await digit1.click();
    await digit1.click();
    await digit1.click();
    await digit1.click();

    // 7. Assert successful authentication by looking for the "Quick Actions" text
    await expect(page.locator('h3', { hasText: 'Quick Actions' })).toBeVisible();
    await expect(page.getByText('New Order')).toBeVisible();
  });
});
