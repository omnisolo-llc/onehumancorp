import { test, expect } from '@playwright/test';

test.describe('Onboarding UI Audit', () => {

  test('Should navigate the setup successfully via Conversational Setup', async ({ page }) => {
    // Start at the onboarding page
    await page.goto('/setup.html');

    // Wait until start button is visible
    await expect(page.locator('[data-testid="next-step-btn"][data-next="step-context"]')).toBeVisible();

    // Click start setup
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

    // Finish Setup
    await page.locator('#finish-btn').click();

    await expect(page).toHaveURL(/.*dashboard.html.*/, { timeout: 15000 });

  });
});
