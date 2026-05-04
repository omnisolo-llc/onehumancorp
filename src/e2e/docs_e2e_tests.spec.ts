import { test, expect } from '@playwright/test';

test.beforeEach(async ({ page }) => {
  await page.goto('/');
  await page.fill('input[type="email"]', 'test@example.com');
  await page.fill('input[type="password"]', 'password123');
  await page.click('button:has-text("Login")');
  await expect(page.locator('text="My Business"')).toBeVisible();
});

test('Help Center Search', async ({ page }) => {
  await page.click('button:has-text("Menu")');
  await page.click('button:has-text("Help Center")');
  await expect(page.locator('text="How can we help?"')).toBeVisible();

  await page.fill('input[placeholder="Search help articles..."]', 'Apple Pay');
  await expect(page.locator('text="How to accept Apple Pay"')).toBeVisible();
});

test('Interactive Walkthrough Progression', async ({ page }) => {
  await page.click('button:has-text("Menu")');
  await page.click('button:has-text("App Tour")');

  await expect(page.locator('text="Welcome to your store!"')).toBeVisible();
  await page.click('button:has-text("Next")');

  await expect(page.locator('text="Add your first product"')).toBeVisible();
  await page.click('button:has-text("Next")');

  await expect(page.locator('text="Set up your storefront"')).toBeVisible();
  await page.click('button:has-text("Next")');

  await expect(page.locator('text="You\'re ready to grow!"')).toBeVisible();
  await page.click('button:has-text("Done")');

  await expect(page.locator('text="Welcome to your store!"')).toBeHidden();
});

test('AI Help Chat Interaction', async ({ page }) => {
  await page.click('button:has-text("Ask AI")');
  await expect(page.locator('text="Ask OHC Support"')).toBeVisible();

  await page.fill('input[placeholder="Type your question here..."]', 'How do I get paid?');
  await page.click('button:has-text("Send")');

  // Verify user message appears (mocked)
  await expect(page.locator('text="How do I get paid?"')).toBeVisible();
});

test('Video Tutorials Visibility', async ({ page }) => {
  await page.click('button:has-text("Menu")');
  await page.click('button:has-text("Video Tutorials")');
  await expect(page.locator('text="Watch & Learn"')).toBeVisible();

  await expect(page.locator('text="Set up your storefront"')).toBeVisible();
  await expect(page.locator('text="Accepting your first payment"')).toBeVisible();
});

test('Release Notes Versioning', async ({ page }) => {
  await page.click('button:has-text("Menu")');
  await page.click('button:has-text("What\'s New")');
  await expect(page.locator('text="What\'s New in OHC"')).toBeVisible();

  // Verify versioning scheme
  await expect(page.locator('text="v0.3.4 (Cloud) / v0.3.4+1 (Standalone)"')).toBeVisible();
});
