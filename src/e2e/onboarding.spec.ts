import { test, expect } from './fixtures';

test.describe('Onboarding Wizard - Real Business Owner E2E Standard', () => {
  test('Maya the Baker sets up her store using mobile dimensions', async ({ page, request }) => {
    // Set viewport to mobile 375px
    await page.setViewportSize({ width: 375, height: 812 });

    // Mock API requests for deterministic E2E in standard Playwright testing
    await page.route('/api/onboarding/state', async route => {
      if (route.request().method() === 'GET') {
        await route.fulfill({ json: { wizardState: {} } });
      } else {
        await route.fulfill({ json: { success: true } });
      }
    });

    await page.route('/api/onboarding/intake', async route => {
      await route.fulfill({
        json: {
          business_type: 'Bakery',
          business_name: 'Maya Bakery',
          categories: ['food'],
          initial_products: [{ name: 'Cake', price: '20' }]
        }
      });
    });

    await page.route('/api/onboarding/start', async route => {
      await route.fulfill({ json: { success: true } });
    });

    // Navigate to the onboarding route directly
    await page.goto('/onboarding');

    // Chat Step 1
    await expect(page.getByRole('heading', { name: /Tell us about your business/i })).toBeVisible();
    await page.getByPlaceholderText(/Maya's Custom Cakes/i).fill('Maya Bakery');
    await page.getByRole('button', { name: /Next/i }).click();

    // Chat Step 2
    await expect(page.getByRole('heading', { name: /What do you sell\?/i })).toBeVisible();
    await page.getByPlaceholderText(/I bake custom vegan cakes/i).fill('Cakes');
    await page.getByRole('button', { name: /Next/i }).click();

    // Chat Step 3
    await expect(page.getByRole('heading', { name: /Where are you located\?/i })).toBeVisible();
    await page.getByPlaceholderText(/Portland, OR/i).fill('Portland, OR');
    await page.getByRole('button', { name: /Generate My Business/i }).click();

    // Step 1 Review
    await expect(page.getByRole('heading', { name: /Review Details/i })).toBeVisible({ timeout: 15000 });
    await page.locator('input[value="Maya Bakery"]').first().waitFor({ state: 'visible' });
    await page.getByRole('button', { name: /Next: Style & AI Team/i }).click();

    // Step 2 Style & Team
    await expect(page.getByRole('heading', { name: /Style & Team/i })).toBeVisible();
    // Select website template
    await page.getByText('E-Commerce').click();

    // Check an AI agent
    await page.getByText('Marketing & Advertising').click();

    // Launch Business
    await page.getByRole('button', { name: /Launch My Business/i }).click();

    // Building Business Step
    await expect(page.getByRole('heading', { name: /Building Your Business.../i })).toBeVisible();

    // Live Screen Step
    await expect(page.getByRole('heading', { name: /You're Live!/i })).toBeVisible({ timeout: 20000 });

    // Check that we can go to dashboard
    await expect(page.getByRole('link', { name: /Go to Dashboard/i })).toBeVisible();
  });
});
