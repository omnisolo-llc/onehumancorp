import { test, expect } from './fixtures';
import * as path from 'path';
import * as fs from 'fs';

test.describe('Tauri Cross Device Onboarding CUJ', () => {
  test('Persona: Business Owner can save draft in Tauri and resume cross device', async ({ page, browser }) => {
    // 1. Owner starts onboarding directly from the current route.

    // Inject fixed IDs to ensure it matches
    await page.evaluate(() => {
      localStorage.setItem('tenant_id', 'storefront');
      localStorage.setItem('user_id', 'test-user');
    });

    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR, process.env.TEST_WORKSPACE)
        : process.cwd();

    const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');

    const serveLocal = async (pageToServe) => {
        await pageToServe.route('/index.html', async route => {
            const content = fs.readFileSync(path.join(tauriUiDir, 'index.html'), 'utf-8');
            await route.fulfill({ contentType: 'text/html', body: content });
        });

        await pageToServe.route('/setup.html', async route => {
            const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
            await route.fulfill({ contentType: 'text/html', body: content });
        });

        await pageToServe.route('/assistant.html', async route => {
            const content = fs.readFileSync(path.join(tauriUiDir, 'assistant.html'), 'utf-8');
            await route.fulfill({ contentType: 'text/html', body: content });
        });

        await pageToServe.route('/success.html', async route => {
            const content = fs.readFileSync(path.join(tauriUiDir, 'success.html'), 'utf-8');
            await route.fulfill({ contentType: 'text/html', body: content });
        });

        await pageToServe.route('/dashboard.html', async route => {
            const content = fs.readFileSync(path.join(tauriUiDir, 'dashboard.html'), 'utf-8');
            await route.fulfill({ contentType: 'text/html', body: content });
        });
    };

    await serveLocal(page);

    // We no longer mock the POST/GET APIs, they hit localhost:18789

    // Navigate to the index.html page.
    await page.goto('/index.html');
    await page.getByRole('button', { name: 'Start Onboarding' }).click();

    // Setup page
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();

    const nameInput = page.getByPlaceholder("e.g. Maya's Bakery");
    await nameInput.fill("Tauri Cross Device Bakery");

    await page.getByRole('button', { name: 'Next' }).click();

    // Verify it transitioned to assistant setup
    await expect(page.getByRole('heading', { name: "Set up your Assistant" })).toBeVisible();

    // Wait a brief moment for the backend save to theoretically complete
    await page.waitForTimeout(500);

    // 4. Simulate a cross-device session with a new browser context
    const newContext = await browser.newContext();
    const newPage = await newContext.newPage();
    await serveLocal(newPage);

    await newPage.goto('/dashboard'); // just to set origin
    await newPage.evaluate(() => {
      localStorage.setItem('tenant_id', 'storefront');
      localStorage.setItem('user_id', 'test-user');
    });

    await newPage.goto('/setup.html');

    // 5. Verify the business name was properly restored from backend response
    await expect(newPage.getByPlaceholder(/e.g. Maya's Bakery/i)).toHaveValue('Tauri Cross Device Bakery', { timeout: 10000 });

    await newContext.close();
  });
});
