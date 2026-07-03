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
    await page.locator('#step-initial .next-step-btn').click();


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

    await page.locator('#step-context .next-step-btn').click();

    // Step 2: Categories
    await expect(page.getByText("What's your category?")).toBeVisible();
    const categoriesSelect = page.locator('#business-categories');
    await expect(categoriesSelect).toHaveValue('Home Baker');
    await page.locator('.step.active .next-step-btn').click();

    // Step 3: Business Name
    await expect(page.getByText("What's the name of your business?")).toBeVisible();
    const nameInput = page.locator('#business-name');
    await expect(nameInput).toHaveValue("Maya's Bakery");
    await page.locator('.step.active .next-step-btn').click();

    // Step 4: Assistant Setup
    await expect(page.getByText("Set up your Assistant")).toBeVisible();
    await expect(page.locator('#assistant-intro')).toContainText("partner in growing this business");
    await page.getByTestId('team-operations').click();

    const assistantTone = page.locator('#assistant-tone');
    await expect(assistantTone).toHaveValue("Friendly");
    await page.locator('.step.active .next-step-btn').click();


    // Step 5: Admin Credentials
    await expect(page.getByText("Admin Credentials")).toBeVisible();
    await page.locator('#admin-name').fill('Admin');
    await page.locator('#admin-email').fill('maya@example.com');
    await page.locator('#admin-password').fill('securepassword123');
    await page.locator('.step.active .next-step-btn').click();

    // Step 6: First Offer
    await expect(page.getByText("Your First Offer")).toBeVisible();
    const firstOffer = page.locator('#first-offer');
    await expect(firstOffer).toHaveValue("Custom Birthday Cake");
    await page.locator('.step.active .next-step-btn').click();

    // Step 6: Template
    await page.locator('#location-input').fill('Portland, OR');
    await page.locator('.step.active .next-step-btn').click();
    await page.locator('#target-audience').fill('Everyone');
    await page.locator('.step.active .next-step-btn').click();
    await page.locator('.step.active .next-step-btn').click();
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
    await page.locator('#step-initial .next-step-btn').click();

    await page.getByText("I'm a Handyman").click();
    await expect(page.getByText("Applied!")).toBeVisible();
    await expect(page.locator('input[value="Local Service"]')).toBeChecked();
    await page.locator('#step-context .next-step-btn').click();

    await expect(page.locator('#business-categories')).toHaveValue('Handyman');
    await page.locator('.step.active .next-step-btn').click();

    await expect(page.locator('#business-name')).toHaveValue("Carlos Repairs");
    await page.locator('.step.active .next-step-btn').click();



    await page.locator('.step.active .next-step-btn').click();

    await page.locator('#admin-name').fill('Admin');
    await page.locator('#admin-email').fill('carlos@example.com');
    await page.locator('#admin-password').fill('securepassword123');
    await page.locator('.step.active .next-step-btn').click();

    await expect(page.locator('#first-offer')).toHaveValue("Standard Repair Visit");
    await page.locator('.step.active .next-step-btn').click();

    await page.locator('#location-input').fill('Portland, OR');
    await page.locator('.step.active .next-step-btn').click();
    await page.locator('#target-audience').fill('Everyone');
    await page.locator('.step.active .next-step-btn').click();
    await page.locator('.step.active .next-step-btn').click();
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
    await page.locator('#step-initial .next-step-btn').click();

    await page.getByText("I'm a Boutique Owner").click();
    await page.locator('#step-context .next-step-btn').click();
    await expect(page.locator('#business-categories')).toHaveValue('Boutique');
    await page.locator('.step.active .next-step-btn').click();
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
    await page.locator('#step-initial .next-step-btn').click();

    await page.getByText("I'm a Tutor").click();
    await page.locator('#step-context .next-step-btn').click();
    await expect(page.locator('#business-categories')).toHaveValue('Tutoring');
    await page.locator('.step.active .next-step-btn').click();
    await expect(page.locator('#business-name')).toHaveValue("Leo's Music");
  });

  test('Manual setup flow without persona', async ({ page }) => {

    const fs = require('fs');
    const path = require('path');
    await page.route('**/setup.html', async route => {
        const fileContent = fs.readFileSync(path.join(process.cwd(), 'src/ui/tauri/src/ui/setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: fileContent });
    });
    await page.goto('http://mock/setup.html');
    await page.locator('#step-initial .next-step-btn').click();

    await page.getByText('Agency or Studio').click();
    await page.locator('#step-context .next-step-btn').click();

    await page.locator('#business-categories').selectOption('Design');
    await page.locator('.step.active .next-step-btn').click();

    await page.locator('#business-name').fill("Nora Studio");
    await page.locator('.step.active .next-step-btn').click();

    await page.getByTestId('team-operations').click();
    await page.locator('#assistant-tone').selectOption('Professional');
    await page.locator('.step.active .next-step-btn').click();

    await page.locator('#first-offer').fill("Logo Design");
    await page.locator('.step.active .next-step-btn').click();

    await page.locator('#location-input').fill('Portland, OR');
    await page.locator('.step.active .next-step-btn').click();
    await page.locator('#target-audience').fill('Everyone');
    await page.locator('.step.active .next-step-btn').click();
    await page.locator('.step.active .next-step-btn').click();
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
