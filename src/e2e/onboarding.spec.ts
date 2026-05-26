import { test, expect } from './fixtures';

test.describe('Onboarding Wizard', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('Maya (The Home Baker) onboarding flow', async ({ page }) => {
    await page.route('**/api/onboarding/intake', async route => route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ business_name: "Maya's Cakes", business_type: "Bakery", categories: ["food", "physical"], initial_products: [{ name: "Custom Vegan Cake", price: "45.00" }] }) }));
    // 0. Start from UI Login
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('maya@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    // Wait for Dashboard
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    // 1. Acquisition & Onboarding start (assuming a "Start Business Setup" or similar button is on dashboard,
    // or direct navigation if that's the only way from an empty dashboard)
    // For now we'll navigate directly to onboarding after login as a user starting the wizard
    await page.goto('/onboarding');

    // Wait for the Smart Builder welcome screen (Step 1)
    await expect(page.getByRole('heading', { name: "What do you do?" })).toBeVisible();

    // Fill in the business type
    await page.getByPlaceholder("e.g. Sell cakes, plumbing").fill("Sell custom cakes");

    // Click Next
    await page.getByRole('button', { name: /Next/i }).click();

    // Step 2
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();

    // Fill in the business name
    await page.getByPlaceholder("e.g. Maya's Cakes").fill("Maya's Cakes");

    // Click Next
    await page.getByRole('button', { name: /Next/i }).click();

    // Step 3
    await expect(page.getByRole('heading', { name: "What's your niche?" })).toBeVisible();

    // Fill in the niche
    await page.getByPlaceholder("e.g. I bake custom wedding cakes").fill("I bake custom vegan cakes");

    // Click Generate Draft
    await page.getByRole('button', { name: /Generate Draft/i }).click();

    // 2. Simplified Mobile First Onboarding - wait for it to generate
    await expect(page.getByRole('heading', { name: 'Ready to Launch!' })).toBeVisible({ timeout: 15000 });

    // Verify keyboard optimizations for price input
    const priceInput = page.getByPlaceholder('0.00');
    await expect(priceInput).toHaveAttribute('inputMode', 'decimal');
    await expect(priceInput).toHaveAttribute('pattern', '[0-9]*\\.?[0-9]*');

    // Verify glassmorphism aesthetics applied
    await expect(priceInput).toHaveClass(/backdrop-blur/);

    // Configure products and domain before publishing
    await page.getByRole('button', { name: 'Playful' }).click();
    await page.getByRole('button', { name: /Connect Custom Domain/i }).click();

    // Publish
    await page.getByRole('button', { name: /Publish Now/i }).click();

    // 3. Activation
    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });

    // 4. Verify Dashboard redirect and action banner
    await page.getByRole('link', { name: /Go to Dashboard/i }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });
    await expect(page.locator('text=1 Action Required: Connect Stripe to accept payments.')).toBeVisible();
  });

  test('Carlos (Handyman) onboarding flow', async ({ page }) => {
    await page.route('**/api/onboarding/intake', async route => route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ business_name: "Carlos Plumbing", business_type: "Service", categories: ["service"], initial_products: [{ name: "Pipe Fix", price: "80.00" }] }) }));
    // 0. Start from UI Login
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('carlos@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    // Wait for Dashboard
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    // 1. Acquisition & Onboarding start
    await page.goto('/onboarding');

    // Wait for the Smart Builder welcome screen (Step 1)
    await expect(page.getByRole('heading', { name: "What do you do?" })).toBeVisible();

    // Fill in the business type
    await page.getByPlaceholder("e.g. Sell cakes, plumbing").fill("Plumbing");

    // Click Next
    await page.getByRole('button', { name: /Next/i }).click();

    // Step 2
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();

    // Fill in the business name
    await page.getByPlaceholder("e.g. Maya's Cakes").fill("Carlos Plumbing");

    // Click Next
    await page.getByRole('button', { name: /Next/i }).click();

    // Step 3
    await expect(page.getByRole('heading', { name: "What's your niche?" })).toBeVisible();

    // Fill in the niche
    await page.getByPlaceholder("e.g. I bake custom wedding cakes").fill("I fix pipes and leaks");

    // Click Generate Draft
    await page.getByRole('button', { name: /Generate Draft/i }).click();

    // 2. Simplified Mobile First Onboarding - wait for it to generate
    await expect(page.getByRole('heading', { name: 'Ready to Launch!' })).toBeVisible({ timeout: 15000 });

    // Verify keyboard optimizations for price input
    const priceInput = page.getByPlaceholder('0.00');
    await expect(priceInput).toHaveAttribute('inputMode', 'decimal');

    // Configure products and domain before publishing
    await page.getByRole('button', { name: 'Modern' }).click();

    // Publish
    await page.getByRole('button', { name: /Publish Now/i }).click();

    // 3. Activation
    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });

    // 4. Verify Dashboard redirect and action banner
    await page.getByRole('link', { name: /Go to Dashboard/i }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });
    await expect(page.locator('text=1 Action Required: Connect Stripe to accept payments.')).toBeVisible();
  });
});
