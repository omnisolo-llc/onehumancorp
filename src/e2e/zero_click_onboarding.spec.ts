import { test, expect } from './fixtures';

test.describe('Zero-Click Onboarding Agent', () => {
  test('Persona: Maya provisions a tenant and drafts products from a single sentence', async ({ page }) => {
    // Navigate to the setup UI
    await page.goto('/ui/setup.html');

    // Verify mobile-first layout
    await page.setViewportSize({ width: 375, height: 812 });

    // Click the "Conversational Setup" button on the initial step
    const conversationalSetupBtn = page.getByRole('button', { name: /Conversational Setup/i });
    await expect(conversationalSetupBtn).toBeVisible({ timeout: 15000 });
    await conversationalSetupBtn.click();

    // Verify we are in the chat step
    await expect(page.locator('h1', { hasText: 'Setup Assistant' })).toBeVisible();

    // Enter business description in the chat input
    const chatInput = page.getByTestId('chat-input');
    await chatInput.fill('I sell custom vegan cakes in Austin');

    // Send the message
    await page.getByTestId('chat-send-btn').click();

    // The assistant should respond with a follow-up asking for more details
    await expect(page.locator('#chat-messages', { hasText: 'Could you provide an example photo or a little more detail' })).toBeVisible({ timeout: 10000 });

    // Respond to the follow up
    await chatInput.fill('My best seller is a chocolate raspberry cake');
    await page.getByTestId('chat-send-btn').click();

    // Wait for the approval step to appear
    await expect(page.locator('h1', { hasText: 'Ready to Launch' })).toBeVisible({ timeout: 15000 });

    // Verify the approval carousel and its content
    await expect(page.locator('#approval-details')).toBeVisible();
    await expect(page.locator('#approval-storefront-content', { hasText: 'Type: Online Store' })).toBeVisible();
    await expect(page.locator('#approval-products-content', { hasText: '6-inch Celebration Cake - 45.00' })).toBeVisible();
    await expect(page.locator('#approval-payments-content', { hasText: "We've prepared a Stripe Express Connected Account" })).toBeVisible();

    // Approve and publish
    const approveBtn = page.getByTestId('approve-publish-btn');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // Verify the loading/generation state appears with the correct text
    await expect(page.locator('h1', { hasText: 'Building Your Business...' })).toBeVisible();
    await expect(page.getByText('Building your catalog...')).toBeVisible();

    // Verify successful creation
    await expect(page.locator('h2', { hasText: "You're Live!" })).toBeVisible({ timeout: 20000 });
  });
});
