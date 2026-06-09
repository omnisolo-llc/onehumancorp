import { test, expect } from '@playwright/test';

test.describe('Cross Device Onboarding CUJ', () => {
  let backendState: any = { wizardState: { step: 0 } };

  test.beforeEach(async ({ page }) => {
    backendState = { wizardState: { step: 0 } };
    await page.route('**/api/onboarding/draft', async (route, request) => {
      if (request.method() === 'POST') {
        const body = JSON.parse(request.postData() || '{}');
        backendState = body;
        await route.fulfill({ status: 200, json: {} });
      } else {
        await route.fulfill({ status: 200, json: backendState });
      }
    });

    await page.route('**/api/onboarding/state', async (route, request) => {
      if (request.method() === 'POST') {
        const body = JSON.parse(request.postData() || '{}');
        backendState = body;
        await route.fulfill({ status: 200, json: {} });
      } else {
        await route.fulfill({ status: 200, json: backendState });
      }
    });
  });

  test('Persona: Business Owner can save draft and resume cross device', async ({ page, browser }) => {
    await page.goto('/onboarding');

    await page.evaluate(() => {
      localStorage.setItem('tenant_id', 'storefront');
      localStorage.setItem('user_id', 'test-user');
    });
    await page.goto('/onboarding');

    const startButton = page.locator('button', { hasText: 'Start My Business' });
    await startButton.waitFor({ state: 'visible', timeout: 10000 }).catch(() => {});
    if (await startButton.isVisible()) {
      await startButton.click({ force: true });
    }

    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();

    const nameInput = page.getByPlaceholder(/e.g. Maya's Custom Cakes/i);
    await nameInput.fill('Cross Device Bakery');

    const saveDraftBtn = page.locator('button', { hasText: 'Save Draft' }).first();
    await saveDraftBtn.click();
    await expect(page.getByText('Draft Saved!')).toBeVisible();

    await page.waitForTimeout(500);

    const newContext = await browser.newContext();
    const newPage = await newContext.newPage();

    // Setup mock on new context
    await newPage.route('**/api/onboarding/draft', async (route, request) => {
      if (request.method() === 'POST') {
        const body = JSON.parse(request.postData() || '{}');
        backendState = body;
        await route.fulfill({ status: 200, json: {} });
      } else {
        await route.fulfill({ status: 200, json: backendState });
      }
    });
    await newPage.route('**/api/onboarding/state', async (route, request) => {
      if (request.method() === 'POST') {
        const body = JSON.parse(request.postData() || '{}');
        backendState = body;
        await route.fulfill({ status: 200, json: {} });
      } else {
        await route.fulfill({ status: 200, json: backendState });
      }
    });

    await newPage.goto('/onboarding'); // Navigate to the same domain first
    await newPage.evaluate(() => {
      localStorage.setItem('tenant_id', 'storefront');
      localStorage.setItem('user_id', 'test-user');
    });

    await newPage.goto('/onboarding'); // Re-navigate so it picks up the correct user id

    const newStartButton = newPage.locator('button', { hasText: 'Start My Business' });
    await newStartButton.waitFor({ state: 'visible', timeout: 5000 }).catch(() => {});
    if (await newStartButton.isVisible()) {
      await newStartButton.click({ force: true });
    }

    await expect(newPage.getByPlaceholder(/e.g. Maya's Custom Cakes/i)).toHaveValue('Cross Device Bakery', { timeout: 10000 });

    await newContext.close();
  });
});
