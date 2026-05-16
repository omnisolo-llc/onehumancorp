import { test, expect } from '@playwright/test';

test.describe('New Wizards Validation', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
    await page.locator('input[placeholder="Email or Username"]').filter({ hasText: '' }).first().fill('test@example.com', { force: true });
    await page.locator('input[placeholder="Password"]').filter({ hasText: '' }).first().fill('password123', { force: true });
    await page.locator('button:has-text("Login")').filter({ hasText: '' }).first().click({ force: true });
    await page.waitForSelector('text="Welcome back, Human."', { timeout: 10000 });
  });

  test('should display Website Builder Onboarding', async ({ page }) => {
    await page.click('button:has-text("Build My Website")');
    await expect(page.locator('text=Brand colors & logo')).toBeVisible();
    await expect(page.locator('text=✨ Generate a logo for me')).toBeVisible();
  });

  test('should display AI Agent Configuration', async ({ page }) => {
    await page.click('button:has-text("Manage my AI team")');

    await expect(page.locator('text=Manage your AI team')).toBeVisible();
    await expect(page.locator('text=Customer Support')).toBeVisible();

    await page.click('button:has-text("Configure Agents →")');
    await expect(page.locator('text=What should they do?')).toBeVisible();

    await page.click('button:has-text("Next →")');
    await expect(page.locator('text=How often should they work?')).toBeVisible();
  });

  test('should display Prompt Tuning', async ({ page }) => {
    await page.click('button:has-text("Tune this agent")');

    await expect(page.locator('text=Tune Agent Personality')).toBeVisible();
    await expect(page.locator('button:has-text("😊 Friendly & Warm")')).toBeVisible();

    await page.click('button:has-text("Next →")');
    await expect(page.locator('text=Teach & Test')).toBeVisible();
  });

  test('should display Ongoing Wizards', async ({ page }) => {
    // Test Fix This
    await page.click('button:has-text("Help me fix this")');
    await expect(page.locator('text=Help me fix this')).toBeVisible();
    await page.click('button:has-text("Dismiss")');

    // Test Grow Business
    await page.click('button:has-text("Grow my business")');
    await expect(page.locator('text=Grow my business')).toBeVisible();
    await page.click('button:has-text("Back to Dashboard")');

    // Test Billing Wizard
    await page.click('button:has-text("What does this cost?")');
    await expect(page.locator('text=What does this cost?')).toBeVisible();
  });
});
