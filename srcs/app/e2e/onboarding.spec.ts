import { test, expect } from '@playwright/test';

test.describe('Onboarding Flow (CUJ)', () => {
  test('User signs up, verifies email, completes wizard, and sees checklist', async ({ page }) => {
    // 1. Visit the app and go to Sign In/Sign Up
    await page.goto('/');

    // Native app might show a Landing page first. Let's try going to /login directly or click the login button
    await page.goto('/login');

    // Toggle to Sign Up mode
    await page.click('text="Don\'t have an account? Sign Up"');

    // Fill in sign up fields
    await page.fill('input[type="text"]', 'maya_baker'); // username
    await page.fill('input[type="email"]', 'maya@example.com'); // email
    await page.fill('input[type="password"]', 'password123'); // password

    // Submit Sign Up
    await page.click('button:has-text("Sign Up")');

    // 2. Email Verification Screen
    await expect(page.locator('text="Verify Your Email"')).toBeVisible();
    await expect(page.locator('text="Resend Verification Email"')).toBeVisible();
    await page.click('text="I have verified my email ->"');

    // 3. Business Setup Wizard
    await expect(page.locator('text="Business Setup"')).toBeVisible();
    await expect(page.locator('text="Welcome! Your AI team, ready in minutes."')).toBeVisible();
    await page.click('button:has-text("Next")');

    // Step 1: Business type
    await expect(page.locator('text="Business type"')).toBeVisible();
    await page.click('button:has-text("Next")');

    // Step 2: Business name and description
    await expect(page.locator('text="Business name"')).toBeVisible();
    await page.click('button:has-text("Next")');

    // Step 3: What you sell
    await expect(page.locator('text="What do you sell?"')).toBeVisible();
    await page.click('button:has-text("Next")');

    // Step 4: Payments
    await expect(page.locator('text="How do you want to receive payments?"')).toBeVisible();
    await page.click('button:has-text("Next")');

    // Step 5: Template
    await expect(page.locator('text="Template Selection"')).toBeVisible();
    await page.click('button:has-text("Next")');

    // Step 6: First Product
    await expect(page.locator('text="First Product / Service"')).toBeVisible();
    await page.click('button:has-text("Next")');

    // Step 7: Domain & Launch
    await expect(page.locator('text="Domain & Go-Live"')).toBeVisible();
    await page.click('button:has-text("Launch My Business →")');

    // 4. Welcome Checklist
    await expect(page.locator('text="You\'re set up! Here\'s what to do next:"')).toBeVisible();
    await expect(page.locator('text="Business Live"')).toBeVisible();
    await expect(page.locator('text="Add 3 more products"')).toBeVisible();

    // Toggle a task
    await page.click('text="Add 3 more products"');

    // Proceed to Dashboard
    await page.click('button:has-text("Go to my Dashboard ->")');

    // Verify we arrived at dashboard
    await expect(page.locator('text="Dashboard"')).toBeVisible();
  });
});
