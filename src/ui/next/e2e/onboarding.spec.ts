import { test, expect } from '@playwright/test';

test.describe('Onboarding Flow', () => {
  test.beforeEach(async ({ page }) => {
    // Mock intake API
    await page.route('**/api/onboarding/intake', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          business_name: 'Tactile Taps',
          business_type: 'Keyboard Shop',
          categories: ['electronics', 'physical'],
          initial_products: [{ name: 'Custom Mechanical Keyboard', price: '199.00' }]
        }),
      });
    });

    // Mock start API
    await page.route('**/api/onboarding/start', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          success: true,
          message: 'Successfully onboarded Tactile Taps!',
          organization_id: 'org-123'
        }),
      });
    });

    // Mock state API
    await page.route('**/api/onboarding/state', async (route) => {
      if (route.request().method() === 'GET') {
        await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({}) });
      } else {
        await route.fulfill({ status: 200 });
      }
    });

    await page.goto('/onboarding');
  });

  test('completes full onboarding journey', async ({ page }) => {
    // Step 1: Business Type
    await expect(page.getByText('What do you do?')).toBeVisible();
    await page.getByPlaceholder('e.g. I sell organic sourdough bread').fill('I sell high-end mechanical keyboards');
    await page.getByRole('button', { name: 'Next' }).click();

    // Step 2: Business Name
    await expect(page.getByText('Business Name')).toBeVisible();
    await page.getByPlaceholder('e.g. Golden Crust Bakery').fill('Tactile Taps');
    await page.getByRole('button', { name: 'Next' }).click();

    // Step 3: Niche
    await expect(page.getByText('Your Niche')).toBeVisible();
    await page.getByPlaceholder('e.g. Local foodies looking for healthy bread').fill('Software engineers and gamers');
    await page.getByRole('button', { name: 'Generate Draft' }).click();

    // Step 4: Review
    await expect(page.getByText('Review Setup')).toBeVisible();
    await expect(page.locator('input').filter({ hasText: 'Custom Mechanical Keyboard' })).toBeVisible();
    await page.getByRole('button', { name: 'Next' }).click();

    // Step 5: AI Team
    await expect(page.getByText('Your AI Team')).toBeVisible();
    await expect(page.getByText('The Manager')).toBeVisible();
    await expect(page.getByText('The Promoter')).toBeVisible();
    await expect(page.getByText('The Salesperson')).toBeVisible();

    // Toggle an agent
    await page.getByText('The Ambassador').click();
    await page.getByRole('button', { name: 'Next' }).click();

    // Step 6: Final Review
    await expect(page.getByText('Ready to Launch?')).toBeVisible();
    await expect(page.getByText('Tactile Taps')).toBeVisible();
    await expect(page.getByText('4 Agents')).toBeVisible(); // 3 default + 1 added
    await page.getByRole('button', { name: 'Publish Now' }).click();

    // Step 7: Live
    await expect(page.getByText("You're Live!")).toBeVisible();
    await expect(page.getByText('Successfully onboarded Tactile Taps!')).toBeVisible();
    await expect(page.getByRole('link', { name: 'Go to Dashboard' })).toBeVisible();
  });
});
