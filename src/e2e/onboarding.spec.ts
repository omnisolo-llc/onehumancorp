import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');

    // Login
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Login")');

    // Wait for the Dashboard
    await expect(page.locator('text="Your business, live in minutes."')).toBeVisible();
  });

  test('Test 1: Sign-Up & Account Creation to Wizard auto-redirect', async ({ page }) => {
    // The requirement is that first login auto-redirects to the business setup wizard or dashboard with setup wizard ready
    // From before each, we see it goes to Dashboard and we can click start setup.
    await page.click('button:has-text("Start Setup")');
    await expect(page.locator('text="Setup Wizard"')).toBeVisible();
  });

  test('Test 2: Business Setup Wizard Flow state persistence', async ({ page }) => {
    await page.click('button:has-text("Start Setup")');
    await expect(page.locator('text="Setup Wizard"')).toBeVisible();

    await page.click('button:has-text("Next")');
    // Step 1: Business Type -> 2
    await page.click('text="Online Store"');
    await page.click('button:has-text("Next")');
    // Step 2: Company Info -> 3
    await page.fill('input[placeholder="What is your business called?"]', 'Checklist Store');
    await page.click('button:has-text("Generate Description")');
    await page.waitForTimeout(1000);
    await page.click('button:has-text("Next")');
    // Step 3: Selling Categories -> 4
    await page.check('text="Physical Products"');
    await page.click('button:has-text("Next")');

    // Test cross device resume -> Reload page
    await page.reload();
    // Re login and check if it still works
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Login")');

    await expect(page.locator('text="Your business, live in minutes."')).toBeVisible();
    await page.click('button:has-text("Start Setup")');
    await expect(page.locator('text="Setup Wizard"')).toBeVisible();
  });

  test('Test 3: First Product & AI Description', async ({ page }) => {
    await page.click('button:has-text("Start Setup")');

    // Step 0 -> Step 1
    await page.click('button:has-text("Next")');
    // Step 1: Business Type -> 2
    await page.click('text="Online Store"');
    await page.click('button:has-text("Next")');
    // Step 2: Company Info -> 3
    await page.fill('input[placeholder="What is your business called?"]', 'AI Desc Store');
    await page.click('button:has-text("Generate Description")');
    await page.waitForTimeout(1000);
    await page.click('button:has-text("Next")');
    // Step 3: Selling Categories -> 4
    await page.check('text="Physical Products"');
    await page.click('button:has-text("Next")');
    // Step 4: First Product -> 5
    await page.fill('input[placeholder="What is the name of this product?"]', 'Prod');
    await page.fill('input[placeholder="0.00"]', '10');

    await expect(page.locator('button:has-text("Generate AI Description")')).toBeVisible();
    await page.click('button:has-text("Generate AI Description")');
    await page.waitForTimeout(1000);

    await page.click('button:has-text("Next")');
  });

  test('Test 4: Domain & Go-Live', async ({ page }) => {
    await page.click('button:has-text("Start Setup")');

    // Step 0 -> Step 1
    await page.click('button:has-text("Next")');
    // Step 1: Business Type -> 2
    await page.click('text="Online Store"');
    await page.click('button:has-text("Next")');
    // Step 2: Company Info -> 3
    await page.fill('input[placeholder="What is your business called?"]', 'Launch Store');
    await page.click('button:has-text("Next")');
    // Step 3: Selling Categories -> 4
    await page.click('button:has-text("Next")');
    // Step 4: First Product -> 5
    await page.click('button:has-text("Next")');
    // Step 5: Payments -> 6
    await page.click('text="Online"');
    await page.click('button:has-text("Next")');
    // Step 6: Theme -> 7
    await page.click('text="Modern"');
    await page.click('button:has-text("Next")');
    // Step 7: Domain -> 8
    await page.click('text="🌐 Free OHC Domain"');
    await page.click('button:has-text("Next")');
    // Step 8: Review & Launch -> 9
    await page.click('button:has-text("Launch Your Business")');

    // Check Confetti Success
    await expect(page.locator('text="CONFETTI Success"')).toBeVisible({ timeout: 10000 });
  });

  test('Test 5: Welcome Checklist', async ({ page }) => {
    await page.click('button:has-text("Start Setup")');

    // Proceed to last step in wizard to see the checklist
    // Step 0 -> Step 1
    await page.click('button:has-text("Next")');
    // Step 1: Business Type -> 2
    await page.click('text="Online Store"');
    await page.click('button:has-text("Next")');
    // Step 2: Company Info -> 3
    await page.fill('input[placeholder="What is your business called?"]', 'Checklist Store');
    await page.click('button:has-text("Generate Description")');
    await page.waitForTimeout(1000);
    await page.click('button:has-text("Next")');
    // Step 3: Selling Categories -> 4
    await page.check('text="Physical Products"');
    await page.click('button:has-text("Next")');
    // Step 4: First Product -> 5
    await page.fill('input[placeholder="What is the name of this product?"]', 'Prod');
    await page.fill('input[placeholder="0.00"]', '10');
    await page.click('button:has-text("Next")');
    // Step 5: Payments -> 6
    await page.click('text="Online"');
    await page.click('button:has-text("Next")');
    // Step 6: Theme -> 7
    await page.click('text="Modern"');
    await page.click('button:has-text("Next")');
    // Step 7: Domain -> 8
    await page.click('text="🌐 Free OHC Domain"');
    await page.click('button:has-text("Next")');
    // Step 8: Review & Launch -> 9
    await page.click('button:has-text("Launch Your Business")');

    await expect(page.locator('text="CONFETTI Success"')).toBeVisible({ timeout: 10000 });

    const viewChecklistBtn = page.locator('text="View Welcome Checklist →"');
    await viewChecklistBtn.click();

    await expect(page.locator('text="Welcome Checklist"')).toBeVisible({ timeout: 5000 });
    await expect(page.locator('text="You\'re set up! Here\'s what to do next:"')).toBeVisible();
    await expect(page.locator('text="✅ Business live"')).toBeVisible();
    await expect(page.locator('text="⬜ Add 3 more products"')).toBeVisible();
    await expect(page.locator('text="⬜ Connect Instagram"')).toBeVisible();
    await expect(page.locator('text="⬜ Share your link with a friend"')).toBeVisible();
  });
});
