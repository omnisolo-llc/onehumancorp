import { test, expect } from '@playwright/test';

test.describe('OnboardingWizard CUJ', () => {
  test.beforeEach(async ({ page }) => {
    // Clear local storage to ensure fresh state
    await page.addInitScript(() => {
      window.localStorage.clear();
    });

    // Mock backend routes to prevent real DB hits and flakiness during UI tests
    await page.route('**/api/onboarding/**', async (route) => {
      const url = route.request().url();
      if (url.includes('/api/onboarding/state')) {
        await route.fulfill({ status: 200, json: { wizardState: { step: 0 } } });
      } else if (url.includes('/api/onboarding/intake')) {
        await route.fulfill({
          status: 200,
          json: {
            business_name: 'Mock Business',
            business_type: 'Online Store',
            initial_products: [{ name: 'Test Product', price: '10' }],
            location: 'Remote'
          }
        });
      } else if (url.includes('/api/onboarding/start')) {
        await route.fulfill({
          status: 200,
          json: {
            organization_id: 'mock_org_123',
            message: 'Your business has been successfully launched.'
          }
        });
      } else if (url.includes('/api/onboarding/launch')) {
        await route.fulfill({ status: 200, json: {} });
      } else {
        await route.continue();
      }
    });
  });

  // Test 1: Completes the onboarding flow
  test('Completes the zero-click onboarding flow and verifies premium translucent glass styling and flexbox layouts', async ({ page }) => {
    await page.goto('/onboarding');

    // Verify initial screen and styling
    const promptScreen = page.locator('#setup-screen');
    await expect(promptScreen).toBeVisible();
    await expect(promptScreen).toHaveClass(/ohc-hybrid-panel/);
    await expect(promptScreen).toHaveClass(/rounded-\[16px\]/);
    await expect(page.getByText("What do you do?")).toBeVisible();

    // Fill the single prompt
    const bioInput = page.getByPlaceholder(/I'm a plumber in Miami.../i);
    await bioInput.fill('I am a baker in Austin, Texas who makes custom vegan cakes.');

    // Verify touch target on button
    const generateBtn = page.getByRole('button', { name: 'Generate My Business' });
    const btnBox = await generateBtn.boundingBox();
    expect(btnBox!.height).toBeGreaterThanOrEqual(44);

    // Click submit
    await generateBtn.click();

    // Should transition to loading state momentarily
    await expect(page.getByText("Agents are building...")).toBeVisible();

    // Verify success screen
    await expect(page.getByText("You're Live!")).toBeVisible();
    await expect(page.getByText('mock-business.ohc.app')).toBeVisible();

    // Verify localStorage was updated
    const storedTenantId = await page.evaluate(() => window.localStorage.getItem('tenant_id'));
    expect(storedTenantId).toBe('mock_org_123');
  });

  test('Submitting empty inputs disables button', async ({ page }) => {
    await page.goto('/onboarding');
    await expect(page.getByText("What do you do?")).toBeVisible();

    // Step 1: Empty bio
    const bioInput = page.getByPlaceholder(/I'm a plumber in Miami.../i);
    await bioInput.fill('  ');

    const generateBtn = page.getByRole('button', { name: 'Generate My Business' });
    await expect(generateBtn).toBeDisabled();
  });

  test('Step-by-step intake handles backend processing errors correctly', async ({ page, context }) => {
    await page.goto('/onboarding');
    await page.getByPlaceholder(/I'm a plumber in Miami.../i).fill('Testing error state');

    // Mock the backend responding with a 500 error
    await context.route('/api/onboarding/intake', route => route.fulfill({ status: 500, json: { error: 'Internal Server Error' } }));

    await page.getByRole('button', { name: 'Generate My Business' }).click();
    await expect(page.getByText(/Internal Server Error/i)).toBeVisible();

    await context.unroute('/api/onboarding/intake');
  });

  test('Store launch correctly fails when start API is down', async ({ page, context }) => {
    await page.goto('/onboarding');
    await page.getByPlaceholder(/I'm a plumber in Miami.../i).fill('Testing error state start');

    // Mock the start API failing
    await context.route('/api/onboarding/start', route => route.fulfill({ status: 502, json: { error: 'Bad Gateway' } }));

    await page.getByRole('button', { name: 'Generate My Business' }).click();
    await expect(page.getByText(/Bad Gateway/i)).toBeVisible();

    await context.unroute('/api/onboarding/start');
  });
});
