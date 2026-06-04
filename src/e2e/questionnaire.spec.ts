import { test, expect } from '@playwright/test';
import { test as baseTest } from './fixtures';

baseTest.describe('Autonomous Client Intake Questionnaire', () => {
  baseTest('Merchant creates service and form, customer books and submits form', async ({ page }) => {
    // Navigate to the Dashboard
    await page.goto('/');
    await expect(page.locator('h1:has-text("Dashboard")').first()).toBeVisible();

    // 1. Merchant Action: Open "Add Product" form
    await page.click('text="Add Product"');
    await expect(page.locator('h1:has-text("Add to Catalog")')).toBeVisible();

    // Select "Service" radio
    await page.locator('input[value="service"]').click();

    // Fill in Service details
    const serviceName = 'Custom Flooring Install';
    await page.fill('#item-name', serviceName);
    await page.fill('#item-price', '1200.00');
    await page.fill('#item-duration', '60');
    await page.fill('#item-desc', 'A customized consultation and installation for high-quality flooring.');

    // Save and capture the new product ID
    let newProductId: string | null = null;
    page.on('response', async response => {
      if (response.url().includes('/api/v1/catalog/product') && response.request().method() === 'POST') {
        try {
          const body = await response.json();
          if (body.product_id) {
            newProductId = body.product_id;
          }
        } catch (e) {}
      }
    });

    let dialogsSeen = 0;
    page.on('dialog', async dialog => {
      dialogsSeen++;
      // First dialog is "Saved successfully", second is "Would you like me to create an intake form"
      if (dialog.message().includes('Would you like me to create an intake form')) {
        await dialog.accept(); // Yes, create intake
      } else {
        await dialog.accept(); // Dismiss "Saved"
      }
    });

    await page.click('button:has-text("Save Item")');

    // Wait for the questionnaire builder screen to be visible
    await expect(page.locator('h1:has-text("Intake for Custom Flooring Install")')).toBeVisible();

    // Give it a moment to ensure newProductId is captured
    await page.waitForTimeout(500);
    expect(newProductId).not.toBeNull();

    // Check default questions are rendered
    const questionInputs = page.locator('.q-text');
    await expect(questionInputs).toHaveCount(3);

    // Save the Questionnaire
    await page.click('button:has-text("Save Questionnaire")');
    await expect(page.locator('h1:has-text("Dashboard")').first()).toBeVisible();

    // 3. Customer Action: Tries to checkout for this service
    // To simulate customer hitting "Buy", we manually trigger `loadIntakeForm` with the ID
    await page.evaluate((pid) => {
        // @ts-ignore
        loadIntakeForm(pid);
    }, newProductId);

    // Intake form appears
    await expect(page.locator('h1:has-text("Service Consultation")')).toBeVisible();
    await expect(page.locator('#intake-title')).toHaveText('Intake for Custom Flooring Install');

    // Fill out the generated questions
    // Question 1: Dimensions
    const inputs = page.locator('.intake-answer');
    await inputs.nth(0).fill('15x20 ft');

    // Question 2: Material
    await inputs.nth(1).selectOption('Hardwood');

    // Submit form
    await page.click('button:has-text("Submit & Request Quote")');

    // Should see success dialog and return to dashboard
    await expect(page.locator('h1:has-text("Dashboard")').first()).toBeVisible();
  });
});
