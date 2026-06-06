import { test, expect } from '@playwright/test';

test('take a screenshot of pos terminal', async ({ page }) => {
  await page.goto('http://127.0.0.1:3000/pos/terminal');
  // Wait for the app to be fully rendered
  await page.waitForLoadState('networkidle');
  // Wait a bit to ensure Client component has mounted
  await page.waitForTimeout(2000);

  // Create a fake offline staff to get past the lock screen
  await page.evaluate(() => {
    localStorage.setItem('ohc_offline_staff', JSON.stringify([{
      id: 'staff1',
      name: 'Test Staff',
      role: 'Manager',
      pin_hash: '1234'
    }]));
  });

  // Reload to pick up local storage
  await page.goto('http://127.0.0.1:3000/pos/terminal');
  await page.waitForTimeout(1000);

  // Punch in the PIN 1234
  await page.getByRole('button', { name: '1' }).click();
  await page.getByRole('button', { name: '2' }).click();
  await page.getByRole('button', { name: '3' }).click();
  await page.getByRole('button', { name: '4' }).click();

  await page.waitForTimeout(2000);

  // Take screenshot
  await page.screenshot({ path: 'screenshot.png', fullPage: true });
});
