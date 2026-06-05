import { test, expect } from '@playwright/test';

test.describe('OnboardingWizard CUJ', () => {
  test.beforeEach(async ({ page }) => {
    // Clear local storage to ensure fresh state
    await page.addInitScript(() => {
      window.localStorage.clear();
    });
<<<<<<< HEAD

    // Universal mock for draft
=======
  });

  test('Maya the Baker can complete the onboarding flow', async ({ page }) => {
    // Mock the draft API call
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
    await page.route('/api/onboarding/draft', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({})
      });
    });
<<<<<<< HEAD
  });

  test('Maya the Baker can complete the onboarding flow', async ({ page }) => {
=======

    // Mock the intake API call (triggered when moving from Step 1 to Step 2)
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
    await page.route('/api/onboarding/intake', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          business_type: 'Bakery',
          business_name: 'Maya Bakery',
          categories: ['food', 'baking'],
          initial_products: [{ name: 'Custom Vegan Cake', price: '45.00' }]
        })
      });
    });

<<<<<<< HEAD
    await page.route('/api/onboarding/start', async route => {
      await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ message: "Your business has been successfully launched." }) });
    });

    await page.goto('/onboarding');
    await expect(page.getByText('Welcome')).toBeVisible();
    await page.getByText('Start Onboarding').click();

    await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Maya Bakery');
    await page.getByRole('button', { name: 'Next' }).click();

    await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('I bake custom vegan cakes for weddings and parties.');
    await page.getByRole('button', { name: 'Next' }).click();

    await page.getByPlaceholder(/Portland, OR/i).fill('Seattle, WA');
    await page.getByRole('button', { name: 'Generate My Business' }).click();

    await expect(page.locator('input[value="Maya Bakery"]')).toBeVisible();
    await page.getByRole('button', { name: 'Continue' }).click();

    await page.getByText('Modern').click();
