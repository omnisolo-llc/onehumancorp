import { test, expect } from '@playwright/test';

test.describe('Mobile Onboarding Comprehensive Flow', () => {

  test('should traverse the onboarding flow successfully - variant 0', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('founder_0@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();

    await page.waitForURL('**/dashboard');

    // Trigger setup wizard
    await page.locator('button:has-text("Start Setup")').click();
    await expect(page.locator('text=Your business, live in minutes.')).toBeVisible({ timeout: 5000 });

    // Step 0: Welcome
    await page.locator('button:has-text("🚀 Start My Business")').filter({ visible: true }).first().click();

    // Step 1: Business Name
    await expect(page.locator('text=1. What is your business called?')).toBeVisible();
    await page.locator('input[id="business-name"]').fill('Business 0');
    await page.locator('button:has-text("Next →")').filter({ visible: true }).first().click();

    // Step 2: Primary Service
    await expect(page.locator('text=2. What is your primary service?')).toBeVisible();
    await page.locator('input[id="business-service"]').fill('Service 0');
    await page.locator('button:has-text("Next →")').filter({ visible: true }).first().click();

    // Step 3: Preferred Language
    await expect(page.locator('text=3. Preferred language?')).toBeVisible();
    await page.locator('select[id="business-language"]').selectOption('en');
    await page.locator('button:has-text("Launch My Business →")').filter({ visible: true }).first().click();

    // Expect success screen
    await expect(page.locator('text=Building your storefront...')).toBeVisible({ timeout: 5000 });
  });

  test('should traverse the onboarding flow successfully - variant 1', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('founder_1@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();

    await page.waitForURL('**/dashboard');

    // Trigger setup wizard
    await page.locator('button:has-text("Start Setup")').click();
    await expect(page.locator('text=Your business, live in minutes.')).toBeVisible({ timeout: 5000 });

    // Step 0: Welcome
    await page.locator('button:has-text("🚀 Start My Business")').filter({ visible: true }).first().click();

    // Step 1: Business Name
    await expect(page.locator('text=1. What is your business called?')).toBeVisible();
    await page.locator('input[id="business-name"]').fill('Business 1');
    await page.locator('button:has-text("Next →")').filter({ visible: true }).first().click();

    // Step 2: Primary Service
    await expect(page.locator('text=2. What is your primary service?')).toBeVisible();
    await page.locator('input[id="business-service"]').fill('Service 1');
    await page.locator('button:has-text("Next →")').filter({ visible: true }).first().click();

    // Step 3: Preferred Language
    await expect(page.locator('text=3. Preferred language?')).toBeVisible();
    await page.locator('select[id="business-language"]').selectOption('en');
    await page.locator('button:has-text("Launch My Business →")').filter({ visible: true }).first().click();

    // Expect success screen
    await expect(page.locator('text=Building your storefront...')).toBeVisible({ timeout: 5000 });
  });

  test('should traverse the onboarding flow successfully - variant 2', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('founder_2@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();

    await page.waitForURL('**/dashboard');

    // Trigger setup wizard
    await page.locator('button:has-text("Start Setup")').click();
    await expect(page.locator('text=Your business, live in minutes.')).toBeVisible({ timeout: 5000 });

    // Step 0: Welcome
    await page.locator('button:has-text("🚀 Start My Business")').filter({ visible: true }).first().click();

    // Step 1: Business Name
    await expect(page.locator('text=1. What is your business called?')).toBeVisible();
    await page.locator('input[id="business-name"]').fill('Business 2');
    await page.locator('button:has-text("Next →")').filter({ visible: true }).first().click();

    // Step 2: Primary Service
    await expect(page.locator('text=2. What is your primary service?')).toBeVisible();
    await page.locator('input[id="business-service"]').fill('Service 2');
    await page.locator('button:has-text("Next →")').filter({ visible: true }).first().click();

    // Step 3: Preferred Language
    await expect(page.locator('text=3. Preferred language?')).toBeVisible();
    await page.locator('select[id="business-language"]').selectOption('en');
    await page.locator('button:has-text("Launch My Business →")').filter({ visible: true }).first().click();

    // Expect success screen
    await expect(page.locator('text=Building your storefront...')).toBeVisible({ timeout: 5000 });
  });

  test('should traverse the onboarding flow successfully - variant 3', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('founder_3@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();

    await page.waitForURL('**/dashboard');

    // Trigger setup wizard
    await page.locator('button:has-text("Start Setup")').click();
    await expect(page.locator('text=Your business, live in minutes.')).toBeVisible({ timeout: 5000 });

    // Step 0: Welcome
    await page.locator('button:has-text("🚀 Start My Business")').filter({ visible: true }).first().click();

    // Step 1: Business Name
    await expect(page.locator('text=1. What is your business called?')).toBeVisible();
    await page.locator('input[id="business-name"]').fill('Business 3');
    await page.locator('button:has-text("Next →")').filter({ visible: true }).first().click();

    // Step 2: Primary Service
    await expect(page.locator('text=2. What is your primary service?')).toBeVisible();
    await page.locator('input[id="business-service"]').fill('Service 3');
    await page.locator('button:has-text("Next →")').filter({ visible: true }).first().click();

    // Step 3: Preferred Language
    await expect(page.locator('text=3. Preferred language?')).toBeVisible();
    await page.locator('select[id="business-language"]').selectOption('en');
    await page.locator('button:has-text("Launch My Business →")').filter({ visible: true }).first().click();

    // Expect success screen
    await expect(page.locator('text=Building your storefront...')).toBeVisible({ timeout: 5000 });
  });

  test('should traverse the onboarding flow successfully - variant 4', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('founder_4@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();

    await page.waitForURL('**/dashboard');

    // Trigger setup wizard
    await page.locator('button:has-text("Start Setup")').click();
    await expect(page.locator('text=Your business, live in minutes.')).toBeVisible({ timeout: 5000 });

    // Step 0: Welcome
    await page.locator('button:has-text("🚀 Start My Business")').filter({ visible: true }).first().click();

    // Step 1: Business Name
    await expect(page.locator('text=1. What is your business called?')).toBeVisible();
    await page.locator('input[id="business-name"]').fill('Business 4');
    await page.locator('button:has-text("Next →")').filter({ visible: true }).first().click();

    // Step 2: Primary Service
    await expect(page.locator('text=2. What is your primary service?')).toBeVisible();
    await page.locator('input[id="business-service"]').fill('Service 4');
    await page.locator('button:has-text("Next →")').filter({ visible: true }).first().click();

    // Step 3: Preferred Language
    await expect(page.locator('text=3. Preferred language?')).toBeVisible();
    await page.locator('select[id="business-language"]').selectOption('en');
    await page.locator('button:has-text("Launch My Business →")').filter({ visible: true }).first().click();

    // Expect success screen
    await expect(page.locator('text=Building your storefront...')).toBeVisible({ timeout: 5000 });
  });
});
