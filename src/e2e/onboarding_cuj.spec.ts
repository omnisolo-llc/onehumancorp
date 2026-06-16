import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard CUJ', () => {
  test.beforeEach(async ({ page }) => {
    await page.addInitScript(() => window.localStorage.clear());
    await page.route('/api/onboarding/intake', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          business_type: 'Bakery',
          business_name: 'Maya Bakery',
          categories: ['food'],
          initial_products: [{ name: 'Cake', price: '20' }]
        }),
      });
    });
    await page.route('/api/onboarding/state', async route => {
      if (route.request().method() === 'GET') {
        await route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({}),
        });
        return;
      }

      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({}),
      });
    });
    await page.route('/api/onboarding/start', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          message: 'Your business has been successfully launched.',
        }),
      });
    });
  });

  async function startOnboarding(page: import('@playwright/test').Page) {
    await page.goto('/onboarding');
    await expect(page.getByRole('heading', { name: 'Tell us about your business' })).toBeVisible();
  }


  // Test 1: Persona navigates from home, starts onboarding
  test('Persona: Business Owner completes initial setup successfully', async ({ page }) => {
    await startOnboarding(page);

    // Owner enters business description
    const bioInput = page.getByPlaceholder(/I run a local bakery/i);
    await bioInput.fill('Maya Bakery selling custom cakes in NY to tech enthusiasts');
    await page.getByTestId('admin-email').fill('maya@example.com');
    await page.getByTestId('admin-password').fill('mypassword123');
    await page.getByRole('button', { name: 'Next', exact: true }).click();

    // Verify it transitions to Building
    await expect(page.getByText('Building Your Business...')).toBeVisible({ timeout: 15000 });

    // Verify it transitions to Live Screen
    await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 15000 });
  });

  // Test 2: Ensure Next button is disabled on empty bio
  test('Persona: Business Owner cannot progress with empty description', async ({ page }) => {
    await startOnboarding(page);

    // Next button should be disabled
    const generateBtn = page.getByRole('button', { name: 'Next', exact: true });
    await expect(generateBtn).toBeDisabled();
  });
});
