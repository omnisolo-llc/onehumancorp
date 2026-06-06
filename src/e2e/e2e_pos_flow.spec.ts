import { test, expect } from './fixtures';

test.describe('In-Person Payment (POS) Flow', () => {
  test('should unlock terminal with PIN and view quick actions', async ({ page }) => {
    // 1. Visit a blank page or root to set localStorage before navigating to the app
    await page.goto('/');

    // 2. Seed localStorage with the required offline staff user
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

    // 3. Navigate to the POS Terminal page
    await page.goto('/pos/terminal');

    // 4. Verify we are on the lock screen
    await expect(page.getByRole('heading', { name: 'Terminal Locked' })).toBeVisible();

    // 5. Click the "1" digit button 4 times to enter PIN "1111"
    const digit1 = page.getByRole('button', { name: '1' });
    await digit1.click();
    await digit1.click();
    await digit1.click();
    await digit1.click();

    // 6. Assert successful authentication by looking for the "Quick Actions" text
    await expect(page.locator('h3', { hasText: 'Quick Actions' })).toBeVisible();
    await expect(page.getByText('New Order')).toBeVisible();
  });
});
