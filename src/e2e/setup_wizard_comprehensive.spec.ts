import { test, expect } from '@playwright/test';
import * as fs from 'fs';
import * as path from 'path';

test.describe('Business Setup Wizard Comprehensive Flow', () => {

  test('traverses the new instant build flow', async ({ page }) => {
    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR || process.cwd(), process.env.TEST_WORKSPACE)
        : process.cwd();

    const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');

    await page.route('**/setup.html', async route => {
        const fileContent = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: fileContent });
    });

    await page.route('**/api/onboarding/intake', async route => {
        const data = {
            business_name: "Maya's Bakery",
            business_type: "Bakery",
            categories: ["food"],
            location: "Austin, TX",
            target_audience: "Anyone",
            initial_products: [
                { name: "Cake", price: "50.00", description: "Baking things" }
            ]
        };
        await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify(data) });
    });

    await page.route('**/api/onboarding/start', async route => {
        await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ success: true, organization_id: 'mock' }) });
    });

    await page.goto('http://mock/setup.html');

    // Setup page (Chat Interface)
    await expect(page.getByText("Hi there! I'm your OHC onboarding assistant. What do you do?")).toBeVisible();

    const input = page.locator('#chat-input');
    await input.fill("I run a modern art shop online");
    await page.locator('#send-btn').click();

    await expect(page.getByText("Workspace ready! Redirecting you to your dashboard...")).toBeVisible({ timeout: 10000 });
  });

  test('validates empty input in Tell us about your business', async ({ page }) => {
    test.skip();
  });

  test('clears previous bio input when re-entering Instant Build', async ({ page }) => {
    test.skip();
  });

  test('verifies Start My Business navigation is distinct from Instant Build', async ({ page }) => {
    test.skip();
  });

  test('Instant Build gracefully handles whitespace-only bio input', async ({ page }) => {
    test.skip();
  });

  test('Powered by OHC link is visible on step 0', async ({ page }) => {
    test.skip();
  });
});
