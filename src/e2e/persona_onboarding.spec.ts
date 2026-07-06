import { test, expect } from '@playwright/test';
import * as fs from 'fs';
import * as path from 'path';

test.describe('Persona-Driven Onboarding E2E', () => {

  const setupMockRoutes = async (page) => {
    // DO NOT use global network rejections in the file here because it's forbidden.
    // E2E Tests should ideally run against real backend.
    // We are mocking ONLY the frontend setup.html routing because the test instructs us to start from the tauri setup page.
    await page.route('**/*success.html*', async route => {
        await route.fulfill({ contentType: 'text/html', body: 'Success' });
    });
    await page.route('**/setup.html', async route => {
        const fileContent = fs.readFileSync(path.join(process.cwd(), 'src/ui/tauri/src/ui/setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: fileContent });
    });
    // This is already in the file at line 144
  };

  test('Maya the Baker persona journey', async ({ page }) => {
    await setupMockRoutes(page);
    await page.goto('http://mock/setup.html');
    await page.getByRole('button', { name: 'Step-by-Step Setup' }).click();

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
    await setupMockRoutes(page);
    await page.goto('http://mock/setup.html');
    await page.getByRole('button', { name: 'Step-by-Step Setup' }).click();

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
    await setupMockRoutes(page);
    await page.goto('http://mock/setup.html');
    await page.getByRole('button', { name: 'Step-by-Step Setup' }).click();
    await page.getByText("I'm a Boutique Owner").click();
    await page.locator('#step-context .next-step-btn').click();
    await expect(page.locator('#business-categories')).toHaveValue('Boutique');
  });

  test('Leo the Tutor persona journey', async ({ page }) => {
    await setupMockRoutes(page);
    await page.goto('http://mock/setup.html');
    await page.getByRole('button', { name: 'Step-by-Step Setup' }).click();
    await page.getByText("I'm a Tutor").click();
    await page.locator('#step-context .next-step-btn').click();
    await expect(page.locator('#business-categories')).toHaveValue('Tutoring');
  });

  test('Manual setup flow without persona', async ({ page }) => {
    await setupMockRoutes(page);
    await page.goto('http://mock/setup.html');
    await page.getByRole('button', { name: 'Step-by-Step Setup' }).click();

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

    // Finish Setup - This matches the original test's logic for mocking the final api call before finish
    await page.route('**/api/onboarding/start', async route => {
        await route.fulfill({ status: 200, contentType: 'application/json', body: '{}' });
    });
    await page.locator('#finish-btn').click();

    await expect(page).toHaveURL(/.*success.html.*/);
  });
});
