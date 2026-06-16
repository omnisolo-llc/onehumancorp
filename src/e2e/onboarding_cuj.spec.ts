import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard CUJ', () => {
  test.beforeEach(async ({ page }) => {
    await page.addInitScript(() => window.localStorage.clear());
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
    await page.route('/api/onboarding/launch', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          status: 'launched',
        }),
      });
    });
  });

  async function startOnboarding(page: import('@playwright/test').Page) {
    await page.goto('/ui/setup.html');
    await expect(page.getByRole('heading', { name: '10-Minute Setup Wizard' })).toBeVisible();
    await page.getByRole('button', { name: 'Start My Business' }).click();
    await expect(page.getByText("Choose a quick-start or select how you work.")).toBeVisible();
  }


  test('Persona: Business Owner completes initial setup successfully', async ({ page }) => {
    await startOnboarding(page);

    // Context
    await page.locator('[data-testid="persona-baker"]').click();
    await page.locator('#step-context .next-step-btn').click();

    // Categories
    await expect(page.getByRole('heading', { name: 'What\'s your category?' })).toBeVisible();
    await page.locator('#step-categories .next-step-btn').click();

    // Name
    await expect(page.getByRole('heading', { name: 'What\'s the name of your business?' })).toBeVisible();
    await page.locator('#step-name .next-step-btn').click();

    // Assistant
    await expect(page.getByRole('heading', { name: 'Set up your Assistant' })).toBeVisible();
    await page.locator('#step-assistant .next-step-btn').click();

    // Admin Credentials
    await expect(page.getByRole('heading', { name: 'Admin Credentials' })).toBeVisible();
    await page.locator('#admin-email').fill('admin@mayabakery.com');
    await page.locator('#admin-password').fill('securepassword123');
    await page.locator('#step-admin .next-step-btn').click();

    // First Offer
    await expect(page.getByRole('heading', { name: 'Your First Offer' })).toBeVisible();
    await page.locator('#step-offer .next-step-btn').click();

    // Template
    await expect(page.getByRole('heading', { name: 'Template Selection' })).toBeVisible();
    await page.locator('#template-selection').selectOption('Modern');
    await page.locator('#finish-btn').click();

    // Verify it transitions to Loading
    await expect(page.getByRole('heading', { name: 'Building Your Business...' })).toBeVisible({ timeout: 15000 });
  });

  test('Persona: Business Owner fails validation on short business name', async ({ page }) => {
    await startOnboarding(page);

    await page.locator('[data-testid="persona-baker"]').click();
    await page.locator('#step-context .next-step-btn').click();

    await expect(page.getByRole('heading', { name: 'What\'s your category?' })).toBeVisible();
    await page.locator('#step-categories .next-step-btn').click();

    await expect(page.getByRole('heading', { name: 'What\'s the name of your business?' })).toBeVisible();

    // Fill short name
    await page.locator('#business-name').fill('Ma');
    await page.locator('#step-name .next-step-btn').click();

    // Expect validation failure message
    await expect(page.getByText('Business Name must be at least 3 characters.')).toBeVisible();
    // Verify we are still on the name step
    await expect(page.getByRole('heading', { name: 'What\'s the name of your business?' })).toBeVisible();
  });

  test('Persona: Business Owner cannot progress without email on admin step', async ({ page }) => {
    await startOnboarding(page);

    await page.locator('[data-testid="persona-baker"]').click();
    await page.locator('#step-context .next-step-btn').click();
    await page.locator('#step-categories .next-step-btn').click();
    await page.locator('#step-name .next-step-btn').click();
    await page.locator('#step-assistant .next-step-btn').click();

    await expect(page.getByRole('heading', { name: 'Admin Credentials' })).toBeVisible();

    // Fill only password
    await page.locator('#admin-password').fill('securepassword123');

    // Try to proceed
    await page.locator('#step-admin .next-step-btn').click();

    // Expect validation failure
    await expect(page.getByText('Please enter a valid email address.')).toBeVisible();
  });

  test('Persona: Business Owner can navigate back from categories to context', async ({ page }) => {
    await startOnboarding(page);

    await page.locator('[data-testid="persona-baker"]').click();
    await page.locator('#step-context .next-step-btn').click();

    await expect(page.getByRole('heading', { name: 'What\'s your category?' })).toBeVisible();

    // Click back
    await page.locator('#step-categories .prev-step-btn').click();

    await expect(page.getByText("Choose a quick-start or select how you work.")).toBeVisible();
  });

  test('Persona: Business Owner can toggle Auto Respond on Admin step', async ({ page }) => {
    await startOnboarding(page);

    await page.locator('[data-testid="persona-baker"]').click();
    await page.locator('#step-context .next-step-btn').click();
    await page.locator('#step-categories .next-step-btn').click();
    await page.locator('#step-name .next-step-btn').click();
    await page.locator('#step-assistant .next-step-btn').click();

    await expect(page.getByRole('heading', { name: 'Admin Credentials' })).toBeVisible();

    // Toggle auto-respond
    const autoRespondToggle = page.locator('#ai-auto-respond');
    await expect(autoRespondToggle).toBeChecked();

    await page.getByText('Allow AI to Auto-Respond').click();

    await expect(autoRespondToggle).not.toBeChecked();
  });
});
