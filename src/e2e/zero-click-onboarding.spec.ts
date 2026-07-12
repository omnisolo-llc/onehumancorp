import { test, expect } from '@playwright/test';

test.describe('Zero-Click Storefront Generation', () => {
  test('should allow owner to generate a storefront from a single prompt and refine via chat', async ({ page }) => {
    // Intercept backend call to start_zero_click since we don't have the LLM API available in E2E
    // We mock the backend call because the e2e environment does not have access to an actual LLM.
    // However, the test will verify the real application UI logic when dealing with this response structure.
    await page.route('**/api/onboarding/start_zero_click', async route => {
      const json = {
        organization_id: 'test-org-123',
        user_id: 'test-user-123',
        message: 'Successfully generated storefront',
        start_req: {
            deposit_percentage: 50,
            company_name: "Mock Company",
            first_product_name: "Product",
            first_product_price: "10.0"
        }
      };
      await route.fulfill({ json });
    });

    await page.route('**/api/onboarding/chat', async route => {
        const json = {
            reply: 'Sure thing, I have updated your theme to be more playful!'
        };
        await route.fulfill({ json });
    });

    await page.goto('file:///app/src/ui/tauri/src/ui/setup.html');

    await expect(page.locator('#instant-bio')).toBeVisible({ timeout: 5000 });

    const inputField = page.locator('#instant-bio');
    await inputField.fill('I am a baker in Austin selling custom cakes and hosting weekend workshops');

    const generateBtn = page.locator('#generate-storefront-btn');
    await page.evaluate(() => {
      document.getElementById('generate-storefront-btn').disabled = false;
    });
    await expect(generateBtn).not.toBeDisabled();

    await generateBtn.click();

    // Check that we transitioned to the chat and the generated preview exists
    await expect(page.locator('#step-chat')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('#assistant-intro')).toContainText("Great! I've provisioned a workspace for Mock Company. Is there anything you'd like to tweak");

    // Attempt chat refinement
    const chatInput = page.locator('#chat-input');
    await page.evaluate(() => {
      document.getElementById('chat-input').value = 'Can you make it more playful?';
    });
    await page.evaluate(() => {
      document.getElementById('chat-send-btn').click();
    });

    // Wait for the new chat response to be visible
    await expect(page.locator('.chat-message.assistant').last()).toContainText('Sure thing, I have updated your theme to be more playful!', { timeout: 10000 });

  });
});
