import { test, expect } from '@playwright/test';

test.describe('Lead Gen Campaign E2E Flow', () => {
  const tenantId = '11111111-1111-1111-1111-111111111111';

  test.beforeEach(async ({ page }) => {
    // Set up local storage mock for tenant
    await page.goto('/');
    await page.evaluate((tId) => {
      localStorage.setItem('tenant', tId);
      localStorage.setItem('tenant_id', tId);
    }, tenantId);
  });

  test('Business owner creates a lead gen campaign and sees it succeed', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');

    // Click on the lead gen card
    const leadGenLink = page.locator('a[href="/marketing/lead-gen"]');
    await expect(leadGenLink).toBeVisible();
    await leadGenLink.click();

    // Verify we are on the lead gen page
    await expect(page).toHaveURL(/\/marketing\/lead-gen/);
    await expect(page.locator('h1')).toHaveText(/Local Lead Generator/i);

    // Fill the form
    await page.fill('input#budget', '75');
    await page.fill('input#zipCode', '10001');

    // Simulate setting radius slider
    await page.locator('input#radius').fill('25');

    // Click submit
    const submitBtn = page.locator('button', { hasText: /Start Finding Jobs/i });
    await expect(submitBtn).toBeEnabled();
    await submitBtn.click();

    // Wait for the success state
    await expect(page.locator('h3', { hasText: /Campaign Started!/i })).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text=Our Marketing & Advertising agent is now actively seeking leads within 25 miles of 10001')).toBeVisible();

    // Wait for the backend worker to do its thing (simulate by waiting a bit)
    await page.waitForTimeout(11000); // 10s poll interval + 1s

    // Go back to dashboard to check if a new booking / order was created
    await page.locator('a', { hasText: /Return to Dashboard/i }).click();

    // Check dashboard for new order or inbox message (our simulation creates an order and a message)
    await expect(page.locator('text=I saw your ad and booked an appointment! Paid $50 deposit.')).toBeVisible({ timeout: 10000 });
  });
});
