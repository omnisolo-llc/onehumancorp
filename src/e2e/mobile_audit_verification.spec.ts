import { test, expect } from '@playwright/test';

test.use({
  viewport: { width: 375, height: 667 },
  userAgent: 'Mozilla/5.0 (iPhone; CPU iPhone OS 14_8 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.1.2 Mobile/15E148 Safari/604.1',
});

test('Mobile Dashboard Audit - Footer and Header Responsiveness', async ({ page }) => {
  // Set up Elena's persona data in localStorage
  await page.goto('http://localhost:3000');
  await page.evaluate(() => {
    localStorage.setItem('business_name', "Elena's Ethos");
    localStorage.setItem('tenant', 'elena-ethos-123');
    localStorage.setItem('has_pro', 'true');
  });

  await page.goto('http://localhost:3000/dashboard');

  // Verify Header Responsiveness
  const header = page.locator('header');
  await expect(header).toBeVisible();

  // Check if header title is visible
  await expect(page.getByText("Elena's Ethos")).toBeVisible();

  // Scroll to the bottom of the dashboard
  await page.evaluate(() => window.scrollTo(0, document.body.scrollHeight));
  await page.waitForTimeout(1000);

  // Take screenshot of the bottom to verify no clipping
  await page.screenshot({ path: 'screenshots/dashboard_bottom_mobile_fixed_375px.png' });

  // Verify Team Activity is visible and not clipped
  const teamActivity = page.locator('section').filter({ hasText: 'Team Activity' });
  await expect(teamActivity).toBeVisible();

  // Check for horizontal overflow
  const hasHorizontalScroll = await page.evaluate(() => {
    return document.documentElement.scrollWidth > document.documentElement.clientWidth;
  });
  expect(hasHorizontalScroll).toBe(false);
});
