import { test, expect } from '@playwright/test';

test.describe('Conversational Onboarding E2E', () => {
  test('should go through the full 3-question conversational setup flow', async ({ page }) => {
    // Navigate to the app root
    await page.goto('/');

    // E2E requirement: must start from home page UI after login
    // Login
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Sign In"), button:has-text("Login")');

    // Wait for the Dashboard
    await expect(page.locator('text="Welcome"')).toBeVisible();

    // Click the Conversational Onboarding button we added
    await page.click('button:has-text("Conversational AI Setup")');

    // Verify chat UI appears
    await expect(page.locator('text="AI Setup"')).toBeVisible();
    await expect(page.locator('text="What is your business called?"')).toBeVisible();

    // Answer Q1: Business Name
    await page.fill('input[placeholder="Type your answer..."]', 'My AI Bakery');
    await page.click('button:has-text("Send")');

    // Wait for Q2: Category
    await expect(page.locator('text=/What category does My AI Bakery fall into/')).toBeVisible({ timeout: 5000 });

    // Answer Q2
    await page.fill('input[placeholder="Type your answer..."]', 'Food');
    await page.click('button:has-text("Send")');

    // Wait for Q3: Goal
    await expect(page.locator('text=/what is your primary goal right now/')).toBeVisible({ timeout: 5000 });

    // Answer Q3
    await page.fill('input[placeholder="Type your answer..."]', 'Take pre-orders');
    await page.click('button:has-text("Send")');

    // Verify Completion
    await expect(page.locator('text="Perfect. We are setting up your business now..."')).toBeVisible({ timeout: 5000 });

    // Check for transition to Dashboard
    await page.click('button:has-text("Go to Dashboard")');
    await expect(page.locator('text="Welcome"')).toBeVisible();
  });
});
