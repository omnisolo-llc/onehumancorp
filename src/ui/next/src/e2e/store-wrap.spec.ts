import { test, expect } from '@playwright/test';

test.describe('Store Wrap Flow', () => {
  test.beforeEach(async ({ page }) => {
    await page.route('/api/v1/dashboard/metrics', async route => {
      const json = { total_sales: 15000, active_customers: 250 };
      await route.fulfill({ json });
    });
  });

  test('displays store wrap metrics and navigation works', async ({ page }) => {
    await page.goto('http://localhost:3000/store-wrap');
    await expect(page.getByRole('heading', { name: 'Store Wrap-Up 🎁' })).toBeVisible();

    // Verify first slide
    await expect(page.getByText('Your Year in Review')).toBeVisible();
    await expect(page.getByText("Let's see how much you've grown")).toBeVisible();

    // Click right to go to next slide
    await page.locator('main > div:nth-child(3)').click(); // The right 2/3 navigation overlay

    // Verify second slide
    await expect(page.getByText('250')).toBeVisible();
    await expect(page.getByText('Happy Customers')).toBeVisible();

    // Click right to go to next slide
    await page.locator('main > div:nth-child(3)').click();

    // Verify third slide
    await expect(page.getByText('$15,000')).toBeVisible();
    await expect(page.getByText('Total Revenue')).toBeVisible();

    // Click right to go to final slide
    await page.locator('main > div:nth-child(3)').click();

    // Verify final slide
    await expect(page.getByText('Share Your Success')).toBeVisible();

    // Verify share links exist
    await expect(page.getByRole('button', { name: 'Copy Invite Link' })).toBeVisible();
    await expect(page.locator('a', { hasText: 'Post to X' })).toBeVisible();
    await expect(page.locator('a', { hasText: 'WhatsApp' })).toBeVisible();

    // Verify glassmorphism properties are applied
    const card = page.locator('text=Share Your Success').locator('..');
    await expect(card).toHaveCSS('backdrop-filter', /blur\(20px\)/);
    await expect(card).toHaveCSS('background-color', 'rgba(255, 255, 255, 0.05)');
  });
});
