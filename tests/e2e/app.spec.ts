import { test, expect } from '@playwright/test';

test.describe('Dashboard', () => {
  test('should load dashboard page', async ({ page }) => {
    await page.goto('/');
    await expect(page).toHaveTitle(/OneHuman/);
  });

  test('should display navigation', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('nav')).toBeVisible();
  });
});

test.describe('Business Setup Wizard', () => {
  test('should show welcome step', async ({ page }) => {
    await page.goto('/business-setup');
    await expect(page.locator('text=Welcome')).toBeVisible();
  });

  test('should navigate through wizard steps', async ({ page }) => {
    await page.goto('/business-setup');

    // Step 0: Welcome -> Next
    const nextButton = page.locator('button:has-text("Next")');
    await nextButton.click();

    // Step 1: Business type
    await page.locator('input[type="text"]').first().fill('Online Store');
    await nextButton.click();

    // Step 2: Company name
    await page.locator('input[type="text"]').first().fill('Test Company');
    await nextButton.click();

    // Verify we can proceed through steps
    await expect(page.locator('text=What do you sell')).toBeVisible();
  });

  test('should reach Welcome Checklist and show interactive items', async ({ page }) => {
    // E2E Standard: Start from home page, login via UI
    await page.goto('/');

    // Simulate login
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();

    // Navigate to business setup
    await page.goto('/business-setup');

    // Navigate through steps to reach Step 10
    // Step 0: Welcome
    await page.locator('button:has-text("Get Started →")').click();

    // Step 1: Type
    await page.locator('text=Online Store').click(); // Simulating selecting type, though the exact Slint dom mapping might vary. Just clicking Next.
    await page.locator('button:has-text("Next →")').click();

    // Step 2: Name
    await page.locator('input').first().fill('Test Company');
    await page.locator('button:has-text("Next →")').click();

    // Step 3: What do you sell
    await page.locator('text=Physical Products').click();
    await page.locator('button:has-text("Next →")').click();

    // Step 4: Payments
    await page.locator('text=Online Payments').click();
    // Next happens automatically on select usually, or we click next

    // Step 5: Admin
    await page.locator('input').first().fill('admin@test.com');
    await page.locator('button:has-text("Next →")').click();

    // Step 6: Template
    await page.locator('text=Modern').click();

    // Step 7: Product
    await page.locator('input').first().fill('My Product');
    await page.locator('button:has-text("Next →")').click();

    // Step 8: Domain
    await page.locator('text=Free OHC Domain').click();

    // Step 9: Launch
    await page.locator('button:has-text("Launch My Business →")').click();

    // Wait for launch to complete (status updates)
    await expect(page.locator('text=Onboarding Complete!')).toBeVisible({ timeout: 15000 });

    // Ensure we are now at Step 10 by checking for the checklist items.
    // Assert the presence of the checklist text strictly.
    await expect(page.locator('text=You\'re set up! Here\'s what to do next:')).toBeVisible();
    await expect(page.locator('text=Add 3 more products')).toBeVisible();
    await expect(page.locator('text=Connect Instagram')).toBeVisible();
    await expect(page.locator('text=Share your link with a friend')).toBeVisible();
  });
});

test.describe('Login', () => {
  test('should show login form', async ({ page }) => {
    await page.goto('/login');
    await expect(page.locator('input[type="email"]')).toBeVisible();
    await expect(page.locator('input[type="password"]')).toBeVisible();
  });

  test('should allow password visibility toggle', async ({ page }) => {
    await page.goto('/login');
    const passwordInput = page.locator('input[type="password"]');
    const toggleButton = page.locator('button:has-text("Show")');
    await expect(toggleButton).toBeVisible();
  });
});

test.describe('Agent Management', () => {
  test('should show agents list', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.locator('h1:has-text("Agents")')).toBeVisible();
  });

  test('should show hire agent button', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.locator('button:has-text("Hire Agent")')).toBeVisible();
  });
});
