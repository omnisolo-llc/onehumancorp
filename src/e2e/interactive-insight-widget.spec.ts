import { test, expect } from '@playwright/test';

test.describe('Interactive Insight Widget Growth Feature', () => {
  test('should display builder, allow configuration, and show paywall on branding removal attempt without pro', async ({ page }) => {
    // Navigate to the new feature page
    await page.goto('/interactive-insight-widget');

    // Verify title and page header
    await expect(page).toHaveTitle('Insight Widget | OHC');
    await expect(page.getByRole('heading', { name: 'Insight Widget Builder' })).toBeVisible();

    // Verify default metric label and value
    const metricLabelInput = page.getByLabel('Metric Label');
    await expect(metricLabelInput).toHaveValue('Projects Completed');
    const metricValueInput = page.getByLabel('Metric Value');
    await expect(metricValueInput).toHaveValue('150+');

    // Verify Live Preview reflects default values
    const livePreviewLabel = page.getByText('Projects Completed', { exact: true });
    await expect(livePreviewLabel).toBeVisible();
    const livePreviewValue = page.getByText('150+', { exact: true });
    await expect(livePreviewValue).toBeVisible();

    // Verify "Powered by OHC" watermark is visible in preview
    const poweredByLink = page.getByRole('link', { name: '⚡ Powered by OHC' });
    await expect(poweredByLink).toBeVisible();

    // Update metric label and value
    await metricLabelInput.fill('Happy Customers');
    await metricValueInput.fill('99%');

    // Verify Live Preview reflects new values
    const updatedLivePreviewLabel = page.getByText('Happy Customers', { exact: true });
    await expect(updatedLivePreviewLabel).toBeVisible();
    const updatedLivePreviewValue = page.getByText('99%', { exact: true });
    await expect(updatedLivePreviewValue).toBeVisible();

    // Attempt to remove branding without Pro
    const removeBrandingCheckbox = page.getByLabel(/Remove "Powered by OHC" Badge/);
    await removeBrandingCheckbox.click();

    // Verify the soft paywall appears
    const paywallHeading = page.getByRole('heading', { name: 'Upgrade to Remove Branding' });
    await expect(paywallHeading).toBeVisible();

    // Close the paywall
    const closeButton = page.getByRole('button', { name: 'Close paywall' });
    await closeButton.click();
    await expect(paywallHeading).not.toBeVisible();

    // Verify that the checkbox remained unchecked (since they didn't have Pro)
    await expect(removeBrandingCheckbox).not.toBeChecked();
    await expect(poweredByLink).toBeVisible();
  });
});