=======
    // Mock the start API call (triggered when moving from Step 3 to Step 4/5)
    await page.route('/api/onboarding/start', async route => {
      // Simulate network delay for the loading animation
      await new Promise(resolve => setTimeout(resolve, 500));
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          message: "Your business has been successfully launched."
        })
      });
    });

    // Navigate to the onboarding page
    await page.goto('/onboarding');

    // Start the onboarding process
    await expect(page.getByText('Welcome')).toBeVisible();
    await page.getByText('Start Onboarding').click();

    // --- Step 1: Chat/Intake ---
    // Wait for the first prompt
    await expect(page.getByText('Tell us about your business')).toBeVisible();

    // Fill in the business name (Chat Step 1)
    const nameInput = page.getByPlaceholder(/Maya's Custom Cakes/i);
    await expect(nameInput).toBeVisible();
    await nameInput.fill('Maya Bakery');
    await page.getByRole('button', { name: 'Next' }).click();

    // Fill in the description (Chat Step 2)
    const sellInput = page.getByPlaceholder(/I bake custom vegan cakes/i);
    await expect(sellInput).toBeVisible();
    await sellInput.fill('I bake custom vegan cakes for weddings and parties.');
    await page.getByRole('button', { name: 'Next' }).click();

    // Fill in the location (Chat Step 3)
    const locInput = page.getByPlaceholder(/Portland, OR/i);
    await expect(locInput).toBeVisible();
    await locInput.fill('Seattle, WA');
    await page.getByRole('button', { name: 'Generate My Business' }).click();

    // --- Step 2: Review Details ---
    await expect(page.getByRole('heading', { name: 'Review Details' })).toBeVisible();

    // Verify AI extracted details correctly
    await expect(page.locator('input[value="Maya Bakery"]')).toBeVisible();
    await expect(page.locator('input[value="Bakery"]')).toBeVisible();
    await expect(page.locator('input[value="Custom Vegan Cake"]')).toBeVisible();
    await expect(page.locator('input[value="45.00"]')).toBeVisible();

    // Maya decides to proceed
    await page.getByRole('button', { name: 'Continue' }).click();

    // --- Step 3: Style & Team ---
    await expect(page.getByText('Style & Team')).toBeVisible();

    // Select website template
    await page.getByText('Modern').click();

    // Check auto respond toggle
    const toggle = page.getByRole('checkbox');
    await expect(toggle).toBeChecked();

    // Account Setup
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
    await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Maya Smith');
    await page.getByPlaceholder(/you@example.com/i).fill('maya@example.com');
    await page.getByPlaceholder(/••••••••/i).fill('mypassword123');

<<<<<<< HEAD
    await page.getByRole('button', { name: 'Launch Store' }).click();
    await expect(page.getByText("You're Live!")).toBeVisible();
  });

  test('Carlos the Handyman sets up his repair business', async ({ page }) => {
    await page.route('/api/onboarding/intake', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          business_type: 'Service',
          business_name: 'Carlos Fixes It',
          categories: ['repairs', 'home services'],
          initial_products: [{ name: 'Plumbing Fix', price: '150.00' }]
        })
      });
    });

    await page.route('/api/onboarding/start', async route => {
      await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ message: "Your business has been successfully launched." }) });
    });

    await page.goto('/onboarding');
    await page.getByText('Start Onboarding').click();

    await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Carlos Fixes It');
    await page.getByRole('button', { name: 'Next' }).click();

    await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('Plumbing and general repairs');
    await page.getByRole('button', { name: 'Next' }).click();

    await page.getByPlaceholder(/Portland, OR/i).fill('Austin, TX');
    await page.getByRole('button', { name: 'Generate My Business' }).click();

    await expect(page.locator('input[value="Carlos Fixes It"]')).toBeVisible();
    await page.getByRole('button', { name: 'Continue' }).click();

    await page.getByText('Minimal').click();
    await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Carlos');
    await page.getByPlaceholder(/you@example.com/i).fill('carlos@example.com');
    await page.getByPlaceholder(/••••••••/i).fill('password123');

    await page.getByRole('button', { name: 'Launch Store' }).click();
    await expect(page.getByText("You're Live!")).toBeVisible();
  });

  test('Leo the Music Tutor configures online bookings', async ({ page }) => {
    await page.route('/api/onboarding/intake', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          business_type: 'Booking',
          business_name: 'Leo Guitar Lessons',
          categories: ['music', 'education'],
          initial_products: [{ name: '1 Hour Guitar Lesson', price: '50.00' }]
        })
      });
    });

    await page.route('/api/onboarding/start', async route => {
      await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ message: "Your business has been successfully launched." }) });
    });

    await page.goto('/onboarding');
    await page.getByText('Start Onboarding').click();

    await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Leo Guitar Lessons');
    await page.getByRole('button', { name: 'Next' }).click();

    await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('Guitar tutoring online');
    await page.getByRole('button', { name: 'Next' }).click();

    await page.getByPlaceholder(/Portland, OR/i).fill('Remote');
    await page.getByRole('button', { name: 'Generate My Business' }).click();

    await expect(page.locator('input[value="Leo Guitar Lessons"]')).toBeVisible();
    await expect(page.locator('input[value="1 Hour Guitar Lesson"]')).toBeVisible();
    await page.getByRole('button', { name: 'Continue' }).click();

    await page.getByText('Classic').click();
    await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Leo Tutor');
    await page.getByPlaceholder(/you@example.com/i).fill('leo@music.com');
    await page.getByPlaceholder(/••••••••/i).fill('pass1234');

    await page.getByRole('button', { name: 'Launch Store' }).click();
    await expect(page.getByText("You're Live!")).toBeVisible();
  });

  test('Fatima the Food Cart Operator on a slower network', async ({ page }) => {
    await page.route('/api/onboarding/intake', async route => {
      await new Promise(resolve => setTimeout(resolve, 1000)); // Simulate slow network
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          business_type: 'Food Cart',
          business_name: 'Fatima Halal Food',
          categories: ['food', 'street vendor'],
          initial_products: [{ name: 'Chicken Over Rice', price: '9.00' }]
        })
      });
    });

    await page.route('/api/onboarding/start', async route => {
      await new Promise(resolve => setTimeout(resolve, 1000));
      await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ message: "Your business has been successfully launched." }) });
    });

    await page.goto('/onboarding');
    await page.getByText('Start Onboarding').click();

    await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Fatima Halal Food');
    await page.getByRole('button', { name: 'Next' }).click();

    await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('Halal food cart pickup orders');
    await page.getByRole('button', { name: 'Next' }).click();

    await page.getByPlaceholder(/Portland, OR/i).fill('New York, NY');
    await page.getByRole('button', { name: 'Generate My Business' }).click();

    await expect(page.locator('input[value="Fatima Halal Food"]')).toBeVisible();
    await page.getByRole('button', { name: 'Continue' }).click();

    await page.getByText('Bold').click();
    await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Fatima');
    await page.getByPlaceholder(/you@example.com/i).fill('fatima@foodcart.com');
    await page.getByPlaceholder(/••••••••/i).fill('halal123');

    await page.getByRole('button', { name: 'Launch Store' }).click();
    await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 5000 });
  });

  test('Validation errors prevent launching without complete admin info', async ({ page }) => {
    await page.route('/api/onboarding/intake', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          business_type: 'Test',
          business_name: 'Test Business',
          categories: ['test'],
          initial_products: [{ name: 'Test Product', price: '1.00' }]
        })
      });
    });

    await page.goto('/onboarding');
    await page.getByText('Start Onboarding').click();

    await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Test Business');
    await page.getByRole('button', { name: 'Next' }).click();

    await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('Testing');
    await page.getByRole('button', { name: 'Next' }).click();

    await page.getByPlaceholder(/Portland, OR/i).fill('Local');
    await page.getByRole('button', { name: 'Generate My Business' }).click();

    await page.getByRole('button', { name: 'Continue' }).click();

    // Do NOT fill out admin email and password
    await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Test Admin');

    // Attempt to launch store
    await page.getByRole('button', { name: 'Launch Store' }).click();

    // Expect validation errors to be visible - check exact wording from page.tsx ("Admin email is required") or just general red borders/messages
    await expect(page.getByText(/is required/i).first()).toBeVisible();

    // Ensure it hasn't progressed to the success screen
    await expect(page.getByText("You're Live!")).toBeHidden();
=======
    // Let the UI finish reacting to input before clicking launch
    await page.waitForTimeout(100);

    // Launch store
    await page.getByRole('button', { name: 'Launch Store' }).click();

    // --- Step 4 & 5: Loading and Success ---
    // Wait for Step 4 loading text temporarily
    await expect(page.getByText('Building Your Business...')).toBeVisible();

    // Wait for the success screen (Step 5)
    await expect(page.getByText("You're Live!")).toBeVisible();
    await expect(page.getByText('Your business has been successfully launched.')).toBeVisible();

    // Verify links to dashboard and storefront are present
    await expect(page.getByRole('link', { name: 'Go to Dashboard' })).toBeVisible();
    await expect(page.getByRole('link', { name: 'Preview Storefront' })).toBeVisible();
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
  });
});
