import { test, expect } from '@playwright/test';

test.describe('Smart Pricing Audit', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/smart-pricing');
  });

  test('should display correctly with default values', async ({ page }) => {
    await expect(page.locator('h1').first()).toHaveText('Smart Pricing');
    await expect(page.getByText('Let AI automatically adjust your prices')).toBeVisible();
    await expect(page.getByText('Configuration')).not.toBeVisible();
  });

  test('should enable smart pricing and show configuration', async ({ page }) => {
    const enableToggle = page.getByTestId('enable-smart-pricing-toggle');
    await enableToggle.click();
    await expect(page.getByText('Configuration')).toBeVisible();
  });

  test('should toggle specific options like perishables and surge pricing', async ({ page }) => {
    const enableToggle = page.getByTestId('enable-smart-pricing-toggle');
    await enableToggle.click();
    await expect(page.getByText('Configuration')).toBeVisible();

    const perishablesToggle = page.getByTestId('discount-perishables-toggle');
    await perishablesToggle.click();

    const surgeToggle = page.getByTestId('surge-pricing-toggle');
    await surgeToggle.click();

    await expect(perishablesToggle).toHaveClass(/bg-\[#0066FF\]/);
    await expect(surgeToggle).toHaveClass(/bg-\[#0066FF\]/);
  });

  test('should adjust maximum price bounds using slider', async ({ page }) => {
    const enableToggle = page.getByTestId('enable-smart-pricing-toggle');
    await enableToggle.click();
    await expect(page.getByText('Configuration')).toBeVisible();

    const slider = page.getByTestId('price-bounds-slider');
    await slider.fill('40');

    await expect(slider).toHaveValue('40');
    await expect(page.getByText('40%')).toBeVisible();
  });

  test('should not make any mock api network requests to simulate-smart-pricing', async ({ page }) => {
    const requestFailed: string[] = [];
    page.on('request', request => {
      if (request.url().includes('simulate-smart-pricing')) {
        requestFailed.push(request.url());
      }
    });

    const enableToggle = page.getByTestId('enable-smart-pricing-toggle');
    await enableToggle.click();
    await expect(page.getByText('Configuration')).toBeVisible();

    // wait briefly to ensure no request is fired in the background
    await page.waitForTimeout(500);

    expect(requestFailed).toHaveLength(0);
  });
});
