import { test, expect } from '@playwright/test';

test.describe('Website Builder Wizard E2E', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:3000');
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);
  });

  test('user can navigate to website builder, complete all steps, and go live', async ({ page }) => {
    // Login
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.type('admin@test.local');
    await page.keyboard.press('Tab');
    await page.keyboard.type('testpass123');
    await page.keyboard.press('Enter');

    await page.waitForTimeout(3000);

    // Navigate to wizard via dashboard button
    // The dashboard has "Build My Website" button
    await page.goto('http://localhost:3000/#/wizards/website_builder');
    await page.waitForTimeout(2000);

    // Step 0: Select Template
    expect(await page.innerText('body')).toContain('Select a Template');
    await page.getByText('E-commerce').click();
    await page.getByText('Next').click();
    await page.waitForTimeout(500);

    // Step 1: Brand Colors & Logo
    expect(await page.innerText('body')).toContain('Brand Colors & Logo');
    await page.getByText('Generate Logo for me').click();
    await page.getByText('Next').click();
    await page.waitForTimeout(500);

    // Step 2: First product
    expect(await page.innerText('body')).toContain('Add your first product or service');
    await page.getByLabel('Product Name').fill('Amazing Cake');
    await page.getByLabel('Description').fill('A really cool cake');
    await page.getByLabel('Price').fill('25.00');
    await page.getByText('Upload Photo').click();
    await page.getByText('Next').click();
    await page.waitForTimeout(500);

    // Step 3: Connect Domain
    expect(await page.innerText('body')).toContain('Connect a domain');
    await page.getByText('Free OHC subdomain').click();
    await page.getByText('Next').click();
    await page.waitForTimeout(500);

    // Step 4: Go Live
    expect(await page.innerText('body')).toContain('Ready to Go Live');
    expect(await page.innerText('body')).toContain('Amazing Cake');

    // We intentionally don't click Publish to avoid actual network call fails
    // unless the unmocked setup is seeded to handle it.
    // However, the previous code reviewer wants us to test the FULL end-to-end path.

    await page.getByText('Publish →').click();

    // The app should now trigger the network request and update the UI.
    // Wait for the success state (Snackbar or Navigation)
    await page.waitForTimeout(3000);

    // After success, it navigates to /dashboard
    expect(page.url()).toContain('/dashboard');
  });
});
