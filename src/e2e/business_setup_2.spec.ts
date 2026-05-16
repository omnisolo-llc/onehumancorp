import { test, expect } from '@playwright/test';

test.describe('E2E Onboarding Persona Journeys', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
    await page.click('button:has-text("Don\'t have an account? Sign Up")');
    await page.fill('input[placeholder="Email or Username"]', 'test_user@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('button:has-text("Sign Up")');
  });

  test('Persona: Leo - The Music Tutor (Subscriptions)', async ({ page }) => {
    await page.click('text=Start Setup');
    await expect(page.locator('text=Welcome to OHC!')).toBeVisible();

    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', "Leo Music");
    await page.selectOption('select#business-category', 'Service Business');
    await page.click('text=Next →');

    // Wait for the generation state
    await expect(page.locator('text=Designing your storefront...')).toBeVisible();

    // Wait for success
    await expect(page.locator('text=CONFETTI SUCCESS')).toBeVisible({ timeout: 10000 });

    await page.click('text="Publish my business →"');
    await expect(page.locator('text="Dashboard"')).toBeVisible({ timeout: 5000 });
  });

  test('Persona: Fatima - The Food Cart (Pre-orders)', async ({ page }) => {
    await page.click('text=Start Setup');
    await expect(page.locator('text=Welcome to OHC!')).toBeVisible();

    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', "Fatima Cart");
    await page.selectOption('select#business-category', 'Restaurant / Food');
    await page.click('text=Next →');

    // Wait for the generation state
    await expect(page.locator('text=Designing your storefront...')).toBeVisible();

    // Wait for success
    await expect(page.locator('text=CONFETTI SUCCESS')).toBeVisible({ timeout: 10000 });

    await page.click('text="Publish my business →"');
    await expect(page.locator('text="Dashboard"')).toBeVisible({ timeout: 5000 });
  });
});

test.describe('E2E Onboarding Persona Journeys - Portfolio', () => {
  test('Persona: Alex - The Artist (Portfolios)', async ({ page }) => {
    await page.goto('/login');
    await page.click('button:has-text("Don\'t have an account? Sign Up")');
    await page.fill('input[placeholder="Email or Username"]', 'alex@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('button:has-text("Sign Up")');

    await page.click('text=Start Setup');
    await expect(page.locator('text=Welcome to OHC!')).toBeVisible();

    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', "Alex Studio");
    await page.selectOption('select#business-category', 'Creative');
    await page.click('text=Next →');

    // Wait for the generation state
    await expect(page.locator('text=Designing your storefront...')).toBeVisible();

    // Wait for success
    await expect(page.locator('text=CONFETTI SUCCESS')).toBeVisible({ timeout: 10000 });

    await page.click('text="Publish my business →"');
    await expect(page.locator('text="Dashboard"')).toBeVisible({ timeout: 5000 });
  });
});

test.describe('Instant Build (AI) Flow', () => {
  test('Instant Build Journey - Full Success', async ({ page }) => {
    await page.goto('/login');
    await page.click('button:has-text("Don\'t have an account? Sign Up")');
    await page.fill('input[placeholder="Email or Username"]', 'ai_user@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('button:has-text("Sign Up")');

    await page.click('text=Start Setup');
    await expect(page.locator('text=Welcome to OHC!')).toBeVisible();
    await page.click('text=⚡ Instant Build (AI) →');

    await expect(page.locator('text=Describe your business in a sentence')).toBeVisible();

    await page.fill('input[placeholder="e.g. I run a local bakery called Maya\'s Cakes..."]', 'I run a custom vegan cake shop in Austin called Austin Vegan Cakes.');

    await expect(page.locator('text=Generate Storefront →')).toBeVisible();
    await page.click('text=Generate Storefront →');

    // Wait for the generation state
    await expect(page.locator('text=Designing your storefront...')).toBeVisible();

    // Check we arrive at Step 9
    await expect(page.locator('text="Launch My Business →"')).toBeVisible({ timeout: 15000 });

    await page.click('text="Launch My Business →"');

    await expect(page.locator('text="Dashboard"')).toBeVisible({ timeout: 5000 });
  });

  test('Instant Build Journey - Back button behavior', async ({ page }) => {
    await page.goto('/login');
    await page.click('button:has-text("Don\'t have an account? Sign Up")');
    await page.fill('input[placeholder="Email or Username"]', 'ai_user3@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('button:has-text("Sign Up")');

    await page.click('text=Start Setup');
    await page.click('text=⚡ Instant Build (AI) →');

    // Test the back button works from the instant input step
    await page.click('text=Back');
    await expect(page.locator('text=Welcome to OHC!')).toBeVisible();
  });
});
