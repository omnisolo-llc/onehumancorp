import { test, expect } from '@playwright/test';

test('viral wrapped: verify year in review widget is dynamically rendered and shareable', async ({ page }) => {
  // Set up local storage to simulate being a tenant
  await page.goto('/');
  await page.evaluate(() => {
    localStorage.setItem('tenant', 'e2e_tenant_id');
  });

  // Load the dashboard - real data will flow from backend
  await page.goto('/dashboard');

  // Verify the widget renders
  const widget = page.getByTestId('wrapped-widget');
  await expect(widget).toBeVisible({ timeout: 15000 });

  // Verify the contents of the widget (we don't mock backend so we only check elements exist)
  await expect(page.getByText('Your Year in Review 🎉')).toBeVisible();
  await expect(page.getByText('Total Sales')).toBeVisible();
  await expect(page.getByText('Orders')).toBeVisible();
  await expect(page.getByText('New Customers')).toBeVisible();

  // Verify Share link structure
  const shareBtn = page.getByTestId('wrapped-share-btn');
  await expect(shareBtn).toBeVisible();

  const twitterBtn = page.getByTestId('wrapped-twitter-btn');
  await expect(twitterBtn).toBeVisible();

  // The copy functionality invokes clipboard API which is restricted in some CI environments,
  // so we check that the Twitter button contains the URL parameter we expect.
  const href = await twitterBtn.getAttribute('href');
  expect(href).toContain('twitter.com/intent/tweet');
  expect(href).toContain('wrapped_share'); // source parameter from our constructed referral link
});
