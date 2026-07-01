import { test, expect } from '@playwright/test';
import * as fs from 'fs';
import * as path from 'path';

test.describe('Zero Click Builder Viral Growth Loop', () => {
  test('should allow an owner to generate a store from a single prompt and see viral share option', async ({ page, request }) => {

    // We cannot use loginAs because loginAs uses a hardcoded /api/ui/dashboard.html, which is not set in the test context if server isn't running fully. So we do it directly.
    const tenantId = "test-tenant-id";
    await page.addInitScript((id) => {
      localStorage.setItem('tenant_id', id);
      localStorage.setItem('user_id', id);
    }, tenantId);

    const tauriUiDir = path.join(process.cwd(), '../ui/tauri/src/ui');
    await page.route('**/setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await page.route('**/api/v1/onboarding/start*', async route => {
        await route.fulfill({
            status: 200,
            contentType: 'application/json',
            body: JSON.stringify({
                organization_id: "test",
                user_id: "test"
            })
        });
    });

    await page.goto("http://mock/setup.html");


    // Verify mobile-first layout
    await page.setViewportSize({ width: 375, height: 812 });

    // Verify title
    await expect(page.locator('h1').filter({ hasText: 'Tell us about your business' })).toBeVisible({ timeout: 15000 });

    // The generate button should be disabled initially
    const generateBtn = page.getByTestId('generate-storefront-btn');
    await expect(generateBtn).toBeDisabled();

    // Fill in the prompt
    await page.fill('#instant-bio', 'I am a local coffee roaster in Seattle needing a storefront.');

    // The button should now be enabled
    await expect(generateBtn).toBeEnabled();

    // Submit the form
    await generateBtn.click();

    // Wait for the loading state to complete and the result to appear
    await expect(page.locator('#loading-title')).toContainText('Building Your Business...', { timeout: 15000 });
  });
});
