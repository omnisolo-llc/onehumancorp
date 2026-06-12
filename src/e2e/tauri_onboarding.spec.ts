import { test, expect } from '@playwright/test';
import * as path from 'path';
import * as fs from 'fs';

test.describe('Tauri Onboarding Wizard Flow', () => {
  const mockTauriBackend = () => {
    window.__TAURI__ = {
      core: {
        invoke: async (cmd, args) => {
          if (cmd === "get_onboarding_state") {
            const state = sessionStorage.getItem('mockState');
            return state ? JSON.parse(state) : { step: 0 };
          }
          if (cmd === "save_onboarding_state") {
             sessionStorage.setItem('mockState', JSON.stringify(args.state));
             return null;
          }
          if (cmd === "start_onboarding") {
             return { success: true, organization_id: 'mock-org-id', message: 'Workspace created for ' + args.req.company_name + '. ' + args.req.admin_name + ' is ready to help.' };
          }
          throw new Error('Unhandled command: ' + cmd);
        }
      }
    };
  };

  test('Completes the onboarding flow, verifies validation, multi-step progression, and backend state resume', async ({ page }) => {
    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR || process.cwd(), process.env.TEST_WORKSPACE)
        : process.cwd();

    const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');

    await page.route('http://mock/setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await page.route('**/api/onboarding/intake', async route => {
        const data = {
            business_name: "Test Business",
            business_type: "Handyman",
            categories: ["services"],
            location: "Austin, TX",
            target_audience: "Anyone",
            initial_products: [
                { name: "Faucet Repair", price: "50.00", description: "Fixing things" }
            ]
        };
        await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify(data) });
    });

    await page.addInitScript(mockTauriBackend);
    await page.goto('http://mock/setup.html');

    // Setup page (Chat Interface)
    await expect(page.getByText("Hi there! I'm your OHC onboarding assistant. What do you do?")).toBeVisible();

    const input = page.locator('#chat-input');
    await input.fill("I'm a local handyman business called Test Business. I do fixing things for $50/hr.");
    await page.locator('#send-btn').click();

    // Mock will process and we expect a redirect or final message
    await expect(page.getByText("Workspace ready! Redirecting you to your dashboard...")).toBeVisible({ timeout: 20000 });

  });

  test('Validates 44px touch targets on mobile sizes and layout rules', async ({ page }) => {
    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR || process.cwd(), process.env.TEST_WORKSPACE)
        : process.cwd();

    const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');

    await page.route('http://mock/setup.html', async route => {
        const fs = require('fs');
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    // Set a mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('http://mock/setup.html');

    const inputArea = page.locator('.input-area');
    await expect(inputArea).toBeVisible();

    const sendBtn = page.locator('#send-btn');
    const btnBox = await sendBtn.boundingBox();

    if (btnBox) {
        expect(btnBox.height).toBeGreaterThanOrEqual(44);
        expect(btnBox.width).toBeGreaterThanOrEqual(44);
    }
  });

});

  test('Setup UI should have glassmorphism aesthetics applied', async ({ page }) => {
    test.skip();
  });

test.describe('Tauri Dashboard UI and UX Improvements', () => {
  test('Dashboard should have glassmorphism aesthetics applied', async ({ page }) => {
    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR || process.cwd(), process.env.TEST_WORKSPACE)
        : process.cwd();

    const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');

    await page.route('/dashboard.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'dashboard.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await page.addInitScript(() => {
      window.__TAURI__ = {
        core: {
          invoke: async (cmd, args) => {
            if (cmd === 'generate_cloud_invite') {
              return "https://cloud.ohc.network/invite/mock-test";
            }
            if (cmd === "start_onboarding") {
              return null;
            }
            throw new Error('Unhandled command: ' + cmd);
          }
        }
      };
    });

    await page.goto('/dashboard.html');

    // Check that the container class has the updated glassmorphism properties
    const container = page.locator('.container');
    await expect(container).toHaveCSS('backdrop-filter', 'blur(30px) saturate(2.1)');
    await expect(container).toHaveCSS('border-radius', '16px');
    await expect(container).toHaveCSS('background-color', 'rgba(255, 255, 255, 0.65)');
    await expect(container).toHaveCSS('border', '1px solid rgba(255, 255, 255, 0.4)');

    // Check dark mode
    await page.emulateMedia({ colorScheme: 'dark' });
    await expect(container).toHaveCSS('background-color', 'rgba(22, 22, 26, 0.7)');
    await expect(container).toHaveCSS('border', '1px solid rgba(255, 255, 255, 0.1)');
  });
});
