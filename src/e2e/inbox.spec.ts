import { test, expect } from './fixtures';

test.describe('Customer Inbox', () => {
  test('navigates to the inbox and asserts the action feed header', async ({ page }) => {
    await page.goto('/');

    // Navigate from the home page
    const inboxLink = page.locator('a[href="/inbox"]');
    await expect(inboxLink).toBeVisible();
    await inboxLink.click();

    await expect(page.getByRole('heading', { name: 'Customer Inbox' })).toBeVisible();
    await expect(page.locator('text=Action Feed')).toBeVisible();
  });

  test('asserts that there are pending items fetching from the backend', async ({ page }) => {
    await page.goto('/');

    // Navigate from the home page
    const inboxLink = page.locator('a[href="/inbox"]');
    await expect(inboxLink).toBeVisible();
    await inboxLink.click();

    // Check for the specific test data seeded in e2e-seed.sql for ambassador_reply
    await expect(page.locator('text=Do you have vegan options for birthday cakes?')).toBeVisible();
    await expect(page.locator('text=Yes, we have several vegan options for birthday cakes. We would love to help you plan your special day!')).toBeVisible();
    await expect(page.locator('text=Customer Inquiry')).toBeVisible();
    await expect(page.locator('text=HIGH Risk')).toBeVisible();
  });

  test('approves a draft reply and asserts success', async ({ page }) => {
    await page.goto('/');

    // Navigate from the home page
    const inboxLink = page.locator('a[href="/inbox"]');
    await expect(inboxLink).toBeVisible();
    await inboxLink.click();

    await expect(page.locator('text=Do you have vegan options for birthday cakes?')).toBeVisible();

    const approveBtn = page.getByRole('button', { name: 'Approve & Send' }).first();
    await approveBtn.click();

    // The item should disappear from the inbox
    await expect(page.locator('text=Do you have vegan options for birthday cakes?')).not.toBeVisible();
    // Assuming inbox zero state might be visible after all approvals are processed
    // Or at least it's gone from the list. Let's just assert it is not visible.
  });

  test('rejects or edits a draft reply', async ({ page }) => {
    await page.goto('/');

    // Navigate from the home page
    const inboxLink = page.locator('a[href="/inbox"]');
    await expect(inboxLink).toBeVisible();
    await inboxLink.click();

    await expect(page.locator('text=Do you have vegan options for birthday cakes?')).toBeVisible();

    const rejectBtn = page.getByRole('button', { name: 'Edit / Dismiss' }).first();
    await rejectBtn.click();

    // The item should disappear from the inbox after rejection
    await expect(page.locator('text=Do you have vegan options for birthday cakes?')).not.toBeVisible();
  });

  test('checks responsive mobile (375px) fallback behaviors', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/');

    // Navigate from the home page
    const inboxLink = page.locator('a[href="/inbox"]');
    await expect(inboxLink).toBeVisible();
    await inboxLink.click();

    await expect(page.getByRole('heading', { name: 'Customer Inbox' })).toBeVisible();

    // Ensure back button functionality on mobile works as expected
    // Click the back link that wraps an SVG and goes to /dashboard
    await page.locator('a[href="/dashboard"]').first().click();

    // Wait for navigation
    await expect(page.url()).toContain('/dashboard');
  });
});
