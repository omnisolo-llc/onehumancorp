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

    await page.route('**/onboarding/zero-click', async route => {
        const fileContent = fs.readFileSync(path.join(workspaceRoot, 'src/ui/tauri/src/ui/setup.html'), 'utf-8');
        await route.fulfill({
            status: 200,
            contentType: 'text/html',
            body: fileContent
        });
    });

    // Mock the api response
    await page.route('**/api/onboarding/start_zero_click*', async route => {
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

    await page.route('**/success.html*', async route => {
        await route.fulfill({
            status: 200,
            contentType: 'text/html',
            body: "<html><body>Success</body></html>"
        });
    });

    // Navigate to the real setup page
    await page.goto('http://mock/onboarding/zero-click');

    // Verify Initial Screen
    await expect(page.locator("h1").filter({ hasText: "Tell us about your business" })).toBeAttached();

    // 3. Fill in the description
    const instantInput = page.locator('#instant-bio');
    await expect(instantInput).toBeAttached();
    await instantInput.evaluate((el: HTMLTextAreaElement) => { el.value = 'I am a home baker in Austin selling custom vegan cakes and cupcakes.'; el.dispatchEvent(new Event('input')); });

    const generateBtn = page.getByTestId('generate-storefront-btn');
    await expect(generateBtn).toBeEnabled();

    // 4. Click generate
    await generateBtn.click();

    // 5. Wait for the approval screen to appear
    await expect(page.locator('h1', { hasText: 'Ready to Launch' })).toBeVisible({ timeout: 15000 });

    // Verify preview and deposit policy text
    await expect(page.locator('#approval-details')).toBeVisible();
    await expect(page.locator('#approval-details')).toContainText('Suggested Deposit Policy');

    // Click Approve & Publish
    const approveBtn = page.locator('#approve-publish-btn');
    await approveBtn.click();

    // 6. Wait for generation to complete and the success message to appear
    await expect(page).toHaveURL(/.*success.html/, { timeout: 15000 });
  });
});
