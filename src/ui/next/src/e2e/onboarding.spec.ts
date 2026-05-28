import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard E2E Tests', () => {

  test.beforeEach(async ({ page }) => {
    // Navigate to builder to clear storage if any
    await page.goto('http://localhost:3000');
    await page.evaluate(() => localStorage.clear());
    await page.goto('http://localhost:3000/onboarding');
  });

  test('Basic rendering of the wizard', async ({ page }) => {
    await expect(page.getByText("What's the name of your business?")).toBeVisible();
    await expect(page.getByRole('button', { name: /Next/i })).toBeDisabled();
  });

  test('Validating step transitions', async ({ page }) => {
    // We should mock intake here because otherwise it will call the real API and wait for it forever or fail.
    await page.route('**/api/onboarding/state', route => route.fulfill({ status: 200, json: {} }));

    // Fill first step
    await page.getByRole('textbox').fill('Maya Bakery');
    await page.getByRole('button', { name: /Next/i }).click();

    // Fill second step
    await expect(page.getByText("What do you sell")).toBeVisible();
    await page.getByRole('textbox').fill('Cakes');
    await page.getByRole('button', { name: /Next/i }).click();

    // Fill third step
    await expect(page.getByText("Where are you located?")).toBeVisible();
    await page.getByRole('textbox').fill('NY');
    await expect(page.getByRole('button', { name: /Generate My Business/i })).not.toBeDisabled();
  });

  test('Handling successful intake (mocked)', async ({ page }) => {
    await page.route('**/api/onboarding/state', route => route.fulfill({ status: 200, json: {} }));
    await page.route('**/api/onboarding/intake', route => route.fulfill({
      status: 200,
      json: {
        business_type: 'Bakery',
        business_name: 'Maya Bakery',
        categories: ['food'],
        initial_products: [{ name: 'Cake', price: '20' }]
      }
    }));

    await page.getByRole('textbox').fill('Maya Bakery');
    await page.getByRole('button', { name: /Next/i }).click();

    await page.getByRole('textbox').fill('Cakes');
    await page.getByRole('button', { name: /Next/i }).click();

    await page.getByRole('textbox').fill('NY');
    await page.getByRole('button', { name: /Generate My Business/i }).click();

    await expect(page.getByText("Review Details")).toBeVisible();
    await expect(page.locator('input[value="Maya Bakery"]')).toBeVisible();
  });

  test('Handling successful launch (mocked)', async ({ page }) => {
    await page.route('**/api/onboarding/state', route => route.fulfill({ status: 200, json: {} }));
    await page.route('**/api/onboarding/intake', route => route.fulfill({
      status: 200,
      json: {
        business_type: 'Bakery',
        business_name: 'Maya Bakery',
        categories: ['food'],
        initial_products: [{ name: 'Cake', price: '20' }]
      }
    }));
    await page.route('**/api/onboarding/start', route => route.fulfill({
      status: 200,
      json: { message: "Success!" }
    }));

    // Advance to review step
    await page.getByRole('textbox').fill('Maya Bakery');
    await page.getByRole('button', { name: /Next/i }).click();
    await page.getByRole('textbox').fill('Cakes');
    await page.getByRole('button', { name: /Next/i }).click();
    await page.getByRole('textbox').fill('NY');
    await page.getByRole('button', { name: /Generate My Business/i }).click();

    // In Review step, continue
    await expect(page.getByText("Review Details")).toBeVisible();
    await page.getByRole('button', { name: /Continue/i }).click();

    // In Style & Team step, launch
    await expect(page.getByText("Style & Team")).toBeVisible();
    await page.getByRole('button', { name: /Launch Store/i }).click();

    // Verify success screen
    await expect(page.getByText("You're Live!")).toBeVisible();
  });

  test('Verifying cross-device/state saving API call is triggered', async ({ page }) => {
    // Wait for the state saving API call to be triggered
    await page.route('**/api/onboarding/state', async (route) => {
        await route.fulfill({ status: 200, json: {} });
    });

    let callFound = false;
    page.on('request', request => {
      if (request.url().includes('/api/onboarding/state') && request.method() === 'POST') {
        callFound = true;
      }
    });

    await page.getByRole('textbox').fill('Maya Bakery');
    await page.getByRole('button', { name: /Next/i }).click();

    // Check if the state sync was triggered
    await expect.poll(() => callFound, { timeout: 10000 }).toBe(true);
  });
});
