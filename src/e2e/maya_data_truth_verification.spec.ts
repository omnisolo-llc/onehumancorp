import { test, expect } from '@playwright/test';

test.describe('Maya Data Truth Verification', () => {
  test('Verify Dashboard Data Integrity - 1440px', async ({ page }) => {
    // Intercept console logs
    page.on('console', msg => {
      if (msg.type() === 'error') console.log(`BROWSER ERROR: ${msg.text()}`);
    });

    await page.setViewportSize({ width: 1440, height: 900 });

    // Login to set user_name in localStorage (simulating real owner flow)
    await page.goto('http://localhost:3000/login');
    await page.fill('input[placeholder="Email or Username"]', 'maya');
    await page.fill('input[placeholder="Password"]', 'password');
    await page.click('button:has-text("Log In")');

    await expect(page).toHaveURL(/.*dashboard/);
    await page.waitForSelector('h2:has-text("Welcome back")');

    const greeting = await page.textContent('h2');
    console.log(`Observed greeting: ${greeting}`);

    // Check that "Human" is not there
    expect(greeting).not.toContain('Human');

    // Check that hidden widgets are actually gone
    const inviteWidget = await page.$('text=Invite & Earn');
    expect(inviteWidget).toBeNull();

    await page.screenshot({ path: 'screenshots/dashboard_data_truth_1440px.png' });
  });
});
