import { test, expect } from './fixtures';

test.describe('Onboarding Wizard', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('Maya (The Home Baker) onboarding flow', async ({ page }) => {
    // 0. Start from UI Login
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('maya@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    // Wait for Dashboard
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    // 1. Acquisition & Onboarding start
    await page.goto('/onboarding');

    // Wait for the Smart Builder welcome screen (Step 1)
    await expect(page.getByRole('heading', { name: "Tell us about your business" })).toBeVisible();

    // Fill in the one-pager business bio
    await page.getByPlaceholder(/e\.g\. My name is Maya/i).fill("My name is Maya and I bake custom vegan cakes for weddings. My business is called Maya's Cakes.");

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
    await expect(page.getByRole('heading', { name: "Tell us about your business" })).toBeVisible();

    // Fill in the one-pager business bio
    await page.getByPlaceholder(/e\.g\. My name is Maya/i).fill("I am Carlos and I run Carlos Plumbing. I fix pipes and leaks.");

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
