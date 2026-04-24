import { test, expect } from '@playwright/test';

test.describe('Business Setup Wizard E2E', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:8081');
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);
  });

  test('user can navigate through the 7 wizard steps from login to launch', async ({ page }) => {
    // Login
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.type('admin@test.local');
    await page.keyboard.press('Tab');
    await page.keyboard.type('testpass123');
    await page.keyboard.press('Enter');

    await page.waitForTimeout(3000);

    // Assuming we can go to the wizard directly. In some flows, users might click "Setup Business" button.
    // For test reliability without knowing exact dash layout, we'll navigate directly.
    await page.goto('http://localhost:8081/#/wizards/setup');
    await page.waitForTimeout(2000);

    // Step 0: Welcome
    expect(await page.innerText('body')).toContain('Your business, live in minutes.');
    await page.getByText('Get Started').click();
    await page.waitForTimeout(500);

    // Step 1: Business Type
    expect(await page.innerText('body')).toContain('What kind of business are you building?');
    await page.getByText('Online Store').click();
    // In our new implementation, tapping the tile auto-advances to next step.
    await page.waitForTimeout(500);

    // Step 2: Business Details
    expect(await page.innerText('body')).toContain('Name & Description');
    await page.getByLabel('Business Name').fill('OHC Store');
    await page.getByText('Next').click();
    await page.waitForTimeout(500);

    // Step 3: What do you sell?
    expect(await page.innerText('body')).toContain('What do you sell?');
    await page.getByText('Physical products').click();
    await page.getByText('Next').click();
    await page.waitForTimeout(500);

    // Step 4: Payment Preference
    expect(await page.innerText('body')).toContain('How do you want to receive payments?');
    await page.getByText('Online only').click();
    // Auto-advances.
    await page.waitForTimeout(500);

    // Step 5: Administrator Account
    expect(await page.innerText('body')).toContain('Create your Administrator account');
    await page.getByLabel('Full Name').fill('Admin User');
    await page.getByLabel('Email Address').fill('admin@ohc.local');
    await page.getByLabel('Password').fill('SecurePassword123');
    await page.getByText('Next').click();
    await page.waitForTimeout(500);

    // Step 6: Review & Launch
    expect(await page.innerText('body')).toContain('Review & Launch');
    expect(await page.innerText('body')).toContain('OHC Store');

    await page.getByText('Launch My Business →').click();

    // Verify loading state
    await page.waitForTimeout(100);
    expect(await page.innerText('body')).toContain('Your business is setting up…');

    // After success, it navigates to /dashboard
    await page.waitForTimeout(3000);
    expect(page.url()).toContain('/dashboard');
  });
});
