import { test, expect } from '@playwright/test';

test('Documentation CUJ: Navigate to Help Center, Walkthrough, AI Chat, and Changelog', async ({ page }) => {
  await page.goto('/');
  await page.waitForLoadState('networkidle');
  const emailInput = page.locator('input[type="email"], input[name="username"]').first();
  await emailInput.waitFor({ state: 'visible', timeout: 30000 });
  await emailInput.fill('admin');

  const passInput = page.locator('input[type="password"], input[name="password"]').first();
  await passInput.fill('admin');

  await page.click('button:has-text("Sign In"), button:has-text("Login")');
  await page.waitForLoadState('networkidle');

  // Click Help Center
  await page.click('text="Help Center"');
  await page.waitForLoadState('networkidle');
  // Assert
  await expect(page.locator('text="How can we help you today?"')).toBeVisible();

  // Click What's New
  await page.click('text="What\\\'s New"');
  await page.waitForLoadState('networkidle');
  // Assert
  await expect(page.locator('text="Release Notes"')).toBeVisible();

  // Click Ask AI FAB
  await page.click('text="Ask AI"');
  await page.waitForLoadState('networkidle');
  // Assert
  await expect(page.locator('text="Hi! I am your OHC Support Agent"')).toBeVisible();

  // Type in Chat
  await page.fill('input[type="text"]', 'How do I add a product?');
  await page.click('button:has-text("send"), button i:text("send")'); // or hit Enter
  await expect(page.locator('text="How do I add a product?"')).toBeVisible();
});
