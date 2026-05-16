import { test, expect } from '@playwright/test';

test.describe('Business Setup Wizard Validation', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
    await page.click('button:has-text("Don\'t have an account? Sign Up")');
    await page.fill('input[placeholder="Email or Username"]', 'test@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('button:has-text("Sign Up")');
  });

  test('should traverse the complete 3-step business setup wizard successfully', async ({ page }) => {
    await page.click('text=Start Setup');

    // Step 1: Input details
    await expect(page.locator('text=Welcome to OHC!')).toBeVisible();
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company');
    await page.selectOption('select#business-category', 'Online Store');

    // Move to step 2 (AI Generation)
    await page.click('text=Next →');
    await expect(page.locator('text=Designing your storefront...')).toBeVisible();

    // Step 3 should auto-appear after generation
    await expect(page.locator('text=CONFETTI SUCCESS')).toBeVisible({ timeout: 10000 });

    // Verify exit state by clicking to Dashboard
    await page.click('text="Publish my business →"');
    await expect(page.locator('text="Dashboard"')).toBeVisible({ timeout: 5000 });
  });

  test('should display the Setup Wizard hero animation elements and complete full setup flow', async ({ page }) => {
    await page.click('text=Start Setup');
    await expect(page.locator('text=Welcome to OHC! Let\'s build your business.')).toBeVisible();
    await expect(page.locator('text=⚡ Instant Build (AI) →')).toBeVisible();

    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company Hero');
    await page.click('text=Next →');

    // Wait for the generation state
    await expect(page.locator('text=Designing your storefront...')).toBeVisible();

    // Wait for success
    await expect(page.locator('text=CONFETTI SUCCESS')).toBeVisible({ timeout: 10000 });
  });
});
