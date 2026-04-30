import { test, expect } from '@playwright/test';

test.describe('Scribe Documentation Features', () => {
  test.beforeEach(async ({ page }) => {
    // Start from the home page (Dashboard)
    await page.goto('http://localhost:3000'); // Adjust to actual local dev URL if needed
  });

  test('Help Center Search Logic', async ({ page }) => {
    // Open Help Center
    await page.click('button:has-text("?")');
    await expect(page.getByText('How can we help?')).toBeVisible();

    // Check initial visibility of categories
    await expect(page.getByText('Getting Started')).toBeVisible();
    await expect(page.getByText('My Store')).toBeVisible();
    await expect(page.getByText('Payments & Billing')).toBeVisible();

    // Search for "payments"
    await page.fill('input[placeholder="Search help articles..."]', 'payments');

    // "Payments & Billing" should still be visible
    await expect(page.getByText('Payments & Billing')).toBeVisible();

    // "My Store" should be hidden (not visible)
    await expect(page.getByText('My Store')).not.toBeVisible();
  });

  test('Interactive Walkthrough Steps', async ({ page }) => {
    // Start walkthrough from Dashboard
    await page.click('button:has-text("Take a Tour")');
    await expect(page.getByText('Welcome to your store!')).toBeVisible();

    // Step 1
    await page.click('button:has-text("Next")');
    await expect(page.getByText('Add your first product')).toBeVisible();

    // Step 2
    await page.click('button:has-text("Next")');
    await expect(page.getByText('Set up payments')).toBeVisible();

    // Step 3
    await page.click('button:has-text("Next")');
    await expect(page.getByText('Activate AI Support')).toBeVisible();

    // Step 4 (Final)
    await page.click('button:has-text("Next")');
    await expect(page.getByText("You're all set!")).toBeVisible();

    await page.click('button:has-text("Done")');
    await expect(page.getByText("You're all set!")).not.toBeVisible();
  });

  test('Tooltips Interaction', async ({ page }) => {
    // Hover over a stat card
    await page.hover('text=Agents Working Now');
    // Check for tooltip text from registry
    await expect(page.getByText('The number of AI agents currently working on tasks for your business.')).toBeVisible();
  });

  test('AI Help Chat and Article Links', async ({ page }) => {
    // Open AI Chat
    await page.click('button:has-text("Ask anything")');
    await expect(page.getByText('✨ OHC Assistant')).toBeVisible();

    // Verify AI welcome message
    await expect(page.getByText("Hi! I'm your OHC Help Assistant.")).toBeVisible();

    // Verify article link presence
    await expect(page.getByText('📖 Read: Connecting your bank')).toBeVisible();
  });

  test('Release Notes Versioning', async ({ page }) => {
    // Open Release Notes
    await page.click('button:has-text("✨ What\'s New")');
    await expect(page.getByText("What's New in OHC")).toBeVisible();
    await expect(page.getByText("v1.2.0")).toBeVisible();
    await expect(page.getByText("✨ Multi-language Support")).toBeVisible();
  });
});
