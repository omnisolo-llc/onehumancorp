import { test, expect } from '@playwright/test';

test.describe('Business Setup Wizard E2E (Cross-Device Resume)', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:3000');
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);
  });

  test('user can start wizard, progress, refresh (resume), and complete', async ({ page }) => {
    // Login
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.type('admin@test.local');
    await page.keyboard.press('Tab');
    await page.keyboard.type('testpass123');
    await page.keyboard.press('Enter');

    await page.waitForTimeout(3000);

    // Navigate to wizard
    await page.goto('http://localhost:3000/#/business_setup');
    await page.waitForTimeout(2000);

    // Step 0: Welcome
    expect(await page.innerText('body')).toContain('Your business, live in minutes');
    await page.getByText('Get Started').click();
    await page.waitForTimeout(1000);

    // Step 1: Type
    expect(await page.innerText('body')).toContain('What kind of business are you building?');
    await page.getByText('Online Store').click();
    // updateBusinessType calls nextStep() implicitly
    await page.waitForTimeout(1000);

    // Step 2: Name
    expect(await page.innerText('body')).toContain('Tell us about your business');
    await page.getByLabel('Business Name').fill('Maya Cakes');
    await page.getByText('Continue').click();
    await page.waitForTimeout(1000);

    // Step 3: What do you sell
    expect(await page.innerText('body')).toContain('What do you sell?');
    await page.getByText('Physical products').click();
    await page.getByText('Continue').click();
    await page.waitForTimeout(1000);

    // Simulated "Refresh/Cross-Device Resume"
    // Reload the page. The backend draft state should load.
    await page.reload();
    await page.waitForTimeout(3000); // Give it time to load from backend

    // We should be back on step 4 now (or whatever step was saved last)
    // Actually the frontend doesn't resume the navigation state directly in the router,
    // but the `BusinessSetupState` does preserve the `step`!
    expect(await page.innerText('body')).toContain('How do you want to receive payments?');
    await page.getByText('Online only').click();
    await page.getByText('Continue').click();
    await page.waitForTimeout(1000);

    // Step 5: Admin
    expect(await page.innerText('body')).toContain('Administrator account');
    await page.getByLabel('Name').fill('Maya');
    await page.getByLabel('Email').fill('maya@cakes.com');
    await page.getByLabel('Password').fill('securepassword123');
    await page.getByText('Continue').click();
    await page.waitForTimeout(1000);

    // Step 6: Launch
    expect(await page.innerText('body')).toContain('Review & Launch');
    expect(await page.innerText('body')).toContain('Maya Cakes');
    expect(await page.innerText('body')).toContain('Online Store');

    await page.getByText('Launch My Business →').click();
    await page.waitForTimeout(3000);

    // After success, it navigates to /dashboard
    expect(page.url()).toContain('/dashboard');
  });
});
