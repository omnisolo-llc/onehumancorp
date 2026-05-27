import { test, expect } from '@playwright/test';

test.describe('Yield Management Pricing UI', () => {

  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('user can navigate to Add Service and see auto-pricing toggle', async ({ page }) => {
    await page.goto('/services/new');
    await expect(page.getByRole('heading', { name: 'Add Service' })).toBeVisible();
    await expect(page.getByText('Auto-optimize pricing to maximize sales')).toBeVisible();
  });

  test('user can toggle auto-pricing and see min/max inputs', async ({ page }) => {
    await page.goto('/services/new');

    // Instead of fighting the label and invisible input, use force click on the input itself
    const toggle = page.getByTestId('auto-pricing-toggle');
    await expect(toggle).not.toBeChecked();

    await toggle.click({ force: true });

    await expect(toggle).toBeChecked();

    await expect(page.getByTestId('min-price-input')).toBeVisible();
    await expect(page.getByTestId('max-price-input')).toBeVisible();
  });

  test('user can fill in service details with pricing constraints', async ({ page }) => {
    await page.goto('/services/new');

    await page.getByPlaceholder('e.g. Weekly Music Tutoring').fill('Weekend Handyman Special');
    await page.getByPlaceholder('0.00').first().fill('100');

    const toggle = page.getByTestId('auto-pricing-toggle');
    await toggle.click({ force: true });

    await page.getByTestId('min-price-input').fill('80');
    await page.getByTestId('max-price-input').fill('150');

    await expect(page.getByTestId('min-price-input')).toHaveValue('80');
    await expect(page.getByTestId('max-price-input')).toHaveValue('150');
  });

  test('user can save service with pricing constraints and see redirect', async ({ page }) => {
    await page.goto('/services/new');

    await page.getByPlaceholder('e.g. Weekly Music Tutoring').fill('Yield Test Service');

    const toggle = page.getByTestId('auto-pricing-toggle');
    await toggle.click({ force: true });

    await page.getByTestId('min-price-input').fill('50');
    await page.getByTestId('max-price-input').fill('200');

    await page.getByRole('button', { name: 'Save Service' }).click();

    await expect(page.getByText('Service Saved!')).toBeVisible();
  });

  test('user cannot save without a title even with pricing set', async ({ page }) => {
    await page.goto('/services/new');

    const toggle = page.getByTestId('auto-pricing-toggle');
    await toggle.click({ force: true });

    await page.getByTestId('min-price-input').fill('50');

    await page.getByRole('button', { name: 'Save Service' }).click();

    await expect(page.getByText('Service Saved!')).toBeHidden();
  });

});
