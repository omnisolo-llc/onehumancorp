import { test, expect } from '@playwright/test';
import * as path from 'path';
import * as fs from 'fs';

test.describe('Conversational Setup CUJ', () => {

  test.beforeEach(async ({ page }) => {
    // Intercept standard setup.html load to serve from filesystem for tests
    const tauriUiDir = path.join(process.cwd(), 'src/ui/tauri/src/ui');
    await page.route('**/setup.html', async route => {
      const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
      await route.fulfill({ contentType: 'text/html', body: content });
    });

    // Mock tooltips call
    await page.route('**/api/tooltips', async route => {
      await route.fulfill({ status: 200, body: JSON.stringify({}) });
    });

    // Mock the state endpoint which the frontend hits
    await page.route('**/api/onboarding/state', async route => {
       await route.fulfill({ status: 200, body: JSON.stringify({}) });
    });

    // Set a known viewport for mobile tests
    await page.setViewportSize({ width: 375, height: 812 });
  });

  test('Persona: Maya (Home Baker) completes the Zero-Click Conversational Onboarding', async ({ page }) => {

    // Mock the backend chat responses to simulate conversational flow
    let chatMessageCount = 0;
    await page.route('**/api/onboarding/chat', async route => {
      chatMessageCount++;
      if (chatMessageCount === 1) {
        // First user message
        await route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({
            is_complete: false,
            reply: "Great! Could you provide an example photo or a little more detail about what you sell?",
            intake_data: null
          })
        });
      } else {
        // Second user message -> finish the flow
        await route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({
            is_complete: true,
            reply: "Give me a minute... I'm building your business.",
            intake_data: {
              business_name: "Maya's Vegan Cakes",
              business_type: "Bakery",
              categories: ["food", "cakes"],
              initial_products: [
                { name: "Custom Vegan Cake", price: "45.00" }
              ],
              location: "Austin, TX",
              target_audience: "Vegans and cake lovers"
            }
          })
        });
      }
    });

    // Mock the final start endpoint that actually provisions the tenant
    let onboardingStarted = false;
    await page.route('**/api/onboarding/start', async route => {
      onboardingStarted = true;
      const postData = JSON.parse(route.request().postData() || '{}');

      // Verify payload was correctly formed from the intake_data
      expect(postData.company_name).toBe("Maya's Vegan Cakes");
      expect(postData.first_product_name).toBe("Custom Vegan Cake");

      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          success: true,
          organization_id: 'tenant-maya-123'
        })
      });
    });

    // Mock the success page redirection target
    await page.route('**/success.html', async route => {
      await route.fulfill({ status: 200, contentType: 'text/html', body: '<h1>Success</h1>' });
    });

    await page.goto('http://mock/setup.html');

    // Verify Initial Screen
    await expect(page.getByRole('heading', { name: '10-Minute Setup Wizard' })).toBeVisible();

    // 1. Click "Conversational Setup"
    await page.getByRole('button', { name: 'Conversational Setup' }).click();

    // 2. Verify we are in the chat step
    await expect(page.getByRole('heading', { name: 'Setup Assistant' })).toBeVisible();

    // Ensure the chat container has proper mobile-first glassmorphism styling
    const chatInput = page.locator('#chat-input');
    await expect(chatInput).toBeVisible();
    await expect(chatInput).toHaveClass(/glassmorphism/);

    // Verify Touch Targets on the chat send button
    const chatSendBtn = page.locator('#chat-send-btn');
    const sendBtnBox = await chatSendBtn.boundingBox();
    expect(sendBtnBox?.height).toBeGreaterThanOrEqual(44);

    // 3. Send first message
    await chatInput.fill('I make custom vegan cakes in Austin.');
    await chatSendBtn.click();

    // 4. Verify bot responds asking for more details
    await expect(page.getByText('Great! Could you provide an example photo or a little more detail about what you sell?')).toBeVisible();

    // 5. Send second message (simulating uploading a photo or additional details)
    await chatInput.fill('Here is a picture of my cakes.');
    await chatSendBtn.click();

    // 6. Verify bot finishes the conversation
    await expect(page.getByText("Give me a minute... I'm building your business.")).toBeVisible();

    // 7. Verify the start onboarding API was called and it navigated to success.html
    await page.waitForURL('**/success.html', { timeout: 5000 });
    expect(onboardingStarted).toBe(true);
  });
});
