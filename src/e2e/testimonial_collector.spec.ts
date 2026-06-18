import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Testimonial Collector Widget', () => {
  test('should load and allow copying the collection link', async ({ browser }) => {
    const page = await adminPage(browser);
    // Navigate to the testimonial collector page
    await page.goto('/testimonial-collector');

    // Verify headers and key structural elements
    await expect(page.locator('text=Testimonial Collector')).toBeVisible();
    await expect(page.locator('text=Gather reviews and earn referrals.')).toBeVisible();

    // Ensure the collection link is generated for the e2e-tenant
    const collectionInput = page.locator('input#collection-link');
    await expect(collectionInput).toBeVisible();
    const inputValue = await collectionInput.inputValue();
    expect(inputValue).toContain('embed/testimonial?tenant=');

    // Verify the "Powered by OHC" watermark link exists and points to the correct growth loop URL
    const poweredByLink = page.locator('a:has-text("Powered by OHC")');
    await expect(poweredByLink).toBeVisible();
    const href = await poweredByLink.getAttribute('href');
    expect(href).toContain('/api/v1/growth/referrals/click?target=/onboarding&ref=');
  });

  test('embed route should allow submission and show viral loop', async ({ browser }) => {
    const page = await adminPage(browser);
    // Navigate to the testimonial embed page
    await page.goto('/embed/testimonial?tenant=TEST_TENANT');

    // Check initial render
    await expect(page.locator('text=Leave a Review')).toBeVisible();

    // Fill the form
    await page.fill('input#name', 'John Doe');
    await page.fill('textarea#review', 'Great service!');

    // Submit form
    await page.click('button[type="submit"]');

    // Check success state
    await expect(page.locator('text=Thank you!')).toBeVisible();
    await expect(page.locator('text=Your review has been submitted successfully.')).toBeVisible();

    // Viral loop should still be present in the embed page
    const poweredByLink = page.locator('a:has-text("Powered by OHC")');
    await expect(poweredByLink).toBeVisible();
    const href = await poweredByLink.getAttribute('href');
    expect(href).toContain('ref=TEST_TENANT');
  });
});
