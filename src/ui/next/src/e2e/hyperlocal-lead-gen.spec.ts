import { test, expect } from '@playwright/test';


test.describe('Hyperlocal Lead Gen CUJ', () => {
  test('Carlos the Handyman sets up a weekly lead gen campaign', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    // 1. Navigate to the dashboard where the Lead Gen card is located.
    // The adminPage fixture logs in and defaults to /dashboard, but ensure we are there.
    await page.goto('/dashboard');

    // 2. Locate the Lead Gen card link and click it to navigate to the new Tauri page
    const leadGenLink = page.locator('a[href="/marketing/lead-gen"]');
    await expect(leadGenLink).toBeVisible();
    await leadGenLink.click();

    // Wait for the new page to load
    await expect(page.locator('h1', { hasText: 'Local Lead Generator' })).toBeVisible();

    // 3. Fill in the budget and zip code.
    const budgetInput = page.locator('input#budget');
    const zipInput = page.locator('input#zipCode');

    await expect(budgetInput).toBeVisible();
    await expect(zipInput).toBeVisible();

    await budgetInput.fill('50');
    await zipInput.fill('90210');

    // 4. Submit the campaign.
    const submitBtn = page.getByRole('button', { name: 'Start Finding Jobs' });
    await submitBtn.click();

    // 5. Verify the UI updates to show the active campaign.
    const successMsg = page.locator('text=Campaign Started! 🚀');
    await expect(successMsg).toBeVisible({ timeout: 10000 });

    const detailsMsg = page.locator('text=Our Marketing & Advertising agent is now actively seeking leads within 10 miles of 90210. We\'ll notify you when a booking is made.');
    await expect(detailsMsg).toBeVisible();
  });
});
