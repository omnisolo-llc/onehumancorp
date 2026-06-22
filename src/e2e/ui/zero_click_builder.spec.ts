import { test, expect } from '@playwright/test';
import * as crypto from 'crypto';
<<<<<<< HEAD
import * as fs from 'fs';
import * as path from 'path';
=======
>>>>>>> dac923c2 (Fix Vitest environment and onboarding adminName logic (#30375))

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

<<<<<<< HEAD
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
    await page.route('**/api/v1/growth/zero-click-builder/generate', async route => {
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
=======
    // Navigate to the real setup page
    await page.goto('/api/ui/setup.html');
>>>>>>> dac923c2 (Fix Vitest environment and onboarding adminName logic (#30375))

    // Verify Initial Screen
    await expect(page.getByRole('heading', { name: '10-Minute Setup Wizard' })).toBeVisible();

    // 1. Click "Instant Build"
    await page.getByRole('button', { name: 'Instant Build' }).click();

<<<<<<< HEAD
    // Wait and check if there's any visibility issues.
    await page.waitForTimeout(500);
    const content = await page.content();
    if (!content.includes('Tell us about your business')) {
        console.log("PAGE CONTENT DOES NOT HAVE HEADING:", content);
    }

=======
>>>>>>> dac923c2 (Fix Vitest environment and onboarding adminName logic (#30375))
    // 2. Verify we are in the instant step
    await expect(page.getByRole('heading', { name: 'Tell us about your business' })).toBeVisible();

    // 3. Fill in the description
    const instantInput = page.locator('#instant-bio');
    await expect(instantInput).toBeVisible();
    await instantInput.fill('I am a home baker in Austin selling custom vegan cakes and cupcakes.');

    const generateBtn = page.getByTestId('generate-storefront-btn');
    await expect(generateBtn).toBeEnabled();

    // 4. Click generate
    await generateBtn.click();

    // 5. Wait for generation to complete and the success message to appear
<<<<<<< HEAD
    await expect(page).toHaveURL(/.*success.html/, { timeout: 15000 });
=======
    await page.waitForURL('**/success.html', { timeout: 30000 });
>>>>>>> dac923c2 (Fix Vitest environment and onboarding adminName logic (#30375))
  });
});
