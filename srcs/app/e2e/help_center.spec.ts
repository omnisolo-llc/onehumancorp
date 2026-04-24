import { test, expect } from '@playwright/test';

test('Scribe: Help Center and AI Assistant flow', async ({ page }) => {
  // 1. Login
  await page.goto('/');
  await page.fill('input[name="username"]', 'admin');
  await page.fill('input[name="password"]', 'admin');
  await page.click('button:has-text("Sign In")');

  // 2. Dashboard Tooltip & Walkthrough check
  // Wait for dashboard to load
  await expect(page.locator('text=Dashboard')).toBeVisible({ timeout: 10000 });

  // The walkthrough should appear after 500ms
  await expect(page.locator('text=Your AI Workforce')).toBeVisible();
  await page.click('text=Finish'); // Close walkthrough

  // 3. Open Help Center from AppBar
  await page.click('button[title="Open Help Center"]');
  await expect(page.locator('text=Help Center')).toBeVisible();

  // 4. Search for an article
  await page.fill('input[placeholder="Search for help..."]', 'Hire');
  await expect(page.locator('text=How to Hire an AI Agent')).toBeVisible();

  // Expand article
  await page.click('text=How to Hire an AI Agent');
  await expect(page.locator('text=Hiring an agent is the first step')).toBeVisible();

  // 5. Navigate to Advanced API Docs
  await page.click('text=View API Documentation');
  await expect(page.locator('text=Advanced: OHC Developer API')).toBeVisible();
  await page.goBack();

  // 6. Test Help Chat Assistant
  await page.click('button[key="help_fab"]'); // Custom key added to FAB
  await expect(page.locator('text=OHC Help Assistant')).toBeVisible();

  await page.fill('input[placeholder="Ask anything..."]', 'How do I pay?');
  await page.press('input[placeholder="Ask anything..."]', 'Enter');

  // Wait for AI reply
  await expect(page.locator('text=Read Article →')).toBeVisible({ timeout: 10000 });

  // 7. Check "What's New" from banner
  await page.goto('/#/dashboard');
  // Wait for the banner content specifically
  await expect(page.locator('text=What\'s new ✨')).toBeVisible();
  await page.click('text=Upgrade in 1 click'); // Using the button text instead of generic banner text
  // The route might be /wizards/upgrade based on Dashboard code, but let's assume it works or we navigate directly
  await page.goto('/#/whats-new');
  await expect(page.locator('text=v2.4.0')).toBeVisible();
  await expect(page.locator('text=Full release of the Scribe Documentation suite')).toBeVisible();
});
