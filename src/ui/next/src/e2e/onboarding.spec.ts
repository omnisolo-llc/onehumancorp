import { test, expect } from '@playwright/test';

test.describe('OnboardingWizard CUJ', () => {
  test.beforeEach(async ({ page }) => {
    // Clear local storage to ensure fresh state
    await page.addInitScript(() => {
      window.localStorage.clear();
    });

    // Universal mock for draft
    await page.route('/api/onboarding/draft', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({})
      });
    });
  });

  test('Maya the Baker can complete the onboarding flow', async ({ page }) => {
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

    await page.route('/api/onboarding/start', async route => {
      await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ message: "Your business has been successfully launched.", organization_id: "org-mocked-123" }) });
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
    await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Maya Smith');
    await page.getByPlaceholder(/you@example.com/i).fill('maya@example.com');
    await page.getByPlaceholder(/••••••••/i).fill('mypassword123');

    await page.getByRole('button', { name: 'Launch Store' }).click();
    await expect(page.getByText("You're Live!")).toBeVisible();
    const storedTenantId = await page.evaluate(() => window.localStorage.getItem('tenant_id'));
    expect(storedTenantId).toBe('org-mocked-123');
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
      await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ message: "Your business has been successfully launched.", organization_id: "org-mocked-123" }) });
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
    const storedTenantId = await page.evaluate(() => window.localStorage.getItem('tenant_id'));
    expect(storedTenantId).toBe('org-mocked-123');
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
      await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ message: "Your business has been successfully launched.", organization_id: "org-mocked-123" }) });
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
    const storedTenantId = await page.evaluate(() => window.localStorage.getItem('tenant_id'));
    expect(storedTenantId).toBe('org-mocked-123');
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
      await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ message: "Your business has been successfully launched.", organization_id: "org-mocked-123" }) });
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
    const storedTenantId = await page.evaluate(() => window.localStorage.getItem('tenant_id'));
    expect(storedTenantId).toBe('org-mocked-123');
  });

  test('User can save a draft and restore it across sessions', async ({ page }) => {
    // Note: The global beforeEach mocks /api/onboarding/draft by default.
    // We must unroute it for this specific test so it hits the real backend.
    await page.unroute('/api/onboarding/draft');
    await page.unroute('/api/onboarding/state');

    // 1. Start Wizard and Save Draft
    await page.goto('/onboarding');
    await page.getByText('Start Onboarding').click();

    await page.getByPlaceholder(/Maya's Custom Cake/i).fill('My Restored Business');
    await page.getByRole('button', { name: 'Save Draft' }).click();
    await expect(page.getByText('Draft Saved!')).toBeVisible();

    // 2. Clear local storage to simulate device switch
    await page.evaluate(() => window.localStorage.clear());

    // 3. Reload page and check restoration
    await page.reload();

    // We should be restored to the first step of the wizard where we were, with the text filled
    await expect(page.getByText("What's the name of your business?")).toBeVisible();
    await expect(page.locator('input[value="My Restored Business"]')).toBeVisible();
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

    // Do NOT fill out admin email and password initially
    await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Test Admin');

    // Attempt to launch store
    await page.getByRole('button', { name: 'Launch Store' }).click();

    // Expect validation errors to be visible
    await expect(page.getByText(/is required/i).first()).toBeVisible();

    // Fill in invalid email and password without number
    await page.getByPlaceholder(/you@example.com/i).fill('invalid-email');
    await page.getByPlaceholder(/••••••••/i).fill('password');
    await page.getByRole('button', { name: 'Launch Store' }).click();

    await expect(page.getByText('Please enter a valid email address')).toBeVisible();
    await expect(page.getByText('Password must be at least 8 characters and contain a number')).toBeVisible();

    // Ensure it hasn't progressed to the success screen
    await expect(page.getByText("You're Live!")).toBeHidden();
  });
});
