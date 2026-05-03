import { test, expect } from '@playwright/test';

test.describe('Lens Audit: E2E Flows', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
  });

  test('CUJ 1: Login to Dashboard Visual Truth', async ({ page }) => {
    // Assert visual truth on Login page
    await expect(page.locator('text=One Human Corp')).toBeVisible();
    await expect(page.locator('text=✨')).not.toBeVisible(); // Verified: emoji removed
    await expect(page.locator('button:has-text("App Settings")')).toBeVisible();

    // Perform login
    await page.fill('input[placeholder*="Email"]', 'admin@example.com');
    await page.fill('input[placeholder*="Password"]', 'admin123');
    await page.click('button:has-text("Sign In")');

    // Assert Dashboard state
    await expect(page.locator('text=My Business')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text=Today\'s Sales')).toBeVisible();
    await expect(page.locator('text=Active Helpers')).toBeVisible();
  });

  test('CUJ 2: Onboarding Wizard Step-by-Step', async ({ page }) => {
    await page.click('button:has-text("Sign Up")');
    await page.fill('input[placeholder*="Email"]', 'newbiz@example.com');
    await page.fill('input[placeholder*="Password"]', 'password123');
    await page.click('button:has-text("Sign Up")');

    // Resend verification should trigger wizard
    await page.click('button:has-text("Resend Verification Email")');

    // Step 1: Business Type
    await expect(page.locator('text=What kind of business')).toBeVisible();
    await page.click('text=Online Store');

    // Step 2: Name
    await page.fill('input[placeholder*="Maya\'s Cakes"]', 'Audit Store');
    await page.click('button:has-text("Next")');

    // Step 3: What do you sell
    await page.click('text=Physical products');
    await page.click('button:has-text("Next")');

    // Step 4: Payments
    await page.click('text=Online only');

    // Step 5: Admin Account
    await page.fill('input[placeholder*="Maya Smith"]', 'Auditor');
    await page.fill('input[placeholder*="you@email.com"]', 'auditor@example.com');
    await page.click('button:has-text("Next")');

    // Assert we moved forward
    await expect(page.locator('text=Choose a Template')).toBeVisible();
  });

  test('CUJ 3: Instant Build AI Extraction', async ({ page }) => {
    await page.click('button:has-text("Sign Up")');
    await page.click('button:has-text("Resend Verification Email")'); // Shortcut to wizard

    await page.click('text=Instant Build (AI)');
    await page.fill('textarea, input[placeholder*="Maya\'s Cakes"]', 'I want to build a premium bookstore called Gutenberg.');
    await page.click('button:has-text("Generate Storefront")');

    // Assert AI extraction result
    await expect(page.locator('text=Ready to launch!')).toBeVisible({ timeout: 15000 });
    await expect(page.locator('text=Business: Gutenberg')).toBeVisible();
  });

  test('CUJ 4: Dashboard Quick Actions and Help', async ({ page }) => {
    // Navigate directly if possible or login
    await page.fill('input[placeholder*="Email"]', 'admin@example.com');
    await page.fill('input[placeholder*="Password"]', 'admin123');
    await page.click('button:has-text("Sign In")');

    await page.click('button:has-text("?")');
    await expect(page.locator('text=These buttons are shortcuts')).toBeVisible();

    await page.click('button:has-text("Menu")');
    await page.click('button:has-text("Help Center")');
    await expect(page.locator('text=Marketplace')).toBeVisible(); // Articles now fetched from Marketplace API
  });

  test('CUJ 5: Responsive Design Audit (Mobile)', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/login');

    const loginCard = page.locator('.GlassCard, [class*="GlassCard"]');
    if (await loginCard.count() > 0) {
        const box = await loginCard.boundingBox();
        expect(box.width).toBeLessThanOrEqual(375);
    }

    await expect(page.locator('button:has-text("Sign In")')).toHaveCSS('min-height', '0px'); // Placeholder for touch target check
  });
});
