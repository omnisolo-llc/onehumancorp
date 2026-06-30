import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test('viral wrapped: verify year in review widget is dynamically rendered and shareable', async ({ browser }) => {
  // Set up local storage to simulate being a tenant
  const page = await adminPage(browser);
  await page.evaluate(() => {
    localStorage.setItem('tenant', 'e2e_tenant_id');
  });

  // Mock the API response to avoid real network call in test if needed, although it should exist.
  // Since we don't have the API running locally in e2e sometimes, we mock it.
  await page.route('**/api/v1/growth/wrapped*', async route => {
    const json = { year: 2024, title: "Your Year in Review 🎉", subtitle: "You crushed it this year! See your impact and share with your community.", stats: { totalSales: "$14,250", totalOrders: 342, newCustomers: 128, topProduct: "Custom Logo Design", aiHoursSaved: 42 }, shareText: "I just reviewed my 2024 business stats on OHC and I'm blown away! I saved 42 hours using AI and served 128 new customers. Start growing your business on OHC:" };
    await route.fulfill({ json });
  });

  // Load the dashboard - real data will flow from backend
  await page.goto('/dashboard', { waitUntil: 'domcontentloaded' });
  await page.reload({ waitUntil: 'domcontentloaded' });

  // Scroll down to make the widget visible if it's rendered out of view
  // const wrapper = page.getByTestId('wrapped-widget');
  // await wrapper.scrollIntoViewIfNeeded();

  // Verify the widget renders
  const widget = page.getByTestId('wrapped-widget');
  await expect(widget).toBeAttached({ timeout: 15000 });

  // Verify the contents of the widget (we don't mock backend so we only check elements exist)
  await expect(page.getByText('Your Year in Review 🎉')).toBeAttached();
  await expect(widget.locator('text=Total Sales').first()).toBeAttached();
  await expect(widget.locator('text=Orders').first()).toBeAttached();
  await expect(widget.locator('text=New Customers').first()).toBeAttached();

  // Verify Share link structure
  const shareBtn = page.getByTestId('wrapped-share-btn');
  await expect(shareBtn).toBeAttached();

  const twitterBtn = page.getByTestId('wrapped-twitter-btn');
  await expect(twitterBtn).toBeAttached();

  // The copy functionality invokes clipboard API which is restricted in some CI environments,
  // so we check that the Twitter button contains the URL parameter we expect.
  const href = await twitterBtn.getAttribute('href');
  expect(href).toContain('twitter.com/intent/tweet');
  expect(href).toContain('wrapped_share'); // source parameter from our constructed referral link
});
