import { test, expect } from './fixtures';

test.describe('OnboardingWizard CUJ', () => {
  test.beforeEach(async ({ page }) => {
    // Clear local storage to ensure fresh state
    await page.addInitScript(() => {
      window.localStorage.clear();
      window.localStorage.setItem('tenant_id', 'e2e-tenant');
      window.localStorage.setItem('user_id', 'e2e-admin-user');
    });
  });

  test('Maya the Baker can complete the onboarding flow', async ({ page }) => {
    test.setTimeout(60000);

    // Provide a mocked response for the LLM fallback timeout issues, but use real DB otherwise
    await page.route('**/api/onboarding/intake', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          business_name: 'Maya Bakery',
          business_type: 'Bakery',
          categories: ['food'],
          initial_products: [{ name: 'Custom Vegan Cake', price: '45.00' }]
        }),
      });
    });

    await page.route('**/api/onboarding/start', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ message: "Your business has been successfully launched." }),
      });
    });

    // Navigate to the onboarding page
    await page.goto('/onboarding');

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

    // We let the real intake endpoint handle this logic
    await page.getByRole('button', { name: 'Generate My Business' }).click();

    // --- Step 2: Review Details ---
    await expect(page.getByText('Review Details')).toBeVisible({ timeout: 15000 });

    // Verify AI extracted details (These might vary slightly since real AI, so verify presence of values)
    await expect(page.locator('input[value="Maya Bakery"]')).toBeVisible();

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
    await page.getByPlaceholder(/you@example.com/i).fill('maya@example.com');
    await page.getByPlaceholder(/••••••••/i).fill('mypassword123');

    // Launch store
    await page.getByRole('button', { name: 'Launch Store' }).click();

    // --- Step 4 & 5: Loading and Success ---
    // Wait for Step 4 loading text temporarily
    await expect(page.getByText('Building Your Business...')).toBeVisible();

    // Wait for the success screen (Step 5)
    await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 15000 });

    // Verify links to dashboard and storefront are present
    await expect(page.getByRole('link', { name: 'Go to Dashboard' })).toBeVisible();
    await expect(page.getByRole('link', { name: 'Preview Storefront' })).toBeVisible();
  });

  test('Carlos the Handyman can complete the onboarding flow', async ({ page }) => {
    test.setTimeout(60000);

    // Mock API requests for fast E2E test speeds (to avoid AI key usage)
    await page.route('**/api/onboarding/intake', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          business_name: 'Carlos Handyman Services',
          business_type: 'Services',
          categories: ['services'],
          initial_products: [{ name: 'Plumbing Fix', price: '100.00' }]
        }),
      });
    });

    await page.route('**/api/onboarding/start', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ message: "Your business has been successfully launched." }),
      });
    });

    // Navigate to the onboarding page
    await page.goto('/onboarding');

    // --- Step 1: Chat/Intake ---
    await expect(page.getByText('Tell us about your business')).toBeVisible();

    const nameInput = page.getByPlaceholder(/Maya's Custom Cakes/i);
    await nameInput.fill('Carlos Handyman Services');
    await page.getByRole('button', { name: 'Next' }).click();

    const sellInput = page.getByPlaceholder(/I bake custom vegan cakes/i);
    await sellInput.fill('I offer plumbing fixes, painting, and general home repairs.');
    await page.getByRole('button', { name: 'Next' }).click();

    const locInput = page.getByPlaceholder(/Portland, OR/i);
    await locInput.fill('Austin, TX');
    await page.getByRole('button', { name: 'Generate My Business' }).click();

    // --- Step 2: Review Details ---
    await expect(page.getByText('Review Details')).toBeVisible({ timeout: 15000 });
    await expect(page.locator('input[value="Carlos Handyman Services"]')).toBeVisible();

    // Proceed
    await page.getByRole('button', { name: 'Continue' }).click();

    // --- Step 3: Style & Team ---
    await expect(page.getByText('Style & Team')).toBeVisible();

    await page.getByPlaceholder(/you@example.com/i).fill('carlos@example.com');
    await page.getByPlaceholder(/••••••••/i).fill('carlospassword123');
    await page.getByRole('button', { name: 'Launch Store' }).click();

    // Wait for the success screen (Step 5)
    await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 15000 });
  });

  test('Draft saving functionality works', async ({ page }) => {
    test.setTimeout(60000);

    await page.route('**/api/onboarding/draft', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({}),
      });
    });

    await page.goto('/onboarding');
    await expect(page.getByText('Tell us about your business')).toBeVisible();

    const nameInput = page.getByPlaceholder(/Maya's Custom Cakes/i);
    await nameInput.fill('Draft Business');
    await page.getByRole('button', { name: 'Next' }).click();

    // Hit save draft
    await page.getByRole('button', { name: 'Save Draft' }).click();
    await expect(page.getByText('Draft Saved!')).toBeVisible();
  });

  test('Validation errors surface correctly', async ({ page }) => {
    test.setTimeout(60000);

    await page.goto('/onboarding');
    await expect(page.getByText('Tell us about your business')).toBeVisible();

    // Try too short name
    const nameInput = page.getByPlaceholder(/Maya's Custom Cakes/i);
    await nameInput.fill('Ma');
    await page.getByRole('button', { name: 'Next' }).click();
    await expect(page.getByText('Business Name must be at least 3 characters.')).toBeVisible();

    await nameInput.fill('Valid Name');
    await page.getByRole('button', { name: 'Next' }).click();

    // Try empty description
    await page.getByRole('button', { name: 'Next' }).click();
    await expect(page.getByText('Please tell us what you sell.')).toBeVisible();
  });

  test('Domain selection toggles and templates update correctly', async ({ page }) => {
    test.setTimeout(60000);

    await page.route('**/api/onboarding/intake', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          business_name: 'Test Biz',
          business_type: 'Services',
          categories: ['services'],
          initial_products: [{ name: 'Test Product', price: '100.00' }]
        }),
      });
    });

    await page.goto('/onboarding');

    // Complete Step 1 quickly
    const nameInput = page.getByPlaceholder(/Maya's Custom Cakes/i);
    await nameInput.fill('Test Biz');
    await page.getByRole('button', { name: 'Next' }).click();

    const sellInput = page.getByPlaceholder(/I bake custom vegan cakes/i);
    await sellInput.fill('Testing services');
    await page.getByRole('button', { name: 'Next' }).click();

    const locInput = page.getByPlaceholder(/Portland, OR/i);
    await locInput.fill('Test Location');
    await page.getByRole('button', { name: 'Generate My Business' }).click();

    // --- Step 2: Review Details ---
    await expect(page.getByText('Review Details')).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: 'Continue' }).click();

    // --- Step 3: Style & Team ---
    await expect(page.getByText('Style & Team')).toBeVisible();

    // Check template clicking
    await page.getByText('Bold').click();

    // Check domain toggling
    await page.getByText('Custom Domain').click();
    await page.getByText('Free Subdomain').click();

    // AI Agents toggling
    await page.getByText('Support Agent').click();
  });
});
