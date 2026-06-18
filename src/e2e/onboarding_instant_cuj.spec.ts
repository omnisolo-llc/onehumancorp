import { test, expect } from '@playwright/test';
import * as path from 'path';
import * as fs from 'fs';

test.describe('Instant Setup CUJ', () => {

  test.beforeEach(async ({ page }) => {
    // Intercept standard setup.html load to serve from filesystem for tests
    const tauriUiDir = path.join(process.cwd(), 'src/ui/tauri/src/ui');
    await page.route('**/setup.html', async route => {
      const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
      await route.fulfill({ contentType: 'text/html', body: content });
    });

    // Mock tooltips call
    await page.route('**/api/tooltips', async route => {
      await route.fulfill({ status: 200, body: JSON.stringify({}) });
    });

    // Mock the state endpoint which the frontend hits
    await page.route('**/api/onboarding/state', async route => {
       await route.fulfill({ status: 200, body: JSON.stringify({}) });
    });

    // Set a known viewport for mobile tests
    await page.setViewportSize({ width: 375, height: 812 });
  });

  test('Persona: Maya (Home Baker) completes the Zero-Click Instant Onboarding', async ({ page }) => {

    // Mock the backend intake response
    let intakeCalled = false;
    await page.route('**/api/onboarding/intake', async route => {
      intakeCalled = true;
      const postData = JSON.parse(route.request().postData() || '{}');
      expect(postData.description).toContain('I make custom vegan cakes in Austin');
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          business_name: "Maya's Vegan Cakes",
          business_type: "Bakery",
          categories: ["food", "cakes"],
          initial_products: [
            { name: "Custom Vegan Cake", price: "45.00" }
          ],
          location: "Austin, TX",
          target_audience: "Vegans and cake lovers"
        })
      });
    });

    // Mock the final start endpoint that actually provisions the tenant
    let onboardingStarted = false;
    await page.route('**/api/onboarding/start', async route => {
      onboardingStarted = true;
      const postData = JSON.parse(route.request().postData() || '{}');

      // Verify payload was correctly formed from the intake_data
      expect(postData.company_name).toBe("Maya's Vegan Cakes");
      expect(postData.first_product_name).toBe("Custom Vegan Cake");

      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          success: true,
          organization_id: 'tenant-maya-123'
        })
      });
    });

    // Mock the success page redirection target
    await page.route('**/success.html', async route => {
      await route.fulfill({ status: 200, contentType: 'text/html', body: `
        <h1>Dashboard</h1>
        <section aria-label="Unified Agent Feed">
          <div id="triage-list">
             <div class="triage-card">Your store is ready. Review and Publish.</div>
             <button data-testid="approve-proposal">Approve & Go Live</button>
          </div>
        </section>
      ` });
    });

    await page.goto('http://mock/setup.html');

    // Verify Initial Screen
    await expect(page.getByRole('heading', { name: '10-Minute Setup Wizard' })).toBeVisible();

    // 1. Click "Instant Build"
    await page.getByRole('button', { name: 'Instant Build' }).click();

    // 2. Verify we are in the instant step
    await expect(page.getByRole('heading', { name: 'Tell us about your business' })).toBeVisible();

    // 3. Fill in the description
    const instantInput = page.locator('#instant-bio');
    await expect(instantInput).toBeVisible();
    await instantInput.fill('I make custom vegan cakes in Austin. I need a website and a way to take bookings.');

    const generateBtn = page.getByTestId('generate-storefront-btn');
    await expect(generateBtn).toBeEnabled();

    // 4. Click generate
    await generateBtn.click();

    // 5. Verify loading texts (animation progress)
    await expect(page.locator('#generate-storefront-btn')).toHaveText('Building Your Business...');

    // 6. Verify it navigated to success.html
    await page.waitForURL('**/success.html', { timeout: 15000 });
    expect(intakeCalled).toBe(true);
    expect(onboardingStarted).toBe(true);

    // 7. Verify Agent Feed Presentation
    await expect(page.locator('section[aria-label="Unified Agent Feed"]')).toBeVisible();
    await expect(page.getByText('Your store is ready. Review and Publish.')).toBeVisible();

    // 8. Review & Action
    const approveBtn = page.getByTestId('approve-proposal');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();
  });
});
