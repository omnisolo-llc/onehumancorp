import { test, expect } from '@playwright/test';

test.describe('Persona-Driven Onboarding E2E', () => {

  test('Maya the Baker persona journey', async ({ page }) => {
    // Start from the Tauri setup page

    const fs = require('fs');
    const path = require('path');
    await page.route('**/setup.html', async route => {
        const fileContent = fs.readFileSync(path.join(process.cwd(), 'src/ui/tauri/src/ui/setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: fileContent });
    });
    await page.goto('http://mock/setup.html');


    // Step 1: Work Context & Persona Quick-Start
    await expect(page.getByText("How do you work?")).toBeVisible();

    // Click "I'm a Baker" persona chip
    const bakerChip = page.getByText("I'm a Baker");
    await expect(bakerChip).toBeVisible();
    await bakerChip.click();
    await expect(page.getByText("Applied!")).toBeVisible();

    // Verify radio "Storefront" is selected (implied by persona)
    const storefrontRadio = page.locator('input[value="Storefront"]');
    await expect(storefrontRadio).toBeChecked();

    await page.getByText('Next').first().click();

    // Step 2: Categories
    await expect(page.getByText("What's your category?")).toBeVisible();
    const categoriesSelect = page.locator('#business-categories');
    await expect(categoriesSelect).toHaveValue('Bakery');
    await page.getByRole('button', { name: 'Next' }).click();

    // Step 3: Business Name
    await expect(page.getByText("What's the name of your business?")).toBeVisible();
    const nameInput = page.locator('#business-name');
    await expect(nameInput).toHaveValue("Maya's Bakery");
    await page.getByRole('button', { name: 'Next' }).click();

    // Step 4: Assistant Setup
    await expect(page.getByText("Set up your Assistant")).toBeVisible();
    await expect(page.locator('#assistant-intro')).toContainText("partner in growing this business");
    const assistantName = page.locator('#assistant-name');
    await expect(assistantName).toHaveValue("Cookie");
    const assistantTone = page.locator('#assistant-tone');
    await expect(assistantTone).toHaveValue("Friendly");
    await page.getByRole('button', { name: 'Next' }).click();

    // Step 5: First Offer
    await expect(page.getByText("Your First Offer")).toBeVisible();
    const firstOffer = page.locator('#first-offer');
    await expect(firstOffer).toHaveValue("Custom Birthday Cake");
    await page.getByRole('button', { name: 'Next' }).click();

    // Step 6: Template
    await expect(page.getByText("Template Selection")).toBeVisible();
  });

  test('Carlos the Handyman persona journey', async ({ page }) => {

    const fs = require('fs');
    const path = require('path');
    await page.route('**/setup.html', async route => {
        const fileContent = fs.readFileSync(path.join(process.cwd(), 'src/ui/tauri/src/ui/setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: fileContent });
    });
    await page.goto('http://mock/setup.html');

    await page.getByText("I'm a Handyman").click();
    await expect(page.getByText("Applied!")).toBeVisible();
    await expect(page.locator('input[value="Local Service"]')).toBeChecked();
    await page.getByText('Next').first().click();

    await expect(page.locator('#business-categories')).toHaveValue('Handyman');
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.locator('#business-name')).toHaveValue("Carlos Repairs");
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.locator('#assistant-name')).toHaveValue("Tools");
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.locator('#first-offer')).toHaveValue("Standard Repair Visit");
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.getByText("Template Selection")).toBeVisible();
  });

  test('Priya the Boutique Owner persona journey', async ({ page }) => {

    const fs = require('fs');
    const path = require('path');
    await page.route('**/setup.html', async route => {
        const fileContent = fs.readFileSync(path.join(process.cwd(), 'src/ui/tauri/src/ui/setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: fileContent });
    });
    await page.goto('http://mock/setup.html');

    await page.getByText("I'm a Boutique Owner").click();
    await page.getByText('Next').first().click();
    await expect(page.locator('#business-categories')).toHaveValue('Boutique');
    await page.getByRole('button', { name: 'Next' }).click();
    await expect(page.locator('#business-name')).toHaveValue("Priya's Boutique");
  });

  test('Leo the Tutor persona journey', async ({ page }) => {

    const fs = require('fs');
    const path = require('path');
    await page.route('**/setup.html', async route => {
        const fileContent = fs.readFileSync(path.join(process.cwd(), 'src/ui/tauri/src/ui/setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: fileContent });
    });
    await page.goto('http://mock/setup.html');

    await page.getByText("I'm a Tutor").click();
    await page.getByText('Next').first().click();
    await expect(page.locator('#business-categories')).toHaveValue('Tutoring');
    await page.getByRole('button', { name: 'Next' }).click();
    await expect(page.locator('#business-name')).toHaveValue("Leo's Music");
  });

  test.skip('Manual setup flow without persona', async ({ page }) => {

    const fs = require('fs');
    const path = require('path');
    await page.route('**/setup.html', async route => {
        const fileContent = fs.readFileSync(path.join(process.cwd(), 'src/ui/tauri/src/ui/setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: fileContent });
    });
    await page.goto('http://mock/setup.html');

    await page.getByText('Agency or Studio').click();
    await page.getByText('Next').first().click();

    await page.locator('#business-categories').selectOption('Design');
    await page.getByRole('button', { name: 'Next' }).click();

    await page.locator('#business-name').fill("Nora Studio");
    await page.getByRole('button', { name: 'Next' }).click();

    await page.locator('#assistant-name').fill("Dash");
    await page.locator('#assistant-tone').selectOption('Professional');
    await page.getByRole('button', { name: 'Next' }).click();

    await page.locator('#first-offer').fill("Logo Design");
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.getByText("Template Selection")).toBeVisible();
    await page.locator('#template-selection').selectOption('Modern');

    // Finish Setup

    await page.route('**/api/onboarding/start', async route => {
        await route.fulfill({ status: 200, contentType: 'application/json', body: '{}' });
    });
    const finishBtn = page.locator('#finish-btn');
    await finishBtn.click();

    // Verification: Success Page
    await expect(page).toHaveURL(/success.html/);
    await expect(page.getByText("You're all set!")).toBeVisible();
  });
});
