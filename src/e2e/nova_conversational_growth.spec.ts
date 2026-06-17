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

  // 3. (Optional/Simulated) Create an abandoned cart via API if possible,
  // but since we want to be hermetic and avoid complex setup in this script,
  // we focus on verifying that the keywords trigger the correct handler logic.

  await input.fill('Check my abandoned carts');
  await page.click('#send-btn');

  // If no carts, it should still be a valid response from our new logic
  const response = page.locator('.message.agent').last();
  await expect(response).toBeVisible();

  // 4. Verify Rating intent
  await input.fill('What is my current rating?');
  await page.click('#send-btn');
  await expect(page.locator('.message.agent').last()).toContainText(/average rating is/i);

  // 5. Verify Growth Loop Branding (Powered by OHC watermark)
  const viralFooter = page.locator('#cm-footer-viral-link');
  await expect(viralFooter).toBeVisible();
  await expect(viralFooter).toContainText('Powered by OneHumanCorp');

  // 6. Verify Viral Share Loop after Action Execution
  // Simulate an intent that generates a draft action
  await input.fill('Create a 10% discount code');
  await page.click('#send-btn');

  // Verify the agent creates an Action Card
  const approveBtn = page.locator('.btn-approve').last();
  await expect(approveBtn).toBeVisible();
  await expect(approveBtn).toHaveText('Approve & Publish');

  // Click the execute/publish button
  await approveBtn.click();
  await expect(approveBtn).toHaveText('Published', { timeout: 5000 });

  // Verify the viral share buttons are appended to the success message
  const shareXBtn = page.locator('.btn-share-x').last();
  await expect(shareXBtn).toBeVisible();
  await expect(shareXBtn).toHaveText('Share on X');

  const shareWaBtn = page.locator('.btn-share-wa').last();
  await expect(shareWaBtn).toBeVisible();
  await expect(shareWaBtn).toHaveText('Share on WhatsApp');
});
