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

    // 1. Acquisition & Onboarding start (assuming a "Start Business Setup" or similar button is on dashboard,
    // or direct navigation if that's the only way from an empty dashboard)
    // For now we'll navigate directly to onboarding after login as a user starting the wizard
    await page.goto('/onboarding');

    // Wait for the Smart Builder welcome screen (Step 1)
    await expect(page.getByRole('heading', { name: "Tell us about your business" })).toBeVisible();

    // Fill in the bio
    await page.getByPlaceholder("e.g. I am Maya. I bake custom vegan cakes in Seattle. Maya's Cakes. Prices start at $50.").fill("I am Maya. I bake custom vegan cakes in Seattle. Maya's Cakes.");

    // Click Generate Draft
    await page.getByRole('button', { name: /Generate Draft/i }).click();

    // 2. Simplified Mobile First Onboarding - wait for it to generate
    await expect(page.getByRole('heading', { name: 'Looks Great!' })).toBeVisible({ timeout: 15000 });

    // Publish
    await page.getByRole('button', { name: /Publish Now/i }).click();

    // 3. Activation
    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });
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

    // Fill in the bio
    await page.getByPlaceholder("e.g. I am Maya. I bake custom vegan cakes in Seattle. Maya's Cakes. Prices start at $50.").fill("I am Carlos. I fix pipes and leaks. Carlos Plumbing.");

    // Click Generate Draft
    await page.getByRole('button', { name: /Generate Draft/i }).click();

    // 2. Simplified Mobile First Onboarding - wait for it to generate
    await expect(page.getByRole('heading', { name: 'Looks Great!' })).toBeVisible({ timeout: 15000 });

    // Publish
    await page.getByRole('button', { name: /Publish Now/i }).click();

    // 3. Activation
    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });
  });
});
