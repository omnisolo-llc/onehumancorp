import { test, expect } from '@playwright/test';

test('viral milestones: verify milestone embed endpoint', async ({ page }) => {
  // Navigate directly to the embed endpoint
  await page.goto('/api/v1/growth/milestone/embed?tenant=my-test-tenant&milestone_id=first_sale&theme=light');

  // Verify the milestone content
  await expect(page.locator('text=First Sale!')).toBeVisible();
  await expect(page.locator('text=Congratulations on your first sale!')).toBeVisible();

  // Verify the viral loop link
  const poweredByLink = page.locator('a[href*="/api/v1/growth/referrals/click"]');
  await expect(poweredByLink).toBeVisible();
  await expect(poweredByLink).toContainText('Powered by OHC');

  // Verify the link has the correct tenant tracking parameter
  const href = await poweredByLink.getAttribute('href');
  expect(href).toContain('ref=my-test-tenant');
  expect(href).toContain('source=milestone_embed');
});

test('viral milestones: verify theme parameter', async ({ page }) => {
    // Navigate directly to the embed endpoint with dark theme
    await page.goto('/api/v1/growth/milestone/embed?tenant=my-test-tenant&milestone_id=10th_order&theme=dark');

    // Verify the milestone content
    await expect(page.locator('text=10th Order!')).toBeVisible();

    // Check if background color is dark
    const widget = page.locator('.widget');
    const bg = await widget.evaluate((el) => {
        return window.getComputedStyle(el).backgroundColor;
    });
    // #1f2937 is rgb(31, 41, 55)
    expect(bg).toBe('rgb(31, 41, 55)');
});
