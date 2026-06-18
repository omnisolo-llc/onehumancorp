import { test, expect } from '@playwright/test';
import * as path from 'path';
import * as fs from 'fs';

test.describe('Persona-Driven Onboarding E2E', () => {

  test.beforeEach(async ({ page }) => {
    // Intercept standard setup.html load to serve from filesystem for tests
    const tauriUiDir = path.join(process.cwd(), 'src/ui/tauri/src/ui');
    await page.route('**/setup.html', async route => {
      const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
      await route.fulfill({ contentType: 'text/html', body: content });
    });

    await page.route('**/success.html', async route => {
      const content = fs.readFileSync(path.join(tauriUiDir, 'success.html'), 'utf-8');
      await route.fulfill({ contentType: 'text/html', body: content });
    });

    // Mock the state endpoint which the frontend hits
    await page.route('**/api/onboarding/state', async route => {
       await route.fulfill({ status: 200, body: JSON.stringify({}) });
    });

    await page.route('**/api/onboarding/draft', async route => {
       await route.fulfill({ status: 200, body: JSON.stringify({}) });
    });
  });

  test('Maya the Baker persona journey', async ({ page }) => {
    await page.goto('http://mock/setup.html');

    const chatButton = page.locator('button', { hasText: 'Conversational Setup' });
    if(await chatButton.isVisible()) {
        await page.evaluate(() => { (window as any).goToStep('step-context') });
    }

    // Step 1: Work Context & Persona Quick-Start
    await expect(page.getByRole('heading', { name: 'How do you work?' })).toBeVisible();

    // Click "I'm a Baker" persona chip
    await page.getByTestId('persona-baker').evaluate((el) => {
        el.click();
    });

    // Verify radio "Storefront" is selected (implied by persona)
    const storefrontRadio = page.locator('input[value="Storefront"]');
    await expect(storefrontRadio).toBeChecked();

    await page.locator('#step-context .next-step-btn').click();

    // Step 2: Categories
    await expect(page.getByRole('heading', { name: "What's your category?" })).toBeVisible();
    await expect(page.locator('#business-categories')).toHaveValue('Bakery');
    await page.locator('#step-categories').getByRole('button', { name: 'Next' }).click();

    // Step 3: Business Name
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    const nameInput = page.locator('#business-name');
    await expect(nameInput).toHaveValue("Maya's Bakery");
    await page.locator('#step-name').getByRole('button', { name: 'Next' }).click();

    // Step 4: Assistant Setup
    await expect(page.getByRole('heading', { name: "Set up your Assistant" })).toBeVisible();
    await expect(page.locator('#assistant-intro')).toContainText("partner in growing this business");
    const assistantName = page.locator('#assistant-name');
    await expect(assistantName).toHaveValue("Cookie");
    const assistantTone = page.locator('#assistant-tone');
    await expect(assistantTone).toHaveValue("Friendly");
    await page.locator('#step-assistant').getByRole('button', { name: 'Next' }).click();


    // Step 5: Admin Credentials
    await expect(page.getByRole('heading', { name: "Admin Credentials" })).toBeVisible();
    await page.locator('#admin-email').fill('maya@example.com');
    await page.locator('#admin-password').fill('securepassword123');
    await page.locator('#step-admin').getByRole('button', { name: 'Next' }).click();

    // Step 6: First Offer
    await expect(page.getByRole('heading', { name: "Your First Offer" })).toBeVisible();
    const firstOffer = page.locator('#first-offer');
    await expect(firstOffer).toHaveValue("Custom Birthday Cake");
    await page.locator('#step-offer').getByRole('button', { name: 'Next' }).click();

    // Step 7: Domain
    await expect(page.getByRole('heading', { name: "Where will your business live?" })).toBeVisible();
    await page.locator('#domain-name').fill('test-domain');
    await page.locator('#step-domain').getByRole('button', { name: 'Next' }).click();

    // Step 8: Template
    await expect(page.getByRole('heading', { name: "Template Selection" })).toBeVisible();
  });

  test('Carlos the Handyman persona journey', async ({ page }) => {
    await page.goto('http://mock/setup.html');

    const chatButton = page.locator('button', { hasText: 'Conversational Setup' });
    if(await chatButton.isVisible()) {
        await page.evaluate(() => { (window as any).goToStep('step-context') });
    }

    await page.getByTestId('persona-handyman').evaluate((el) => { el.click() });

    await expect(page.locator('input[value="Local Service"]')).toBeChecked();
    await page.locator('#step-context .next-step-btn').click();

    await expect(page.locator('#business-categories')).toHaveValue('Handyman');
    await page.locator('#step-categories').getByRole('button', { name: 'Next' }).click();

    await expect(page.locator('#business-name')).toHaveValue("Carlos Repairs");
    await page.locator('#step-name').getByRole('button', { name: 'Next' }).click();


    await expect(page.locator('#assistant-name')).toHaveValue("Tools");
    await page.locator('#step-assistant').getByRole('button', { name: 'Next' }).click();

    await page.locator('#admin-email').fill('carlos@example.com');
    await page.locator('#admin-password').fill('securepassword123');
    await page.locator('#step-admin').getByRole('button', { name: 'Next' }).click();

    await expect(page.locator('#first-offer')).toHaveValue("Standard Repair Visit");
    await page.locator('#step-offer').getByRole('button', { name: 'Next' }).click();

    // Step 7: Domain
    await expect(page.getByRole('heading', { name: "Where will your business live?" })).toBeVisible();
    await page.locator('#domain-name').fill('test-domain');
    await page.locator('#step-domain').getByRole('button', { name: 'Next' }).click();

    await expect(page.getByRole('heading', { name: "Template Selection" })).toBeVisible();
  });

  test('Priya the Boutique Owner persona journey', async ({ page }) => {
    await page.goto('http://mock/setup.html');

    const chatButton = page.locator('button', { hasText: 'Conversational Setup' });
    if(await chatButton.isVisible()) {
        await page.evaluate(() => { (window as any).goToStep('step-context') });
    }

    await page.getByTestId('persona-boutique').evaluate((el) => { el.click() });
    await page.locator('#step-context .next-step-btn').click();
    await expect(page.locator('#business-categories')).toHaveValue('Boutique');
    await page.locator('#step-categories').getByRole('button', { name: 'Next' }).click();
    await expect(page.locator('#business-name')).toHaveValue("Priya's Boutique");
  });

  test('Leo the Tutor persona journey', async ({ page }) => {
    await page.goto('http://mock/setup.html');

    const chatButton = page.locator('button', { hasText: 'Conversational Setup' });
    if(await chatButton.isVisible()) {
        await page.evaluate(() => { (window as any).goToStep('step-context') });
    }

    await page.getByTestId('persona-tutor').evaluate((el) => { el.click() });
    await page.locator('#step-context .next-step-btn').click();
    await expect(page.locator('#business-categories')).toHaveValue('Tutoring');
    await page.locator('#step-categories').getByRole('button', { name: 'Next' }).click();
    await expect(page.locator('#business-name')).toHaveValue("Leo's Music");
  });

  test('Manual setup flow without persona', async ({ page }) => {
    await page.goto('http://mock/setup.html');

    const chatButton = page.locator('button', { hasText: 'Conversational Setup' });
    if(await chatButton.isVisible()) {
        await page.evaluate(() => { (window as any).goToStep('step-context') });
    }

    await page.locator('.context-card').first().evaluate((el) => {
        el.click();
        el.dispatchEvent(new Event('change', { bubbles: true }));
    });

    await page.locator('#step-context .next-step-btn').click();

    await page.locator('#business-categories').selectOption('Handyman');
    await page.locator('#step-categories').getByRole('button', { name: 'Next' }).click();

    await page.locator('#business-name').fill("Nora Studio");
    await page.locator('#step-name').getByRole('button', { name: 'Next' }).click();

    await page.locator('#assistant-name').fill("Dash");
    await page.locator('#assistant-tone').selectOption('Professional');
    await page.locator('#step-assistant').getByRole('button', { name: 'Next' }).click();

    await page.locator('#admin-email').fill('test@test.com');
    await page.locator('#admin-password').fill('password123');
    await page.locator('#step-admin').getByRole('button', { name: 'Next' }).click();

    await page.locator('#first-offer').fill("Logo Design");
    await page.locator('#step-offer').getByRole('button', { name: 'Next' }).click();

    await page.locator('#domain-name').fill('test-domain');
    await page.locator('#step-domain').getByRole('button', { name: 'Next' }).click();

    await expect(page.getByRole('heading', { name: "Template Selection" })).toBeVisible();
    await page.locator('#template-selection').selectOption('Modern');

    // Finish Setup

    await page.route('**/api/onboarding/start', async route => {
        await route.fulfill({ status: 200, contentType: 'application/json', body: '{}' });
    });
    const finishBtn = page.locator('#finish-btn');
    await finishBtn.click();

    // Verification: Success Page
    await expect(page).toHaveURL(/success.html/);
    await expect(page.getByRole('heading', { name: "You're all set!" })).toBeVisible();
  });
});
