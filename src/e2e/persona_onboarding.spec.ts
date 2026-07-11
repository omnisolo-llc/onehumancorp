import { test, expect } from '@playwright/test';

test.describe('Persona-Driven Onboarding E2E', () => {

  test('Maya the Baker persona journey', async ({ page }) => {
    await page.goto('/setup.html');
    await page.locator('[data-testid="next-step-btn"][data-next="step-context"]').click();

    await expect(page.getByText("How do you work?")).toBeVisible();
    const bakerChip = page.getByText("I'm a Baker");
    await expect(bakerChip).toBeVisible();
    await bakerChip.click();
    await expect(page.getByText("Applied!")).toBeVisible();
    await expect(page.locator('input[value="Storefront"]')).toBeChecked();
    await page.locator('#step-context .next-step-btn').click();

    await expect(page.locator('#business-categories')).toHaveValue('Home Baker');
    await page.locator('#step-categories .next-step-btn').click();

    await expect(page.locator('#business-name')).toHaveValue("Maya's Bakery");
    await page.locator('#step-name .next-step-btn').click();

    await expect(page.locator('#assistant-tone')).toHaveValue("Friendly");
    await page.locator('#step-assistant .next-step-btn').click();

    await page.locator('#admin-name').fill('Admin');
    await page.locator('#admin-email').fill('maya@example.com');
    await page.locator('#admin-password').fill('securepassword123');
    await page.locator('#step-admin .next-step-btn').click();

    await expect(page.locator('#first-offer')).toHaveValue("Custom Birthday Cake");
    await page.locator('#step-offer .next-step-btn').click();

    await page.locator('#location-input').fill('Portland, OR');
    await page.locator('#step-location .next-step-btn').click();
    await page.locator('#target-audience').fill('Everyone');
    await page.locator('#step-target-audience .next-step-btn').click();
    await page.locator('#domain-name').fill('mayas-bakery');
    await page.locator('#step-domain .next-step-btn').click();
    await expect(page.locator('#step-template')).toBeVisible();
  });

  test('Carlos the Handyman persona journey', async ({ page }) => {
    await page.goto('/setup.html');
    await page.locator('[data-testid="next-step-btn"][data-next="step-context"]').click();

    await page.getByText("I'm a Handyman").click();
    await expect(page.getByText("Applied!")).toBeVisible();
    await page.locator('#step-context .next-step-btn').click();

    await expect(page.locator('#business-categories')).toHaveValue('Handyman');
    await page.locator('#step-categories .next-step-btn').click();

    await expect(page.locator('#business-name')).toHaveValue("Carlos Repairs");
    await page.locator('#step-name .next-step-btn').click();

    await expect(page.locator('#assistant-tone')).toHaveValue("Concise");
    await page.locator('#step-assistant .next-step-btn').click();

    await page.locator('#admin-name').fill('Admin');
    await page.locator('#admin-email').fill('carlos@example.com');
    await page.locator('#admin-password').fill('securepassword123');
    await page.locator('#step-admin .next-step-btn').click();

    await expect(page.locator('#first-offer')).toHaveValue("Standard Repair Visit");
    await page.locator('#step-offer .next-step-btn').click();

    await page.locator('#location-input').fill('Portland, OR');
    await page.locator('#step-location .next-step-btn').click();
    await page.locator('#target-audience').fill('Everyone');
    await page.locator('#step-target-audience .next-step-btn').click();
    await page.locator('#domain-name').fill('carlos-repairs');
    await page.locator('#step-domain .next-step-btn').click();
    await expect(page.locator('#step-template')).toBeVisible();
  });

  test('Priya the Boutique Owner persona journey', async ({ page }) => {
    await page.goto('/setup.html');
    await page.locator('[data-testid="next-step-btn"][data-next="step-context"]').click();
    await page.getByText("I'm a Boutique Owner").click();
    await page.locator('#step-context .next-step-btn').click();
    await expect(page.locator('#business-categories')).toHaveValue('Boutique');
  });

  test('Leo the Tutor persona journey', async ({ page }) => {
    await page.goto('/setup.html');
    await page.locator('[data-testid="next-step-btn"][data-next="step-context"]').click();
    await page.getByText("I'm a Tutor").click();
    await page.locator('#step-context .next-step-btn').click();
    await expect(page.locator('#business-categories')).toHaveValue('Tutoring');
  });

  test('Manual setup flow without persona', async ({ page }) => {
    await page.goto('/setup.html');
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
