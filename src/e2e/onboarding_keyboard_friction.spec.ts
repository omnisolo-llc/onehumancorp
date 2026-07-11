import { test, expect } from '@playwright/test';
import { memberPage } from './fixtures';

test.describe('Onboarding Keyboard Friction / Business Type mapping', () => {

  test('Should navigate the setup successfully via Conversational Setup', async ({ page }) => {
    await page.goto('/ui/setup.html');

    await expect(page.locator('[data-testid="next-step-btn"][data-next="step-context"]')).toBeVisible();

    await page.locator('[data-testid="next-step-btn"][data-next="step-context"]').click();

    await page.getByText('Agency or Studio').click();
    await page.locator('#step-context .next-step-btn').click();

    await page.locator('#business-categories').selectOption('Design');
    await page.locator('#step-categories .next-step-btn').click();

    await page.locator('#business-name').fill("Nora Studio");
    await page.locator('#step-name .next-step-btn').click();

    await page.getByTestId('team-operations').click();
    await page.locator('#assistant-tone').selectOption('Professional');
    await page.locator('#step-assistant .next-step-btn').click();

    await page.locator('#admin-name').fill('Admin');
    await page.locator('#admin-email').fill('nora@example.com');
    await page.locator('#admin-password').fill('securepassword123');
    await page.locator('#step-admin .next-step-btn').click();

    await page.locator('#first-offer').fill("Logo Design");
    await page.locator('#step-offer .next-step-btn').click();

    await page.locator('#location-input').fill('Portland, OR');
    await page.locator('#step-location .next-step-btn').click();

    await page.locator('#target-audience').fill('Everyone');
    await page.locator('#step-target-audience .next-step-btn').click();

    await page.locator('#domain-name').fill('nora-studio');
    await page.locator('#step-domain .next-step-btn').click();

    await expect(page.locator('#step-template')).toBeVisible();
    await page.locator('#template-selection').selectOption('Modern');

    let startRequestPromise = page.waitForRequest(request =>
      request.url().includes('/api/onboarding/start') && request.method() === 'POST'
    );

    // intercept start_onboarding response to prevent actual redirect so test completes faster
    await page.route('**/api/onboarding/start', async (route) => {
      const body = JSON.parse(route.request().postData() || "{}");
      expect(body.business_type).toBe('Agency');
      route.fulfill({
         status: 200,
         contentType: 'application/json',
         body: JSON.stringify({ organization_id: 'test-org-1' }),
      });
    });

    await page.locator('#finish-btn').click();

    const request = await startRequestPromise;
    const body = JSON.parse(request.postData() || "{}");
    expect(body.business_type).toBe('Agency');
  });
});
