import { test, expect } from './fixtures';

test.describe('Walkthrough and Tooltips features', () => {
  test('Help center walkthrough tour buttons are visible', async ({ page, adminUser, loginAs }) => {
    // Navigate using the admin credentials implicitly logged in by global setup, or just go directly
    await loginAs(page, adminUser);
    await page.goto('/dashboard');

    // Wait for network idle to ensure tooltips load
    await page.waitForLoadState('networkidle');

    // In our Next.js UI, the walkthroughs are triggered from the Help modal or ? button
    const helpBtn = page.getByRole('button', { name: 'Need help? Click here to access our Help Center, Ask AI, Video Tutorials, and Release Notes.' });
    await expect(helpBtn).toBeVisible({ timeout: 15000 });
    await helpBtn.click();

    // Test that one of the tour buttons is present in the help center dropdown
    const tourBtn = page.getByRole('button', { name: 'Tour: Accept your first payment' });
    await expect(tourBtn).toBeVisible();
  });

  test('Tooltips are injected into the page', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    // Hover over the total sales tooltip target
    const target = page.locator('#total-sales-tooltip');
    await expect(target).toBeVisible({ timeout: 15000 });
    await target.hover();

    // Check that a tooltip with text appeared
    const tooltipText = page.getByText('Total revenue generated from database orders.');
    await expect(tooltipText).toBeVisible();
  });

  test('Help Center elements are visible', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto('/help');

    // Verify title
    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible({ timeout: 15000 });

    // Verify search
    const search = page.getByPlaceholder('Search for help articles and videos...');
    await expect(search).toBeVisible();

    // The chat widget should also be there inside the help tab content
    const chatBtn = page.getByRole('button', { name: 'Ask AI Support Agent' });
    // Note: It's only visible when search has no results, let's verify that flow.
    await search.fill('somerandomstringthatwontmatch');
    await expect(chatBtn).toBeVisible();
  });
});
