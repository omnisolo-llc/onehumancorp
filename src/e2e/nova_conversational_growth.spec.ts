import { test, expect } from './fixtures';

test('Conversational Growth Loop CUJ', async ({ page, loginAs, adminUser }) => {
  // 1. Setup: Use a dedicated test tenant to avoid interference
  const testTenant = `growth-test-${Math.floor(Math.random() * 1000000)}`;

  // Login as admin
  await loginAs(page, adminUser);

  // Navigate to Conversational Manager
  await page.goto('/conversational-manager.html');
  await page.waitForLoadState('networkidle');

  // 2. Ask about growth when performance is "clean"
  const input = page.locator('#message-input');
  await input.fill('How can I grow my business?');
  await page.click('#send-btn');

  // Verify generic positive response
  await expect(page.locator('.message.agent').last()).toContainText(/performing well|no abandoned carts/i);

  // 3. Verify Rating intent
  await input.fill('What is my current rating?');
  await page.click('#send-btn');
  await expect(page.locator('.message.agent').last()).toContainText(/average rating is/i);
  await expect(page.locator('.message.agent').last()).toContainText(/Powered by OHC/i);

  // 4. Verify Social Media Post Intent
  await input.fill('Generate a social post');
  await page.click('#send-btn');
  await expect(page.locator('.message.agent').last()).toContainText(/highlighting your recent success/i);
  await expect(page.locator('.action-card').last()).toContainText(/Post to X \(Twitter\)/i);
  await expect(page.locator('.action-card').last()).toContainText(/Powered by OHC/i);
});
