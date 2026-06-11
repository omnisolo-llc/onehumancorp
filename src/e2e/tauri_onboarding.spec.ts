import { test, expect } from '@playwright/test';
import * as path from 'path';
import * as fs from 'fs';

test.describe('Tauri Onboarding Wizard Flow', () => {
  test('Completes the onboarding flow, verifies validation, multi-step progression, and backend state resume', async ({ page, browser }) => {
    // We mock the Tauri backend API to allow state save/resume
    const mockTauriBackend = () => {
      window.__TAURI__ = {
        core: {
          invoke: async (cmd, args) => {
            if (cmd === 'get_onboarding_state') {
              const stateStr = sessionStorage.getItem('mockState');
              return stateStr ? JSON.parse(stateStr) : {};
            } else if (cmd === 'save_onboarding_state') {
              const stateStr = sessionStorage.getItem('mockState');
              const currentState = stateStr ? JSON.parse(stateStr) : {};
              sessionStorage.setItem('mockState', JSON.stringify({ ...currentState, ...args.state }));
              return null;
            } else if (cmd === 'start_onboarding') {
              return null;
            }
            if (cmd === "start_onboarding") {
              return null;
            }
            throw new Error(`Unhandled command: ${cmd}`);
          }
        }
      };
    };
    await page.addInitScript(mockTauriBackend);

    // Navigate to the server-hosted onboarding
    await page.goto('/api/ui/onboarding/index.html');

    await expect(page.getByRole('heading', { name: "Welcome to OHC" })).toBeVisible();
    await page.getByRole('button', { name: 'Manual Setup' }).click();

    // Setup page (Step 1: Context)
    await expect(page.getByRole('heading', { name: "How do you work?" })).toBeVisible();

    // Verify validation triggers
    await page.locator('#step-context').getByRole('button', { name: 'Next' }).click();
    await expect(page.locator('#context-error')).toBeVisible();

    // Valid context
    await page.getByText('Local Service').click();
    await page.locator('#step-context').getByRole('button', { name: 'Next' }).click();

    // Step 2: Categories
    await expect(page.getByRole('heading', { name: "What's your category?" })).toBeVisible();
    await page.locator('#step-categories').getByRole('button', { name: 'Next' }).click();
    await expect(page.locator('#categories-error')).toBeVisible();
    await page.waitForTimeout(500);

    await page.locator('#business-categories').selectOption({ label: 'Handyman' });
    await page.locator('#step-categories').getByRole('button', { name: 'Next' }).click();

    // Step 3: Name
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();

    // Less than 3 chars validation
    await page.getByPlaceholder("e.g. Maya's Bakery").fill("Te");
    await page.locator('#step-name').getByRole('button', { name: 'Next' }).click();
    await expect(page.locator('#name-error')).toBeVisible();

    // Valid business name
    await page.getByPlaceholder("e.g. Maya's Bakery").fill("Test Business");
    await page.getByPlaceholder("Tagline (optional)").fill("Fixing things");
    await expect(page.locator('#name-error')).toBeHidden();

    await page.locator('#step-name').getByRole('button', { name: 'Next' }).click();

    // Step 4: Assistant
    await expect(page.getByRole('heading', { name: "Set up your Assistant" })).toBeVisible();

    // Verify validation triggers
    await page.locator('#step-assistant').getByRole('button', { name: 'Next' }).click();
    await expect(page.locator('#assistant-name-error')).toBeVisible();
    await expect(page.locator('#tone-error')).toBeVisible();
    await expect(page.locator('#assistant-name')).toHaveCSS('border-color', 'rgb(255, 59, 48)');

    await page.getByPlaceholder("e.g. Jarvis").fill("Jarvis");
    await page.locator('#assistant-tone').selectOption('Professional');
    await page.locator('#step-assistant').getByRole('button', { name: 'Next' }).click();


    // Step 5: Admin Setup
    await expect(page.getByRole('heading', { name: "Admin Credentials" })).toBeVisible();
    await page.getByPlaceholder("admin@mybusiness.com").fill("test@mybusiness.com");
    await page.getByPlaceholder("Password (min 8 chars)").fill("mypassword");
    await page.locator('#step-admin').getByRole('button', { name: 'Next' }).click();

    // Step 6: Offer
    await expect(page.getByRole('heading', { name: "Your First Offer" })).toBeVisible();

    await page.getByPlaceholder("e.g. Custom Birthday Cake").fill("Faucet Repair");
    await page.locator('#step-offer').getByRole('button', { name: 'Next' }).click();

    // Step 6: Template
    await expect(page.getByRole('heading', { name: "Template Selection" })).toBeVisible();

    // Verify validation triggers
    await page.getByRole('button', { name: 'Finish Setup' }).click();
    await expect(page.locator('#template-error')).toBeVisible();

    await page.locator('#template-selection').selectOption('Modern');



    // 2. Simulate Cross-Device Resume (Closing Page, Reopening, Checking State via Backend invoke mock)
    const savedStateStr = await page.evaluate(() => {
        try { return sessionStorage.getItem('mockState'); } catch(e) { return null; }
    });

    const newContext = await browser.newContext();
    const newPage = await newContext.newPage();

    await newPage.addInitScript(mockTauriBackend);
    await newPage.goto('/api/ui/onboarding/index.html');

    await newPage.evaluate((stateStr) => {
        if (stateStr) {
            try { sessionStorage.setItem('mockState', stateStr); } catch(e) {}
        }
    }, savedStateStr);

    // Owner comes back to the app on another device
    await newPage.goto('/api/ui/onboarding/setup.html');

    // It should load the values, we can skip through
    await newPage.waitForTimeout(500);
    await expect(newPage.locator('input[value="Local Service"]')).toBeChecked();
    await newPage.evaluate(() => { document.querySelector('#step-context .next-step-btn').click(); });

    await expect(newPage.locator('#business-categories')).toHaveValue('Handyman');
    await newPage.locator('#step-categories').getByRole('button', { name: 'Next' }).click();

    await expect(newPage.getByPlaceholder("e.g. Maya's Bakery")).toHaveValue("Test Business");
    await expect(newPage.getByPlaceholder("Tagline (optional)")).toHaveValue("Fixing things");
    await newPage.locator('#step-name').getByRole('button', { name: 'Next' }).click();

    await expect(newPage.getByPlaceholder("e.g. Jarvis")).toHaveValue("Jarvis");
    await expect(newPage.locator('#assistant-tone')).toHaveValue('Professional');
    await newPage.locator('#step-assistant').getByRole('button', { name: 'Next' }).click();


    // Step 5: Admin Setup
    await expect(newPage.getByRole('heading', { name: "Admin Credentials" })).toBeVisible();
    await expect(newPage.getByPlaceholder("admin@mybusiness.com")).toHaveValue("test@mybusiness.com");
    await expect(newPage.getByPlaceholder("Password (min 8 chars)")).toHaveValue("mypassword");
    await newPage.locator('#step-admin').getByRole('button', { name: 'Next' }).click();

    // Step 6: Offer
    await expect(newPage.getByRole('heading', { name: "Your First Offer" })).toBeVisible();
    await expect(newPage.getByPlaceholder("e.g. Custom Birthday Cake")).toHaveValue("Faucet Repair");
    await newPage.locator('#step-offer').getByRole('button', { name: 'Next' }).click();

    // Step 6: Template
    await expect(newPage.getByRole('heading', { name: "Template Selection" })).toBeVisible();


    // Verify validation triggers
    await newPage.getByRole('button', { name: 'Finish Setup' }).click();
    await expect(newPage.locator('#template-error')).toBeVisible();

    await newPage.locator('#template-selection').selectOption('Modern');

    // Submit
    await newPage.getByRole('button', { name: 'Finish Setup' }).click();

    // Success page
    await expect(newPage.getByRole('heading', { name: "You're all set!" })).toBeVisible();
    await expect(newPage.getByText('Workspace created for Test Business. Jarvis is ready to help.')).toBeVisible();

    await newContext.close();

  });

  test('Completes the Instant Build (AI) onboarding flow', async ({ page }) => {
    // We mock only the AI response for determinism in E2E,
    // but we let it hit the real server-hosted UI and state APIs
    await page.route(url => url.pathname.includes('/api/onboarding/intake'), async route => {
        await route.fulfill({
            contentType: 'application/json',
            body: JSON.stringify({
                business_name: "AI Generated Bakery",
                business_type: "Home Bakery",
                categories: ["food", "physical"],
                initial_products: [{ name: "AI Cupcake", price: "5.00" }]
            })
        });
    });

    await page.addInitScript(() => {
      window.__TAURI__ = {
        core: {
          invoke: async (cmd, args) => {
            if (cmd === 'get_onboarding_state') return {};
            if (cmd === 'save_onboarding_state') return null;
            if (cmd === 'start_onboarding') return null;
            throw new Error(`Unhandled command: ${cmd}`);
          }
        }
      };
    });

    await page.goto('/api/ui/onboarding/index.html');
    await page.getByRole('button', { name: 'Instant Build (AI)' }).click();

    await expect(page.getByRole('heading', { name: "Describe your business" })).toBeVisible();
    await page.locator('#ai-description').fill("I run a bakery that makes cupcakes.");
    await page.getByRole('button', { name: 'Generate My Business' }).click();

    // Should jump to Step 3: Name with AI data pre-filled
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await expect(page.locator('#business-name')).toHaveValue("AI Generated Bakery");

    // Continue flow
    await page.locator('#step-name').getByRole('button', { name: 'Next' }).click();
    await page.getByPlaceholder("e.g. Jarvis").fill("AI Assistant");
    await page.locator('#assistant-tone').selectOption('Friendly');
    await page.locator('#step-assistant').getByRole('button', { name: 'Next' }).click();

    await page.getByPlaceholder("admin@mybusiness.com").fill("ai@bakery.com");
    await page.getByPlaceholder("Password (min 8 chars)").fill("aipassword");
    await page.locator('#step-admin').getByRole('button', { name: 'Next' }).click();

    await expect(page.locator('#first-offer')).toHaveValue("AI Cupcake");
    await page.locator('#step-offer').getByRole('button', { name: 'Next' }).click();

    await page.locator('#template-selection').selectOption('Modern');
    await page.getByRole('button', { name: 'Finish Setup' }).click();

    await expect(page.getByRole('heading', { name: "You're all set!" })).toBeVisible();
  });

  test('Validates 44px touch targets on mobile sizes and layout rules', async ({ page }) => {
    // Set a mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/api/ui/onboarding/setup.html');

    // Wait for the container to be visible
    const container = page.locator('.container');
    await expect(container).toBeVisible();
    await expect(container).toHaveClass(/glassmorphism/);

    const option = page.locator('.radio-option').first();
    const optionBox = await option.boundingBox();
    const catInput = page.getByPlaceholder("e.g. Graphic Design");

    if (optionBox) {
        expect(optionBox.height).toBeGreaterThanOrEqual(44);
    }
  });

});

test.describe('Tauri Dashboard UI and UX Improvements', () => {


  test('Setup UI should have glassmorphism aesthetics applied', async ({ page }) => {
    await page.goto('/api/ui/onboarding/setup.html');

    // Check that the container class has the updated glassmorphism properties
    const container = page.locator('.container');
    await expect(container).toHaveCSS('backdrop-filter', 'blur(30px) saturate(2.1)');
    await expect(container).toHaveCSS('border-radius', '16px');
    await expect(container).toHaveCSS('background-color', 'rgba(255, 255, 255, 0.65)');
    await expect(container).toHaveCSS('border', '1px solid rgba(255, 255, 255, 0.4)');
  });

  test('Dashboard should have glassmorphism aesthetics applied', async ({ page }) => {
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
            throw new Error(`Unhandled command: ${cmd}`);
          }
        }
      };
    });

    await page.goto('/api/ui/dashboard.html');

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
