import { test, expect } from '@playwright/test';

test.describe('OnboardingWizard CUJ', () => {
  test.beforeEach(async ({ page, context }) => {
    // Clear local storage to ensure fresh state
    await page.addInitScript(() => {
      window.localStorage.clear();
    });
  });

  test('User can complete onboarding via Zero-Click Chat Agent', async ({ page }) => {
    await page.route('**/*api/onboarding/chat*', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          is_complete: true,
          reply: '[COMPLETE] Give me a minute... I\'m building your business.',
          intake_data: {
            business_name: 'Mock Business',
            business_type: 'Mock Type',
            categories: ['physical'],
            initial_products: [{ name: 'Mock Product', price: '10' }]
          }
        })
      });
    });

    await page.goto('/onboarding');
    await expect(page.getByText("What do you want to build or manage today?")).toBeVisible();

    // Click the predefined chip
    await page.getByText('Cake Shop', { exact: true }).click();

    // Wait for the UI to be in loading state after the chip click (which triggers send)
  });
});
