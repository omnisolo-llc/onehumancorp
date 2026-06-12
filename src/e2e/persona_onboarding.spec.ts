import { test, expect } from '@playwright/test';
import * as fs from 'fs';
import * as path from 'path';

test.describe('Persona-Driven Onboarding E2E', () => {

  test('Maya the Baker persona journey', async ({ page }) => {
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

    const input = page.locator('#chat-input');
    await input.fill("I am Maya the baker");
    await page.locator('#send-btn').click();

    await expect(page.getByText("Workspace ready! Redirecting you to your dashboard...")).toBeVisible({ timeout: 10000 });
  });

  test('Carlos the Handyman persona journey', async ({ page }) => {
    test.skip();
  });

  test('Priya the Boutique Owner persona journey', async ({ page }) => {
    test.skip();
  });

  test('Leo the Tutor persona journey', async ({ page }) => {
    test.skip();
  });

  test('Manual setup flow without persona', async ({ page }) => {
    test.skip();
  });
});
