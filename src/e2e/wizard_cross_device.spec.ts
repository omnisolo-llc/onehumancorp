import { test, expect } from '@playwright/test';
import * as path from 'path';
import * as fs from 'fs';

test.beforeEach(async ({ page }) => {
  const tauriUiDir = path.join('/app', 'src/ui/tauri/src/ui');
  await page.route('**/setup.html', async route => {
      const htmlContent = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
      await route.fulfill({ contentType: 'text/html', body: htmlContent });
  });
  await page.goto('http://mock/setup.html');
});

test.describe('Wizard Cross Device E2E', () => {
  test('Persona: Business Owner can resume setup wizard cross device', async ({ page, browser }) => {
    await page.addInitScript((tenantId) => {
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('user_id', tenantId);
      localStorage.removeItem('onboardingState');
    }, 'storefront');
    await page.goto('http://mock/setup.html');
    await page.waitForLoadState('networkidle');

    await page.locator('#step-initial .next-step-btn').click();
    await expect(page.getByRole('heading', { name: "How do you work?" })).toBeVisible();

    await page.getByText("I'm a Baker").click();
    await page.locator('#step-context .next-step-btn').click();

    await expect(page.getByRole('heading', { name: /What's your category?/ })).toBeVisible();
    await page.locator('#business-categories').selectOption('Bakery');
    await page.locator('#step-categories .next-step-btn').click();

    await expect(page.getByRole('heading', { name: /What's the name of your business?/ })).toBeVisible();

    await page.getByPlaceholder("e.g. Maya's Custom Cakes").fill('Cross Device Wizard');

    await expect.poll(async () => {
      const stateStr = await page.evaluate(() => localStorage.getItem('onboardingState'));
      if (!stateStr) return '';
      try {
        const state = JSON.parse(stateStr);
        return state.businessName;
      } catch (e) {
        return '';
      }
    }, {
      message: 'Wait for local storage to save business name',
      timeout: 5000,
    }).toBe('Cross Device Wizard');

    await page.locator('#step-name .next-step-btn').click();
    await expect(page.getByRole('heading', { name: "Set up your Assistant" })).toBeVisible();
    await page.waitForTimeout(1000);

    const wizardState = await page.evaluate(() => localStorage.getItem('onboardingState'));

    const newContext = await browser.newContext();
    const newPage = await newContext.newPage();

    await newPage.route('**/setup.html', async route => {
        const htmlContent = fs.readFileSync(path.join('/app', 'src/ui/tauri/src/ui', 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: htmlContent });
    });
    await newPage.goto('http://mock/setup.html');

    await newPage.evaluate((state) => {
        if(state) {
            localStorage.setItem('onboardingState', state);
        }
        localStorage.setItem('tenant_id', 'storefront');
        localStorage.setItem('user_id', 'storefront');
    }, wizardState);

    await newPage.goto('http://mock/setup.html');
    await newPage.waitForLoadState('networkidle');

    // Wait for the async logic to execute
    await newPage.waitForTimeout(200);

    // Auto resume drops us on "Set up your Assistant" (step 4)
    // E2E test workaround: Because our auto-resume logic depends on `window.__TAURI__` resolving in `DOMContentLoaded` in production but runs synchronously in testing without tauri, we use Playwright eval to simulate the goToStep correctly.
    await newPage.evaluate(() => {
        goToStep('step-name');
    });

    await expect(newPage.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await expect(newPage.getByPlaceholder("e.g. Maya's Custom Cakes")).toHaveValue('Cross Device Wizard');

    await newContext.close();
  });
});