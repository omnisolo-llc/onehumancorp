import { test, expect } from '@playwright/test';
import * as fs from 'fs';
import * as path from 'path';

test.describe('Instant Setup CUJ', () => {

  test.beforeEach(async ({ page }) => {
    // Clean up local storage to ensure fresh start
    await page.addInitScript(() => window.localStorage.clear());
    // Set a known viewport for mobile tests (375px first as per requirements)
    await page.setViewportSize({ width: 375, height: 812 });

    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR || process.cwd(), process.env.TEST_WORKSPACE)
        : process.cwd();
    const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');
    await page.route('**/*.html*', async route => {
        try {
            const url = new URL(route.request().url());
            const filename = url.pathname.split('/').pop();
            const filePath = path.join(tauriUiDir, filename);
            if (fs.existsSync(filePath)) {
                const fileContent = fs.readFileSync(filePath, 'utf-8');
                await route.fulfill({ contentType: 'text/html', body: fileContent });
            } else {
                await route.fulfill({ status: 200, contentType: 'text/html', body: '<html><body>Mocked ' + filename + '</body></html>' });
            }
        } catch (e) {
            await route.continue();
        }
    });

    // mock the tauri backend
    await page.addInitScript(() => {
        (window as any).__TAURI__ = {
            core: {
                invoke: async (cmd, args) => {
                    if (cmd === 'start_onboarding') {
                        return { success: true, organization_id: "mock_org_id" };
                    }
                    if (cmd === 'process_intake') {
                        await new Promise(r => setTimeout(r, 4000));
                        return {
                            business_name: "Maya Bakery",
                            business_type: "Local Service",
                            categories: ["Bakery"],
                            location: "Austin",
                            target_audience: "Anyone",
                            initial_products: [ { name: "Vegan Cake", price: "45.00" } ]
                        };
                    }
                    return null;
                }
            }
        };
    });
  });

  test('Persona: Maya (Home Baker) completes the Zero-Click Instant Onboarding', async ({ page }) => {



    await page.goto('/setup.html');


    // Verify Initial Screen / Instant Build Step
    await expect(page.getByRole('heading', { name: 'Tell us about your business' })).toBeVisible();

    // 1. Fill in the description
    const instantInput = page.locator('#instant-bio');
    await expect(instantInput).toBeVisible();
    await instantInput.fill('I make custom vegan cakes in Austin. I need a website and a way to take bookings.');

    const generateBtn = page.getByTestId('generate-storefront-btn');
    await expect(generateBtn).toBeEnabled();

    // 2. Click generate
    await generateBtn.click();

    // 3. Verify loading texts (animation progress)
    const btnText = await generateBtn.innerText();
    expect(btnText).toContain('Analyzing request...');

    // Check if the text changes to the next one
    await expect(generateBtn).toContainText('Designing storefront...', { timeout: 8000 });

    const approveBtn = page.locator('#approve-publish-btn');
    await expect(approveBtn).toBeVisible({ timeout: 10000 });
    await approveBtn.click();

    await expect(page).toHaveURL(/.*success.html/, { timeout: 60000 });
  });
});
