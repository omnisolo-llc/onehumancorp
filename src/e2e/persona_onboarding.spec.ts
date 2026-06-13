import { test, expect } from '@playwright/test';

test.describe('Persona-Driven Zero-Click Onboarding E2E Flow', () => {
  test.beforeEach(async ({ page }) => {
    await page.route('/api/onboarding/zero_click', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          message: 'Your business has been successfully launched.',
          organization_id: 'test-org-123'
        }),
      });
    });
    // Fallback if it hits intake
    await page.route('/api/onboarding/intake', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          business_type: 'Bakery',
          business_name: 'Maya Bakery',
          categories: ['food'],
          location: 'Austin',
          initial_products: [{ name: 'Cake', price: '20' }]
        }),
      });
    });
    await page.route('/api/onboarding/start', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          message: 'Your business has been successfully launched.',
          organization_id: 'test-org-123'
        }),
      });
    });
    await page.route('/api/onboarding/state', async route => {
      await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({}) });
    });
  });

  test('Maya the Baker persona journey', async ({ page }) => {
    await page.goto('/onboarding');
    await expect(page.getByRole('heading', { name: "What do you do?" })).toBeVisible();

    const bioInput = page.getByPlaceholder(/I run a local bakery/i);
    await bioInput.fill("I am Maya, a home baker selling custom cakes through Instagram DMs.");

    await page.getByRole('button', { name: 'Generate Storefront' }).click();

    await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 15000 });
  });

  test('Carlos the Field Service Owner persona journey', async ({ page }) => {
    await page.goto('/onboarding');
    await expect(page.getByRole('heading', { name: "What do you do?" })).toBeVisible();

    const bioInput = page.getByPlaceholder(/I run a local bakery/i);
    await bioInput.fill("I am Carlos, I run a repair and home-improvement service from an Android phone.");

    await page.getByRole('button', { name: 'Generate Storefront' }).click();

    await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 15000 });
  });

  test('Priya the Boutique Operator persona journey', async ({ page }) => {
    await page.goto('/onboarding');
    const bioInput = page.getByPlaceholder(/I run a local bakery/i);
    await bioInput.fill("I am Priya, running a clothing shop wanting online demand.");

    await page.getByRole('button', { name: 'Generate Storefront' }).click();
    await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 15000 });
  });

  test('Leo the Creator and Tutor persona journey', async ({ page }) => {
    await page.goto('/onboarding');
    const bioInput = page.getByPlaceholder(/I run a local bakery/i);
    await bioInput.fill("I am Leo, I teach music online and in person.");

    await page.getByRole('button', { name: 'Generate Storefront' }).click();
    await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 15000 });
  });

  test('Fatima the Food Cart Operator persona journey', async ({ page }) => {
    await page.goto('/onboarding');
    const bioInput = page.getByPlaceholder(/I run a local bakery/i);
    await bioInput.fill("I am Fatima, handling pre-orders and daily menus for a food cart.");

    await page.getByRole('button', { name: 'Generate Storefront' }).click();
    await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 15000 });
  });

  test('Nora the Agency Principal persona journey', async ({ page }) => {
    await page.goto('/onboarding');
    const bioInput = page.getByPlaceholder(/I run a local bakery/i);
    await bioInput.fill("I am Nora, running a small design studio with contractors and clients.");

    await page.getByRole('button', { name: 'Generate Storefront' }).click();
    await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 15000 });
  });
});
