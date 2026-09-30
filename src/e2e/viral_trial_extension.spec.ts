import { createGrowthOwner } from './growth_owner';
import { test, expect } from './fixtures';

test.describe('Viral Trial Extension Loop', () => {
  test('should display the trial extension page and handle share', async ({ page, baseURL }) => {
    await createGrowthOwner(page, baseURL);
    await page.goto('/trial-extension');

    await expect(page.getByRole('heading', { name: 'Interactive Pro Activation' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Activate Pro Access?' })).toBeVisible();

    // The share button should be present
    const shareButton = page.getByRole('button', { name: 'Share on X to Activate Pro' });
    await expect(shareButton).toBeVisible();
    await expect(shareButton).toBeEnabled();

    const poweredByLink = page.locator('a', { hasText: /OmniSolo/i }).first();
    await expect(poweredByLink).toBeVisible();
    await expect(poweredByLink).toHaveAttribute('href', /.*\/api\/v1\/growth\/referrals\/click\?target=\/onboarding&ref=trial_extension/);

    // We cannot use waitForEvent('popup') because we mock window.open
    await page.evaluate(() => {
      window.open = function() { return null; };
    });

    const claimResponse = page.waitForResponse(response =>
      response.url().endsWith('/api/v1/growth/trial-extension/claim') && response.request().method() === 'POST');
    await shareButton.click();
    expect((await claimResponse).ok()).toBeTruthy();

    await expect(page.getByRole('heading', { name: 'Pro Access Activated' })).toBeVisible();
    await expect(page.getByText('Thank you for sharing. The backend confirmed Pro access for this account.')).toBeVisible();

    const dashboardBtn = page.getByRole('link', { name: /Dashboard/i }).first();
    await expect(dashboardBtn).toBeVisible();
    await dashboardBtn.click();

    await expect(page).toHaveURL(/.*\/dashboard/);
  });
  test('should display the trial extension widget on the Pricing page and handle share', async ({ page, baseURL }) => {
    // Navigate to pricing
    await createGrowthOwner(page, baseURL);
    await page.goto('/pricing');

    // Wait for the Pricing screen to load
    await expect(page.locator('h1:has-text("Pricing Plans")')).toBeVisible();

    // Verify the widget text
    await expect(page.getByText(/Want (Pro Access\?|7 Extra Days of Pro\?)/i)).toBeVisible();
    await expect(page.getByText(/Share on X \(Twitter\) to (request access to|unlock a free week of) advanced features\./i)).toBeVisible();

    // The share button should be present inside the widget
    const shareButton = page.getByRole('button', { name: /Share (to Unlock|on X to Request Pro)/i });
    await expect(shareButton).toBeVisible();
    await expect(shareButton).toBeEnabled();

    // Mock window.open to prevent popup
    await page.evaluate(() => {
      window.open = function() { return null; };
    });

    await shareButton.click();

    // Verify it transitions to success state
    await expect(page.getByText(/(Pro Access Activated|Trial Extended!)/i)).toBeVisible({ timeout: 15000 });
    await expect(page.getByText(/(The backend confirmed Pro access for this account\.|You've unlocked 7 days of Pro for free\.)/i)).toBeVisible();
  });
});
