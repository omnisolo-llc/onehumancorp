import { test, expect } from '@playwright/test';

test.describe('Onboarding E2E CUJ', () => {
  test('User completes onboarding end-to-end', async ({ page }) => {
    // Navigate to the onboarding start page
    await page.route('**/api/onboarding/intake', route => route.fulfill({
      status: 200,
      contentType: 'application/json',
      body: JSON.stringify({
        business_type: 'Bakery',
        business_name: 'Test Bakery',
        categories: ['food'],
        initial_products: [{ name: 'Cake', price: '20' }]
      })
    }));

    await page.route('**/api/onboarding/start', route => route.fulfill({
      status: 200,
      contentType: 'application/json',
      body: JSON.stringify({ message: "Success!" })
    }));
    await page.goto('/onboarding');

    // Step 1: Start Onboarding Chat
    // Chat Step 1: Business Name
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Custom Cakes").fill('Test Bakery');
    await page.getByRole('button', { name: 'Next' }).click();

    // Chat Step 2: What do you sell
    await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();
    await page.getByPlaceholder('e.g. I bake custom vegan cakes for weddings and parties...').fill('Custom cakes and pastries');
    await page.getByRole('button', { name: 'Next' }).click();

    // Chat Step 3: Location
    await expect(page.getByRole('heading', { name: 'Where are you located?' })).toBeVisible();
    await page.getByPlaceholder('e.g. Portland, OR').fill('New York, NY');

    // Intake
    await page.getByRole('button', { name: 'Generate My Business' }).click();

    // Step 2: Review Details
    await expect(page.getByRole('heading', { name: 'Review Details' })).toBeVisible({ timeout: 15000 });
    // No getByDisplayValue in playwright, use locator
    await expect(page.locator('input[value="Test Bakery"]')).toBeVisible();

    // Fill in required admin email field that was added
    const adminEmailInput = page.getByPlaceholder('admin@example.com');
    await expect(adminEmailInput).toBeVisible();
    await adminEmailInput.fill('admin@testbakery.com');

    // Fill price field (type="number")
    const priceInput = page.locator('input[inputMode="decimal"]');
    await expect(priceInput).toBeVisible();
    await priceInput.fill('25.00');

    await page.getByRole('button', { name: 'Continue' }).click();

    // Step 3: Style & Team
    await expect(page.getByRole('heading', { name: 'Style & Team' })).toBeVisible();

    // Choose custom domain
    await page.getByText('Custom Domain').click();

    // Select an AI Agent
    await page.getByText('Sales Agent').click();

    // Finish onboarding
    await page.getByRole('button', { name: 'Launch Store' }).click();

    // Step 4: Loading Screen (Building Business)
    await expect(page.getByRole('heading', { name: 'Building Your Business...' })).toBeVisible({ timeout: 5000 });

    // Step 5: Success Screen (You're Live!)
    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 30000 });
    await expect(page.getByRole('link', { name: 'Go to Dashboard' })).toBeVisible();

    // Navigate to dashboard as final proof point
    await page.getByRole('link', { name: 'Go to Dashboard' }).click();

    // Simple verification that we reached dashboard
    await expect(page).toHaveURL(/.*\/dashboard/);
  });
});
