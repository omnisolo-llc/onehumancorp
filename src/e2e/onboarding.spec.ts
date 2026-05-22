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
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();

    // Fill in the business name
    await page.getByPlaceholder("e.g. Maya's Cakes").fill("Maya's Cakes");

    // Click Next
    await page.getByRole('button', { name: /Next/i }).click();

    // Step 2
    await expect(page.getByRole('heading', { name: "What's your niche?" })).toBeVisible();

    // Fill in the niche
    await page.getByPlaceholder("e.g. I bake custom wedding cakes").fill("I bake custom vegan cakes");

    // Click Generate Draft
    await page.getByRole('button', { name: /Generate Draft/i }).click();

    // 2. Simplified Mobile First Onboarding - wait for it to generate
    await expect(page.getByRole('heading', { name: 'Looks Great!' })).toBeVisible({ timeout: 15000 });

    // Publish
    await page.getByRole('button', { name: /Publish Now/i }).click();

    // 3. Activation
    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });
  });

  test('Instant Build Flow', async ({ page }) => {
    // 0. Start from UI Login
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('maya@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    // Wait for Dashboard
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    // 1. Navigate to onboarding
    await page.goto('/onboarding');

    // Wait for the Smart Builder welcome screen (Step 1)
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();

    // Switch to Instant Build
    await page.getByRole('switch').click();
    await expect(page.getByRole('heading', { name: "Tell us about your business" })).toBeVisible();

    // Enter business description
    await page.getByPlaceholder('e.g. I bake custom wedding cakes in Brooklyn. My bestseller is the 3-tier vanilla cake for $300.').fill('I run a small boutique cafe in Seattle. We sell artisan coffee and pastries. Our signature is the lavender latte for $6.50.');

    // Click Generate Draft
    await page.getByRole('button', { name: /Generate Draft/i }).click();

    // Wait for it to generate and show the "Looks Great!" screen
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
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();

    // Fill in the business name
    await page.getByPlaceholder("e.g. Maya's Cakes").fill("Carlos Plumbing");

    // Click Next
    await page.getByRole('button', { name: /Next/i }).click();

    // Step 2
    await expect(page.getByRole('heading', { name: "What's your niche?" })).toBeVisible();

    // Fill in the niche
    await page.getByPlaceholder("e.g. I bake custom wedding cakes").fill("I fix pipes and leaks");

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
