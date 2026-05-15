import { test, expect } from '@playwright/test';

test('Comprehensive Day One Onboarding Flow', async ({ page }) => {
  await page.goto('/');

  // 1. Welcome Screen
  await expect(page.locator('h1')).toContainText('Your business, live in minutes.');
  await page.click('text=Launch My Business');

  // 2. Business Type
  await expect(page.locator('h2')).toContainText('What are you building?');
  await page.click('text=Online Store');
  await page.click('button:has-text("Continue")');

  // 3. Name & Description
  await expect(page.locator('h2')).toContainText('Name your vision');
  await page.fill('#biz-name', 'Maya Magic Cakes');
  // Wait for AI suggestion
  await page.waitForTimeout(2000);
  const desc = await page.inputValue('#biz-desc');
  expect(desc).toContain('Maya Magic Cakes');
  await page.click('text=Next Step');

  // 4. Sell Categories
  await expect(page.locator('h2')).toContainText("What's on the menu?");
  await page.click('text=Products');
  await page.click('text=Digital');
  await page.click('text=Almost there');

  // 5. Payments
  await expect(page.locator('h2')).toContainText('Get Paid');
  await page.click('text=Online Payments');
  await page.click('button:has-text("Continue")');

  // 6. Account
  await expect(page.locator('h2')).toContainText('Create your account');
  await page.fill('#admin-name', 'Maya Smith');
  await page.fill('#admin-email', 'maya@cakes.com');
  await page.fill('#admin-pass', 'password123');
  await page.click('text=Finish Setup');

  // 7. Launch
  await expect(page.locator('h2')).toContainText('Ready for liftoff?');
  await page.click('text=Launch My Business');

  // 8. Loading & Dashboard
  await expect(page.locator('h2')).toContainText('Setting up your empire...');
  await expect(page.locator('h1', { timeout: 10000 })).toContainText('Maya Smith');

  // 9. Website Builder
  await page.click('text=Edit Website');
  await expect(page.locator('h2')).toContainText('Website Builder');
  await page.click('text=Bold & Colorful');
  await page.click('text=Continue to Brand');
  await page.click('text=Continue to Products');
  await page.fill('#prod-name', 'Magic Cupcake');
  await page.waitForTimeout(1500);
  expect(await page.inputValue('#prod-desc')).toContain('Magic Cupcake');
  await page.click('text=Continue to Domain');
  await page.click('text=Go Live Now!');

  // Assert back to Dashboard
  await expect(page.locator('h1')).toContainText('Maya Smith');

  // 10. Agent Config
  await page.click('text=The Manager');
  await expect(page.locator('h2')).toContainText('The Manager');
  await page.click('text=Professional');
  await page.fill('#sandbox-input', 'Hello');
  await page.press('#sandbox-input', 'Enter');
  await expect(page.locator('#sandbox-history')).toContainText('Agent:');
  await page.click('text=Save & Deploy Changes');

  // Final check
  await expect(page.locator('h1')).toContainText('Maya Smith');
});
