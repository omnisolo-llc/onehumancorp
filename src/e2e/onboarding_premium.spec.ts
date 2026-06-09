import { test, expect } from '@playwright/test';
import * as path from 'path';
import * as fs from 'fs';

test.describe('OHC Premium Onboarding Wizard', () => {
  const tauriUiDir = path.join(process.cwd(), 'src/ui/tauri/src/ui');

  const routeHandler = async (route) => {
    const url = new URL(route.request().url());
    const filename = path.basename(url.pathname);
    const filepath = path.join(tauriUiDir, filename);
    if (fs.existsSync(filepath)) {
      const content = fs.readFileSync(filepath);
      let contentType = 'text/html';
      if (filename.endsWith('.css')) contentType = 'text/css';
      else if (filename.endsWith('.js')) contentType = 'application/javascript';
      await route.fulfill({ contentType, body: content });
    } else {
      await route.continue();
    }
  };

  test.beforeEach(async ({ page }) => {
    await page.route('http://mock/**', routeHandler);

    await page.addInitScript(() => {
      window.__TAURI__ = {
        core: {
          invoke: async (cmd: string, args: any) => {
            if (cmd === 'get_onboarding_state') {
              const state = sessionStorage.getItem('mockState');
              return state ? JSON.parse(state) : {};
            } else if (cmd === 'save_onboarding_state') {
              const state = sessionStorage.getItem('mockState');
              const currentState = state ? JSON.parse(state) : {};
              sessionStorage.setItem('mockState', JSON.stringify({ ...currentState, ...args.state }));
              return null;
            }
            return {};
          }
        }
      };
    });
  });

  test('Maya Persona: Full Onboarding Journey', async ({ page }) => {
    await page.goto('http://mock/index.html');

    // Step 0: Welcome
    await expect(page.getByRole('heading', { name: 'Welcome to OHC' })).toBeVisible();
    await page.getByRole('button', { name: 'Start Onboarding' }).click();

    // Step 1: Business Profile
    await expect(page.getByRole('heading', { name: 'Business Profile' })).toBeVisible();

    // Interaction Audit: Verify name validation
    const nextBtn = page.getByRole('button', { name: 'Next' });
    await nextBtn.click();
    await expect(page.locator('#name-error')).toBeVisible();
    await expect(page.locator('#industry-error')).toBeVisible();

    await page.getByPlaceholder("e.g. Maya's Bakery").fill("Maya's Custom Cakes");
    await page.getByText("🍰 Bakery").click();
    await nextBtn.click();

    // Step 2: AI Assistant Setup
    await expect(page.getByRole('heading', { name: 'Your AI Team' })).toBeVisible();

    // Verify Agent Team display
    await expect(page.getByText('📋 The Manager (Operations)')).toBeVisible();
    await expect(page.getByText('📣 The Promoter (Marketing)')).toBeVisible();

    await page.getByPlaceholder("e.g. Jarvis").fill("Jarvis");
    await page.selectOption("#assistant-tone", "Friendly");
    await page.getByRole('button', { name: 'Finish Setup' }).click();

    // Step 3: Success
    await expect(page.getByRole('heading', { name: "You're all set!" })).toBeVisible();
    await expect(page.locator('#success-msg')).toContainText("Maya's Custom Cakes");
    await expect(page.locator('#success-msg')).toContainText("Jarvis");
  });

  test('Responsive Layout: 375px Verification', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('http://mock/setup.html');

    const container = page.locator('.container');
    const box = await container.boundingBox();
    expect(box?.width).toBeLessThanOrEqual(375);

    const nameInput = page.getByPlaceholder("e.g. Maya's Bakery");
    const inputBtn = await nameInput.boundingBox();
    expect(inputBtn?.height).toBeGreaterThanOrEqual(44);
  });

  test('State Persistence: Refresh during wizard', async ({ page }) => {
    await page.goto('http://mock/setup.html');
    await page.getByPlaceholder("e.g. Maya's Bakery").fill("Persistent Business");
    await page.getByText("🏢 Agency").click();

    await page.getByRole('button', { name: 'Next' }).click();
    await expect(page.getByRole('heading', { name: 'Your AI Team' })).toBeVisible();

    await page.getByRole('button', { name: 'Back' }).click();
    await page.reload();

    await expect(page.getByPlaceholder("e.g. Maya's Bakery")).toHaveValue("Persistent Business");
    await expect(page.locator('.select-card[data-value="Agency"]')).toHaveClass(/selected/);
  });

  test('Industry Selection Interaction Audit', async ({ page }) => {
    await page.goto('http://mock/setup.html');

    const bakery = page.getByText("🍰 Bakery");
    const agency = page.getByText("🏢 Agency");

    await bakery.click();
    await expect(page.locator('.select-card[data-value="Bakery"]')).toHaveClass(/selected/);
    await expect(page.locator('.select-card[data-value="Agency"]')).not.toHaveClass(/selected/);

    await agency.click();
    await expect(page.locator('.select-card[data-value="Agency"]')).toHaveClass(/selected/);
    await expect(page.locator('.select-card[data-value="Bakery"]')).not.toHaveClass(/selected/);
  });

  test('Dark Mode Visual Audit', async ({ page }) => {
    await page.emulateMedia({ colorScheme: 'dark' });
    await page.goto('http://mock/index.html');

    const bodyColor = await page.evaluate(() => getComputedStyle(document.body).backgroundColor);
    expect(bodyColor).toBe('rgb(22, 22, 26)');
  });
});
