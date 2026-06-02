import { test, expect } from '@playwright/test';

test.describe('Maya CUJ: End-to-end Dashboard Workflow', () => {
  // Use page directly from Playwright, set localStorage before navigating
  test.beforeEach(async ({ page }) => {
    // Navigate to a blank page to allow setting localStorage
    await page.goto('about:blank');

    // Evaluate the tenant_id in localStorage which the dashboard expects
    await page.evaluate(() => {
      localStorage.setItem('tenant', 'e2e-tenant');
      localStorage.setItem('tenant_id', 'e2e-tenant');
      localStorage.setItem('business_name', 'Maya');
      localStorage.setItem('user_id', 'e2e-admin-user');
    });

    // Navigate to the real dashboard route
    await page.goto('/dashboard');
    await page.waitForLoadState('domcontentloaded');

    // Close the morning briefing and milestone modal if they pop up
    const morningBriefingClose = page.locator('button:has-text("Let\'s Build")').first();
    if (await morningBriefingClose.isVisible({ timeout: 2000 })) {
      await morningBriefingClose.click();
    }
  });

  test('Verify dashboard is properly rendering', async ({ page }) => {
    await expect(page.locator('text=Morning, Maya')).toBeVisible();
    await expect(page.locator('text=e2e-tenant').first()).not.toBeVisible(); // Just verifying the mock text isn't plastered unnecessarily
  });

  test('Verify Embed Storefront modal opens and contains the real tenant_id', async ({ page }) => {
    // Click the Embed Storefront button
    const embedButton = page.locator('button:has-text("Embed Storefront")');
    await expect(embedButton).toBeVisible();
    await embedButton.click();

    // The modal should appear
    await expect(page.locator('h2:has-text("Embed Storefront")')).toBeVisible();

    // Verify the widget HTML snippet has the actual tenant_id 'e2e-tenant'
    const snippetInput = page.locator('textarea[readonly]');
    await expect(snippetInput).toBeVisible();
    await expect(snippetInput).toHaveValue(/tenant=e2e-tenant/);
  });

  test('Verify Referral Modal opens and contains the real tenant_id', async ({ page }) => {
    // Find a link or button that opens the referral modal
    const referralFloatButton = page.locator('div.fixed.bottom-4.right-4 button').first();
    await expect(referralFloatButton).toBeVisible();
    await referralFloatButton.click();

    // The modal should appear
    await expect(page.locator('h2:has-text("Help a Business Grow!")')).toBeVisible();

    // The read-only input should contain the actual tenant_id 'e2e-tenant'
    const linkInput = page.locator('input[readonly]');
    await expect(linkInput).toBeVisible();
    await expect(linkInput).toHaveValue(/ref=e2e-tenant/);
  });

  test('Verify Twitter intent on Milestone completion shares proper tenant link', async ({ page }) => {
    // Ensure milestone banner is visible
    const claimRewardButton = page.locator('button:has-text("Share & Claim Reward")');
    if (await claimRewardButton.isVisible()) {
        const [popup] = await Promise.all([
            page.waitForEvent('popup'),
            claimRewardButton.click()
        ]);

        await popup.waitForLoadState();
        expect(popup.url()).toContain('twitter.com/intent/tweet');
        expect(popup.url()).toContain('ref%3De2e-tenant');
        await popup.close();
    }
  });

  test('Verify Post-Sale share uses proper tenant link', async ({ page }) => {
      // Find the share on Twitter button in the new sale celebration
      const shareTwitterButton = page.locator('a:has-text("X (Twitter)")').first();
      if (await shareTwitterButton.isVisible()) {
          const href = await shareTwitterButton.getAttribute('href');
          expect(href).toContain('ref%3De2e-tenant');
      }
  });

});
