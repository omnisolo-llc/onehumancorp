import { test, expect } from '@playwright/test';

test.describe('Checkout Success Modal Growth Loop', () => {
  test('should not show success modal on normal load', async ({ page }) => {
    await page.goto('/checkout');
    await expect(page.locator('text=Payment Successful!')).not.toBeVisible();
    await expect(page.locator('.one-tap-referral')).not.toBeVisible();
  });

  test('should show success modal when success=true is passed', async ({ page }) => {
    await page.goto('/checkout?success=true');
    await expect(page.locator('text=Payment Successful!')).toBeVisible();
    await expect(page.locator('.one-tap-referral')).toBeVisible();
  });

  test('should show success modal when session_id is passed', async ({ page }) => {
    await page.goto('/checkout?session_id=cs_test_123');
    await expect(page.locator('text=Payment Successful!')).toBeVisible();
    await expect(page.locator('.one-tap-referral')).toBeVisible();
  });

  test('should interact with OneTapReferral inside modal', async ({ page, context }) => {
    await page.goto('/checkout?success=true');
    await expect(page.locator('text=Payment Successful!')).toBeVisible();

    // Verify OneTapReferral widget is visible
    const oneTapWidget = page.locator('.one-tap-referral');
    await expect(oneTapWidget).toBeVisible();
    await expect(oneTapWidget.locator('text=Refer & Earn $50')).toBeVisible();

    // Verify Copy Link button works
    const copyButton = oneTapWidget.locator('button:has-text("Copy Link")');
    await copyButton.click();
    await expect(oneTapWidget.locator('button:has-text("Copied!")')).toBeVisible();

    // Verify Dashboard redirect works
    const dashboardButton = page.locator('button:has-text("Continue to Dashboard")');
    await dashboardButton.click();
    await expect(page).toHaveURL('/dashboard');
  });

  test('should navigate to affiliate onboarding from powered by link', async ({ page }) => {
    await page.goto('/checkout?success=true');
    const affiliateLink = page.locator('a:has-text("⚡ Powered by OHC")');
    await expect(affiliateLink).toHaveAttribute('href', /^\/onboarding\?ref=.*&source=checkout_affiliate/);
  });
});
