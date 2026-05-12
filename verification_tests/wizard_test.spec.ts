import { test, expect } from '@playwright/test';

test('business setup wizard flow', async ({ page }) => {
  await page.goto('http://localhost:8081/business-setup');

  // Step 0: Welcome
  await expect(page.locator('h2')).toContainText('Your business, live in minutes');
  await page.click('text=Get Started');

  // Step 1: Business Type
  await page.click('text=Online Store');
  await page.click('text=Continue');

  // Step 2: Identity
  await page.fill('#biz-name', 'Acme Rocketry');
  // AI suggest check (mocked or real)
  await page.click('text=AI Suggest');
  await page.waitForTimeout(1000);
  await page.click('text=Continue');

  // Step 3: Offerings
  await page.click('text=Physical products');
  await page.click('text=Continue');

  // Step 4: Payments
  await page.click('text=Online only');
  await page.click('text=Continue');

  // Step 5: Account
  await page.fill('input[placeholder="Jane Doe"]', 'John Rocket');
  await page.fill('input[placeholder="jane@example.com"]', 'john@acme.com');
  await page.fill('input[placeholder="••••••••"]', 'secret123');
  await page.click('text=Review & Launch');

  // Step 6: Review
  await expect(page.locator('.glass-card')).toContainText('Acme Rocketry');
  await expect(page.locator('.glass-card')).toContainText('john@acme.com');
});
