import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard Flow', () => {

  test('Test 1: Full Journey from signup to Live', async ({ page }) => {
    // 1. Visit signup screen
    await page.goto('/signup');

    // 2. Click start setup wizard (instead of signup for this test route)
    await page.click('button:has-text("Start Setup")');
    await expect(page.locator('text="Your business, live in minutes."')).toBeVisible();

    // Step 0 -> Step 1
    await page.click('button:has-text("Start My Business")');
    // Step 1: Business Type -> 2
    await page.click('text="Online Store"');

    // Step 2: Company Info -> 3
    await page.fill('input[placeholder="What is your business called?"]', 'My Onboarding Store');
    await page.click('button:has-text("Generate Description")');
    await page.waitForTimeout(1000); // Simulate generation delay
    await page.click('button:has-text("Next")');

    // Step 3: Selling Categories -> 4
    await page.click('text="Physical products"');
    await page.click('button:has-text("Next")');

    // Step 4: First Product -> 5
    await page.fill('input[placeholder="What is the name of this product?"]', 'Prod');
    await page.fill('input[placeholder="0.00"]', '10');
    await page.click('button:has-text("Next")');

    // Step 5: Payments -> 6
    await page.click('text="Online only"');

    // Step 6: Admin account -> 7
    await page.fill('input[placeholder="e.g. Maya Smith"]', 'Maya Smith');
    await page.fill('input[placeholder="you@email.com"]', 'maya@test.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('button:has-text("Next")');

    // Step 7: Theme -> 8
    await page.click('button:has-text("Modern")');

    // Step 8: Domain -> 9
    await page.click('button:has-text("Free OHC Domain")');
    await page.click('button:has-text("Next")');

    // Step 9: Review & Launch -> 100
    await page.click('button:has-text("Launch!")');

    // Wait for the confetti screen
    await expect(page.locator('text="Success! Your business is live!"')).toBeVisible({ timeout: 10000 });
  });

});
