import { Page } from '@playwright/test';

export async function login(page: Page) {
  await page.goto((process.env.PLAYWRIGHT_BASE_URL || 'http://localhost:8080') + '/');

  // Wait for the app to load
  await page.waitForLoadState('networkidle');
  await page.waitForTimeout(2000);

  // Click on "Or continue to Cloud Dashboard" by text, ignoring casing issues
  await page.locator('text=Or continue to Cloud Dashboard').click({ timeout: 5000 }).catch(async () => {
    // Fallback: If click fails, evaluate navigation directly in the window object (if go router allows it) or just use route.
    console.log("Could not click 'Or continue to Cloud Dashboard', falling back to direct navigation to /login");
    await page.goto((process.env.PLAYWRIGHT_BASE_URL || 'http://localhost:8080') + '/#/login');
  });

  await page.waitForTimeout(1000);

  // To avoid semantic locator flakiness, just click the screen roughly where the first field is or use keyboard tabbing.
  // We can tab into the fields reliably if the app starts focus properly.
  // Actually, we can just find inputs by type if possible, but Flutter Canvas doesn't render DOM inputs by default unless accessibility is on.

  // Tab through to the first field and type
  await page.keyboard.press('Tab');
  await page.keyboard.press('Tab');
  await page.keyboard.type('admin');

  await page.keyboard.press('Tab');
  await page.keyboard.type('adminpass123');

  await page.keyboard.press('Enter');

  // Wait for login to complete (either network request or dashboard nav)
  await page.waitForTimeout(3000);
}
