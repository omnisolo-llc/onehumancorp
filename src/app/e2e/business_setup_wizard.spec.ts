import { test, expect } from '@playwright/test';

test.describe('Business Setup Wizard E2E - All Personas', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the app root and wait for network idle to ensure the app is fully loaded.
    await page.goto('http://localhost:3000');
    await page.waitForLoadState('networkidle');

    // Every E2E test MUST start from the home page after user login via the UI (no pre-authenticated state shortcuts).
    await page.fill('input[name="username"]', 'admin');
    await page.fill('input[name="password"]', 'admin');
    await page.click('button:has-text("Login")');

    // Wait for navigation to complete after login
    await expect(page.locator('text=Dashboard')).toBeVisible();
  });

  test('Maya (The Home Baker) - Online Store, Physical Products, Online Payments', async ({ page }) => {
    // Navigate to the business setup wizard via the UI
    await page.goto('http://localhost:3000/#/business_setup');
    await expect(page.getByText('Your business, live in minutes')).toBeVisible();

    // Step 0: Welcome Screen
    await page.getByText('Get Started').click();

    // Step 1: Business Type
    await expect(page.getByText('What kind of business are you building?')).toBeVisible();
    await page.getByText('Products').click();

    // Step 2: Tell us about your business
    await expect(page.getByText('Tell us about your business')).toBeVisible();
    await page.getByLabel('Business Name').fill('Maya\'s Cakes');
    await page.getByLabel('Short Description').fill('Custom Orders');
    await page.getByText('Continue').click();

    // Step 3: What do you sell?
    await expect(page.getByText('What do you sell?')).toBeVisible();
    await page.getByText('Physical products').click();
    await page.getByText('Continue').click();

    // Step 4: Payments
    await expect(page.getByText('How do you want to receive payments?')).toBeVisible();
    await page.getByText('Online only').click();
    await page.getByText('Continue').click();

    // Step 5: Administrator account
    await expect(page.getByText('Administrator account')).toBeVisible();
    await page.getByLabel('Name').fill('Maya');
    await page.getByLabel('Email').fill('maya@cakes.com');
    await page.getByLabel('Password').fill('pass1234');
    await page.getByText('Continue').click();

    // Step 6: Review & Launch
    await expect(page.getByText('Review & Launch')).toBeVisible();
    await expect(page.getByText('Maya\'s Cakes')).toBeVisible();
    await expect(page.getByText('Products')).toBeVisible();
    await expect(page.getByText('Physical products')).toBeVisible();
    await expect(page.getByText('Online only')).toBeVisible();

    await page.getByText('Launch My Business →').click();

    // Wait for the URL to change to dashboard without fixed timeouts
    await expect(page).toHaveURL(/.*\/dashboard/);
  });

  test('Carlos (The Freelance Handyman) - Service Business, Appointments, Skip Payments', async ({ page }) => {
    await page.goto('http://localhost:3000/#/business_setup');
    await expect(page.getByText('Your business, live in minutes')).toBeVisible();

    await page.getByText('Get Started').click();

    await expect(page.getByText('What kind of business are you building?')).toBeVisible();
    await page.getByText('Services').click();

    await expect(page.getByText('Tell us about your business')).toBeVisible();
    await page.getByLabel('Business Name').fill('Handyman Services');
    await page.getByLabel('Short Description').fill('Plumbing Fix');
    await page.getByText('Continue').click();

    await expect(page.getByText('What do you sell?')).toBeVisible();
    await page.getByText('Services / appointments').click();
    await page.getByText('Continue').click();

    await expect(page.getByText('How do you want to receive payments?')).toBeVisible();
    await page.getByText('Skip for now').click();
    await page.getByText('Continue').click();

    await expect(page.getByText('Administrator account')).toBeVisible();
    await page.getByLabel('Name').fill('Carlos');
    await page.getByLabel('Email').fill('carlos@handy.com');
    await page.getByLabel('Password').fill('pass1234');
    await page.getByText('Continue').click();

    await expect(page.getByText('Handyman Services')).toBeVisible();
    await expect(page.getByText('Services', { exact: true })).toBeVisible();
    await expect(page.getByText('Services / appointments')).toBeVisible();
    await expect(page.getByText('Skip for now')).toBeVisible();

    await page.getByText('Launch My Business →').click();
    await expect(page).toHaveURL(/.*\/dashboard/);
  });

  test('Priya (The Boutique Owner) - Local Business, Physical Products, POS Payments', async ({ page }) => {
    await page.goto('http://localhost:3000/#/business_setup');
    await expect(page.getByText('Your business, live in minutes')).toBeVisible();

    await page.getByText('Get Started').click();

    await expect(page.getByText('What kind of business are you building?')).toBeVisible();
    await page.getByText('Products').click();

    await expect(page.getByText('Tell us about your business')).toBeVisible();
    await page.getByLabel('Business Name').fill('Priya Boutique');
    await page.getByLabel('Short Description').fill('Clothing');
    await page.getByText('Continue').click();

    await expect(page.getByText('What do you sell?')).toBeVisible();
    await page.getByText('Physical products').click();
    await page.getByText('Continue').click();

    await expect(page.getByText('How do you want to receive payments?')).toBeVisible();
    await page.getByText('In-person (POS)').click();
    await page.getByText('Continue').click();

    await expect(page.getByText('Administrator account')).toBeVisible();
    await page.getByLabel('Name').fill('Priya');
    await page.getByLabel('Email').fill('priya@boutique.com');
    await page.getByLabel('Password').fill('pass1234');
    await page.getByText('Continue').click();

    await expect(page.getByText('Priya Boutique')).toBeVisible();
    await expect(page.getByText('Products')).toBeVisible();
    await expect(page.getByText('In-person (POS)')).toBeVisible();

    await page.getByText('Launch My Business →').click();
    await expect(page).toHaveURL(/.*\/dashboard/);
  });

  test('Leo (The Music Tutor) - Creative / Portfolio, Subscriptions, Online Payments', async ({ page }) => {
    await page.goto('http://localhost:3000/#/business_setup');
    await expect(page.getByText('Your business, live in minutes')).toBeVisible();

    await page.getByText('Get Started').click();

    await expect(page.getByText('What kind of business are you building?')).toBeVisible();
    await page.getByText('Portfolios').click();

    await expect(page.getByText('Tell us about your business')).toBeVisible();
    await page.getByLabel('Business Name').fill('Leo Music');
    await page.getByLabel('Short Description').fill('Tutor');
    await page.getByText('Continue').click();

    await expect(page.getByText('What do you sell?')).toBeVisible();
    await page.getByText('Subscriptions').click();
    await page.getByText('Continue').click();

    await expect(page.getByText('How do you want to receive payments?')).toBeVisible();
    await page.getByText('Online only').click();
    await page.getByText('Continue').click();

    await expect(page.getByText('Administrator account')).toBeVisible();
    await page.getByLabel('Name').fill('Leo');
    await page.getByLabel('Email').fill('leo@music.com');
    await page.getByLabel('Password').fill('pass1234');
    await page.getByText('Continue').click();

    await expect(page.getByText('Leo Music')).toBeVisible();
    await expect(page.getByText('Portfolios')).toBeVisible();
    await expect(page.getByText('Subscriptions')).toBeVisible();

    await page.getByText('Launch My Business →').click();
    await expect(page).toHaveURL(/.*\/dashboard/);
  });

  test('Fatima (The Food Cart Operator) - Restaurant / Food, Food & beverages, POS Payments', async ({ page }) => {
    await page.goto('http://localhost:3000/#/business_setup');
    await expect(page.getByText('Your business, live in minutes')).toBeVisible();

    await page.getByText('Get Started').click();

    await expect(page.getByText('What kind of business are you building?')).toBeVisible();
    await page.getByText('Food').click();

    await expect(page.getByText('Tell us about your business')).toBeVisible();
    await page.getByLabel('Business Name').fill('Fatima Cart');
    await page.getByLabel('Short Description').fill('Halal Plate');
    await page.getByText('Continue').click();

    await expect(page.getByText('What do you sell?')).toBeVisible();
    await page.getByText('Food & beverages').click();
    await page.getByText('Continue').click();

    await expect(page.getByText('How do you want to receive payments?')).toBeVisible();
    await page.getByText('In-person (POS)').click();
    await page.getByText('Continue').click();

    await expect(page.getByText('Administrator account')).toBeVisible();
    await page.getByLabel('Name').fill('Fatima');
    await page.getByLabel('Email').fill('fatima@cart.com');
    await page.getByLabel('Password').fill('pass1234');
    await page.getByText('Continue').click();

    await expect(page.getByText('Fatima Cart')).toBeVisible();
    await expect(page.getByText('Food', { exact: true })).toBeVisible();
    await expect(page.getByText('Food & beverages')).toBeVisible();

    await page.getByText('Launch My Business →').click();
    await expect(page).toHaveURL(/.*\/dashboard/);
  });
});
