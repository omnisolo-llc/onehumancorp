import { test, expect } from '@playwright/test';

test('Documentation and Help Features E2E', async ({ page }) => {
  // Wait for the app to be ready
  await page.goto('http://localhost:3000/onboarding');

  // Verify floating Help Agent button exists and opens chat
  const askAnythingBtn = page.locator('button:has-text("Ask anything")');
  await expect(askAnythingBtn).toBeAttached();
  await askAnythingBtn.click();

  await expect(page.getByRole('heading', { name: 'Help Agent' })).toBeVisible();
  const helpInput = page.getByPlaceholder('Ask me anything...');
  await helpInput.fill('How do I add a product?');
  await helpInput.press('Enter');

  await expect(page.getByText('How do I add a product?')).toBeVisible();

  // Wait for network mock / AI reply
  await expect(page.getByText('I specialize in answering questions about OHC features')).toBeVisible();

  // Close Chat
  await page.locator('.help-chat-wrapper button').filter({ has: page.locator('svg') }).first().click();

  // Navigate to Help Center page directly
  await page.goto('http://localhost:3000/help');
  await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();

  // Verify help topics are loaded
  await expect(page.getByText('Getting Started')).toBeVisible();
  await expect(page.getByText('My Store')).toBeVisible();

  // Verify API Docs
  await page.goto('http://localhost:3000/api-docs');
  await expect(page.getByText('Advanced: This section is for developers')).toBeVisible();
});
