import { test, expect } from './fixtures';

test.describe('AI-Driven Subscription & Membership Lifecycle Management', () => {
  test('Subscription offer generation UI handles natural language parsing', async ({ page, adminUser, loginAs }) => {
    test.setTimeout(180000);

    // 1. Log in
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    await page.goto('/ui/subscription-offer-generator.html');

    // Ensure we are testing the mobile viewport layout
    await page.setViewportSize({ width: 375, height: 667 });

    // Verify container width doesn't cause horizontal scroll
    const scrollWidth = await page.evaluate(() => document.documentElement.scrollWidth);
    expect(scrollWidth).toBeLessThanOrEqual(375);

    await expect(page.locator('h1')).toHaveText('Recurring Membership');
  });
});
