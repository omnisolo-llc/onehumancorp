import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard Additional Journeys', () => {

  test('Instant Build Flow - Carlos the Handyman', async ({ page }) => {
    await page.goto('/');

    await page.fill('input[type="email"]', 'carlos@example.com');
    await page.fill('input[type="password"]', 'pass123');
    await page.click('button:has-text("Sign In")');

    await expect(page.locator('text="Your business, live in minutes."')).toBeVisible({ timeout: 10000 });
    await page.click('button:has-text("Start Setup")');
    await expect(page.locator('text="Setup Wizard"')).toBeVisible();

    // Toggle Instant Build mode
    await page.click('button:has-text("Use Instant Build")');

    // Fill bio
    await page.fill('textarea[placeholder="e.g. I\'m Carlos, a freelance handyman..."]', "I'm Carlos, a freelance handyman offering home repair services.");

    // Generate
    await page.click('button:has-text("Generate Business Setup")');
    await page.waitForTimeout(2000); // Simulate API call wait

    // Should skip directly to review
    await expect(page.locator('text="Review & Launch"')).toBeVisible();
    await page.click('button:has-text("Launch!")');

    await expect(page.locator('text="Onboarding Complete!"')).toBeVisible({ timeout: 15000 });
  });

  test('Maya The Home Baker - Full flow with product addition and free domain', async ({ page }) => {
    await page.goto('/');

    // Sign up
    await page.click('button:has-text("New here? Create an account")');
    await page.fill('input[type="email"]', 'maya@example.com');
    await page.fill('input[type="password"]', 'securebaker123');
    await page.click('button:has-text("Sign Up")');

    // Wait for Dashboard to show Wizard auto-trigger
    await expect(page.locator('text="Your business, live in minutes."')).toBeVisible({ timeout: 10000 });
    await page.click('button:has-text("Start Setup")');
    await expect(page.locator('text="Setup Wizard"')).toBeVisible();

    // 0 -> 1: Intro
    await page.click('button:has-text("Next")');

    // Step 1: Business Type
    await page.click('text="Food & Beverage"');
    await page.click('button:has-text("Next")');

    // Step 2: Company Info
    await page.fill('input[placeholder="What is your business called?"]', 'Mayas Cakes');
    await page.click('button:has-text("Generate Description")');
    await page.waitForTimeout(1000); // mock generation
    await page.click('button:has-text("Next")');

    // Step 3: Categories
    await page.check('text="Physical Products"');
    await page.click('button:has-text("Next")');

    // Step 4: First Product
    await page.fill('input[placeholder="What is the name of this product?"]', 'Vegan Chocolate Cake');
    await page.fill('input[placeholder="0.00"]', '45.00');
    await page.click('button:has-text("Next")');

    // Step 5: Payment
    await page.click('text="Both (In-person & Online)"');
    await page.click('button:has-text("Next")');

    // Step 6: Theme
    await page.click('text="Modern"');
    await page.click('button:has-text("Next")');

    // Step 7: Domain
    await page.click('text="🌐 Free OHC Domain"');
    await page.click('button:has-text("Next")');

    // Step 8: Admin Info
    await page.fill('input[placeholder="Your Full Name"]', 'Maya Baker');
    await page.fill('input[placeholder="your@email.com"]', 'maya@example.com');
    await page.fill('input[placeholder="Create a strong password"]', 'securebaker123');
    await page.click('button:has-text("Review & Launch")');

    // Launch Step
    await expect(page.locator('text="Almost there"')).toBeVisible({ timeout: 5000 });
    await page.click('button:has-text("Launch!")');

    // Verify completion
    await expect(page.locator('text="Onboarding Complete!"')).toBeVisible({ timeout: 15000 });

    // Checklist
    await page.click('button:has-text("View Welcome Checklist →")');
    await expect(page.locator('text="Welcome Checklist"')).toBeVisible();

    // Verify all items exist
    await expect(page.locator('text="Business live"')).toBeVisible();
    await expect(page.locator('text="Add 3 more products"')).toBeVisible();
    await expect(page.locator('text="Connect Instagram"')).toBeVisible();
    await expect(page.locator('text="Share your link with a friend"')).toBeVisible();

    // Ensure it goes to dashboard from checklist
    await page.click('button:has-text("Go to Dashboard")');
    await expect(page.locator('text="Your business, live in minutes."')).toBeVisible();
  });

  test('Cross Device Resume Persistence', async ({ page }) => {
    // Session 1: Mobile Start
    await page.goto('/');

    await page.fill('input[type="email"]', 'carlos@example.com');
    await page.fill('input[type="password"]', 'pass123');
    await page.click('button:has-text("Sign In")');

    await expect(page.locator('text="Your business, live in minutes."')).toBeVisible();
    await page.click('button:has-text("Start Setup")');

    await page.click('button:has-text("Next")');

    // Enter Business Type
    await page.click('text="Service Provider"');
    await page.click('button:has-text("Next")');

    // Enter Name
    await page.fill('input[placeholder="What is your business called?"]', 'Carlos Handyman');

    // Hard refresh page to simulate closing app
    await page.context().clearCookies(); await page.goto("/");

    // Session 2: Desktop Resume
    await page.fill('input[type="email"]', 'carlos@example.com');
    await page.fill('input[type="password"]', 'pass123');
    await page.click('button:has-text("Sign In")');

    await expect(page.locator('text="Your business, live in minutes."')).toBeVisible();
    await page.click('button:has-text("Start Setup")');

    // Check we resumed
    await expect(page.locator('text="Setup Wizard"')).toBeVisible();
  });
});
