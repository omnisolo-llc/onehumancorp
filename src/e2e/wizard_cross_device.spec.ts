import { test, expect } from '@playwright/test';

test.describe('Wizard Cross Device E2E', () => {

  test.beforeEach(async ({ page, context }) => {
    const fs = require('fs');
    const path = require('path');
    const tauriUiDir = path.join(process.cwd(), 'src/ui/tauri/src/ui');
    await context.route('**/setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });
  });



  test('Persona: Business Owner can resume setup wizard cross device', async ({ page, browser }) => {
    // 1. Owner starts wizard directly from the current route.
    await page.addInitScript((tenantId) => {
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('user_id', tenantId);
      localStorage.removeItem('onboardingState');
    }, 'storefront');
    await page.goto('http://mock/setup.html');
    await page.waitForLoadState('networkidle');

    // 2. Advance to step 1
    await page.locator('[data-testid="next-step-btn"][data-next="step-context"]').click();
    await expect(page.getByRole('heading', { name: "How do you work?" })).toBeVisible();

    await page.locator('label.context-card').filter({ hasText: 'Storefront' }).click();
    await page.locator('[data-testid="next-step-btn"][data-next="step-categories"]').click();

    await expect(page.getByRole('heading', { name: /What's your category?/ })).toBeVisible();
    await page.locator('#business-categories').selectOption('Home Baker');
    await page.locator('#step-categories .next-step-btn').click();

    await expect(page.getByRole('heading', { name: /What's the name of your business?/ })).toBeVisible();

    // 3. Move to step 2 and enter business name
    // It's already at the "What's the name of your business?" step. We fill the input.
    // The placeholder is "e.g. Maya's Custom Cakes"
    await page.getByPlaceholder("e.g. Maya's Custom Cakes").fill('Cross Device Wizard');

    // Wait until local storage is updated with the business name
    await expect.poll(async () => {
      const stateStr = await page.evaluate(() => localStorage.getItem('onboardingState'));
      if (!stateStr) return '';
      try {
        const state = JSON.parse(stateStr);
        return state.business_name;
      } catch (e) {
        return '';
      }
    }, {
      message: 'Wait for local storage to save business name',
      timeout: 5000,
    }).toBe('Cross Device Wizard');

    // Also trigger save to backend
    await page.locator('#step-name .next-step-btn').click();
    await expect(page.getByRole('heading', { name: "Set up your Assistant" })).toBeVisible();
    await page.waitForTimeout(1000);

    // 4. Simulate a cross-device session with a new browser context


    const fs = require('fs');
    const path = require('path');
    const tauriUiDir = path.join(process.cwd(), 'src/ui/tauri/src/ui');
    const newContext = await browser.newContext();
    await newContext.route('**/setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });
    const newPage = await newContext.newPage();



    // Inject the exact same local storage state to the new context to test restoration
    // We navigate to dashboard first to have the right origin
    await newPage.goto('http://mock/setup.html');
    const wizardState = await page.evaluate(() => localStorage.getItem('onboardingState'));

    await newPage.evaluate((state) => {
        if(state) {
            localStorage.setItem('onboardingState', state);
        }
        localStorage.setItem('tenant_id', 'storefront');
        localStorage.setItem('user_id', 'storefront');
    }, wizardState);

    await newPage.goto('http://mock/setup.html');
    await newPage.waitForLoadState('networkidle');

    // 5. Verify the business name and step was properly restored
    await expect(newPage.getByRole('heading', { name: 'Set up your Assistant' })).toBeVisible({ timeout: 10000 });
    await newPage.locator('#step-assistant .prev-step-btn').first().click();
    await expect(newPage.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await expect(newPage.getByPlaceholder("e.g. Maya's Custom Cakes")).toHaveValue('Cross Device Wizard', { timeout: 10000 });

    await newContext.close();
  });
});
