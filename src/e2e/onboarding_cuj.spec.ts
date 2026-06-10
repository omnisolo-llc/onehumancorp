import { test, expect } from '@playwright/test';

test.describe('Onboarding CUJ Tests', () => {

  test.beforeEach(async ({ page }) => {
    // 1. Mock state check to assume not onboarded
    await page.route('/api/onboarding/state', async route => {
      await route.fulfill({
        status: 200,
        json: { wizardState: { step: 0 } },
      });
    });

    // 2. Mock the intake API logic to simulate an AI response
    await page.route('/api/onboarding/intake', async route => {
      await route.fulfill({
        status: 200,
        json: {
          business_name: "Maya's Test Cakes",
          business_type: "Bakery",
          initial_products: [{ name: "Vegan Cake", price: "45.00" }],
          location: "Austin, TX"
        },
      });
    });

    // 3. Mock the start API to simulate DB provisioning
    await page.route('/api/onboarding/start', async route => {
      await route.fulfill({
        status: 200,
        json: {
          organization_id: "test_tenant_id",
          message: "Business started successfully."
        },
      });
    });

    // 4. Mock the launch route
    await page.route('/api/onboarding/launch', async route => {
      await route.fulfill({ status: 200, json: {} });
    });

    await page.goto('/onboarding');
  });

  // Test 1: Persona navigates from home, starts onboarding
  test('New User Completes Zero-Click Onboarding and Starts Business', async ({ page }) => {
    // 1. Assert we are on the new zero-click setup screen
    await expect(page.getByText('What do you do?')).toBeVisible();

    // 2. Persona fills the single prompt text area
    const bioInput = page.getByPlaceholder(/I'm a plumber in Miami.../i);
    await bioInput.fill('I am Maya, I run a local bakery making custom vegan cakes in Austin, TX.');

    // 3. Persona clicks "Generate My Business"
    const generateBtn = page.getByRole('button', { name: 'Generate My Business' });
    await expect(generateBtn).not.toBeDisabled();
    await generateBtn.click();

    // 4. Loading state assertions
    await expect(page.getByText('Agents are building...')).toBeVisible();

    // 5. Success state assertions
    await expect(page.getByText("You're Live!")).toBeVisible();
    await expect(page.getByText("Business started successfully.")).toBeVisible();

    // 6. Assert correct subdomain is generated
    await expect(page.getByText("mayas-test-cakes.ohc.app")).toBeVisible();

    // 7. Verify localStorage token
    const tenantId = await page.evaluate(() => window.localStorage.getItem('tenant_id'));
    expect(tenantId).toBe('test_tenant_id');

    // 8. Assert links are present
    const assistantLink = page.getByRole('link', { name: /Publish & Share Link/i });
    await expect(assistantLink).toBeVisible();
    await expect(assistantLink).toHaveAttribute('href', '/assistant');

    const previewLink = page.getByRole('link', { name: /Preview Storefront/i });
    await expect(previewLink).toBeVisible();
    await expect(previewLink).toHaveAttribute('href', '/builder');
  });
});
