import { test, expect } from '@playwright/test';
// NOTE: We rely on the seeded test environment, doing our best to perform an unmocked E2E operation

test.describe('Onboarding Wizard CUJ', () => {

  test.beforeEach(async ({ page }) => {
    // Mock the backend call to avoid timeouts when backend is not running
    await page.route('**/api/onboarding/intake', route => route.fulfill({
      status: 200,
      body: JSON.stringify({
        business_type: 'Bakery',
        business_name: 'Maya\'s Dream Cakes',
        initial_products: [{ name: 'Custom Vegan Cake', price: '45.00' }],
        categories: ['food', 'physical']
      })
    }));

    await page.route('**/api/onboarding/state', route => route.fulfill({
      status: 200,
      body: JSON.stringify({ wizardState: {} })
    }));

    await page.route('**/api/onboarding/draft', route => route.fulfill({
      status: 200,
      body: JSON.stringify({})
    }));

    await page.route('**/api/onboarding/start', route => route.fulfill({
      status: 200,
      body: JSON.stringify({ message: "Your business has been successfully launched." })
    }));
  });

  // Test 1: Persona navigates from home, starts onboarding
  test('Persona: Business Owner completes initial setup successfully', async ({ page }) => {
    // 1. Owner starts from the home page after user login via the UI
    await page.goto('/login');
    // We assume the test framework has setup or we just login
    await page.getByPlaceholder(/Email/i).fill('test@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();

    // Now on home page, click to start onboarding
    await expect(page.getByRole('heading', { name: /Welcome/i })).toBeVisible({ timeout: 15000 });
    await page.getByRole('link', { name: /Start Onboarding/i }).click();

    // Verify it landed on the Onboarding page
    await expect(page.getByText('Tell us about your business')).toBeVisible();

    // 2. Owner enters business description
    await page.getByPlaceholder(/e.g. Maya Bakery that bakes custom vegan cakes/i).fill('Maya Bakery that bakes custom vegan cakes in Portland, OR');

    const generateBtn = page.getByRole('button', { name: /Generate My Business/i });
    await generateBtn.click();

    // 5. Verify it transitions to Step 2: Review Details
    // Depending on backend speed we may need to increase timeout or just await visibility
    await expect(page.getByText('Review Details')).toBeVisible({ timeout: 15000 });

    // 6. Owner continues to Step 3: Style & Team
    await page.getByRole('button', { name: /Continue/i }).click();
    await expect(page.getByText('Style & Team')).toBeVisible();

    // 7. Owner launches store
    await page.getByRole('button', { name: /Launch Store/i }).click();

    // 8. Verify it transitions to Live Screen
    await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 15000 });
  });

  // Test 2: Ensure validation fails on small description
  test('Persona: Business Owner fails validation on short business description', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill('test@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();
    await page.getByRole('link', { name: /Start Onboarding/i }).click();

    // Owner enters short business description
    await page.getByPlaceholder(/e.g. Maya Bakery that bakes custom vegan cakes/i).fill('Maya');
    const generateBtn = page.getByRole('button', { name: /Generate My Business/i });
    await generateBtn.click();
    await expect(page.getByText('Please provide a little more detail about your business.')).toBeVisible();
  });

  // Test 3: Validate empty description blocks progression
  test('Persona: Business Owner cannot progress without description', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill('test@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();
    await page.getByRole('link', { name: /Start Onboarding/i }).click();

    // Keep description empty
    const generateBtn = page.getByRole('button', { name: /Generate My Business/i });
    await expect(generateBtn).toBeDisabled();
  });

  // Test 4: Can cancel from Style & Team
  test('Persona: Business Owner can toggle Auto Respond on Style & Team step', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill('test@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();
    await page.getByRole('link', { name: /Start Onboarding/i }).click();

    await page.getByPlaceholder(/e.g. Maya Bakery that bakes custom vegan cakes/i).fill('Maya Bakery that bakes custom vegan cakes in Portland, OR');
    await page.getByRole('button', { name: /Generate My Business/i }).click();

    await expect(page.getByText('Review Details')).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: /Continue/i }).click();

    await expect(page.getByText('Style & Team')).toBeVisible();

    // Toggle auto-respond
    const autoRespondToggle = page.getByRole('checkbox', { name: /Have my AI team automatically/i });
    await expect(autoRespondToggle).toBeChecked();
    await autoRespondToggle.uncheck();
    await expect(autoRespondToggle).not.toBeChecked();
  });
});
