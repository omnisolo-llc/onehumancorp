import { test, expect } from '@playwright/test';
import * as crypto from 'crypto';
import * as fs from 'fs';
import * as path from 'path';

test.describe('Zero-Click Business Generator CUJ', () => {

  test.beforeEach(async ({ page }) => {
    // Navigate using the real server routing instead of mocks
    const tenantId = `tenant-${crypto.randomBytes(4).toString('hex')}`;
    await page.addInitScript((id) => {
      localStorage.setItem('tenant_id', id);
      localStorage.setItem('user_id', id);
    }, tenantId);

    await page.setViewportSize({ width: 375, height: 812 });
  });

  test('User can generate a business with a single prompt', async ({ page }) => {

    const workspaceRoot = process.env.TEST_WORKSPACE ? path.join(process.env.TEST_SRCDIR || path.resolve(__dirname, '..', '..', '..'), process.env.TEST_WORKSPACE) : path.resolve(__dirname, '..', '..', '..');

    await page.route('**/setup.html', async route => {
        const fileContent = fs.readFileSync(path.join(workspaceRoot, 'src/ui/tauri/src/ui/setup.html'), 'utf-8');
        await route.fulfill({
            status: 200,
            contentType: 'text/html',
            body: fileContent
        });
    });

    // Mock the api response
    await page.route('**/api/v1/growth/zero-click-builder/generate*', async route => {
        await route.fulfill({
            status: 200,
            contentType: 'application/json',
            body: JSON.stringify({
                organization_id: "test-org",
                user_id: "test-user",
                message: "Storefront generated successfully"
            })
        });
    });

    await page.route('**/success.html', async route => {
        await route.fulfill({
            status: 200,
            contentType: 'text/html',
            body: "<html><body>Success</body></html>"
        });
    });


    // Navigate to the real setup page
    await page.goto('http://mock/setup.html');

    // Wait and check if there's any visibility issues.
    await page.waitForTimeout(500);

    // 1. Verify we are in the chat step
    await expect(page.locator("h1").filter({ hasText: "Setup Assistant" })).toBeAttached();

    // 2. Fill in the description in chat input
    const chatInput = page.getByTestId('chat-input');
    await expect(chatInput).toBeAttached();
    await chatInput.fill('I am a home baker in Austin selling custom vegan cakes and cupcakes.');

    // 3. Mock the chat API response
    await page.route('**/api/onboarding/chat*', async route => {
        await route.fulfill({
            status: 200,
            contentType: 'application/json',
            body: JSON.stringify({
                is_complete: true,
                reply: "Give me a minute... I'm building your business.",
                intake_data: {
                    business_name: "Mock Bakery",
                    business_type: "Bakery",
                    categories: ["food"],
                    initial_products: [{ name: "Vegan Cake", price: "25.00" }]
                }
            })
        });
    });

    // Mock the start API response
    await page.route('**/api/onboarding/start*', async route => {
        await route.fulfill({
            status: 200,
            contentType: 'application/json',
            body: JSON.stringify({
                organization_id: "test-org",
                user_id: "test-user",
                status: "success"
            })
        });
    });

    const sendBtn = page.getByTestId('chat-send-btn');
    await expect(sendBtn).toBeEnabled();

    // 4. Click send
    await sendBtn.click();

    // 5. Wait for generation to complete and the approve button to appear
    const approveBtn = page.getByTestId('approve-publish-btn');
    await expect(approveBtn).toBeAttached({ timeout: 5000 });

    // Click approve
    await approveBtn.click();

    // 6. Wait for success page
    await expect(page).toHaveURL(/.*success.html/, { timeout: 15000 });
  });

});
