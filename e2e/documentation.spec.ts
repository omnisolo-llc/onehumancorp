import { test, expect } from '@playwright/test';

test.describe('Documentation Features CUJ', () => {

  test('Persona: Business Owner uses the Help Center search to find answers', async ({ page }) => {
    // Navigate to Help Center
    await page.goto('/help');

    // Check header
    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();

    // Mock the api response
    await page.route('/api/help', async route => {
      await route.fulfill({ json: [
        { title: "Getting Started", desc: "Learn how to easily set up your store and accept your first payment.", link: "/help/getting-started" },
        { title: "Your AI Helpers", desc: "Learn how to hire AI helpers and give them tasks to do.", link: "/help/ai-agents" }
      ] });
    });

    // Wait for load, search for something
    const searchInput = page.getByPlaceholder(/Search for help articles.../i);
    await searchInput.waitFor();
    await searchInput.fill('AI Helpers');

    // Verify filtered results
    await expect(page.getByText('Learn how to hire AI helpers')).toBeVisible();
    await expect(page.getByText('Learn how to easily set up your store')).not.toBeVisible();
  });

  test('Persona: Business Owner views Tooltips', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');

    // Mock tooltip api response
    await page.route('/api/tooltips', async route => {
      await route.fulfill({ json: { "nav-dashboard-tooltip": "Check your sales, recent orders, and how your store is doing." } });
    });

    // The dashboard contains nav links with tooltips
    const dashboardNav = page.locator('a[href="/dashboard"]').first();
    await dashboardNav.hover();

    // The tooltip should be visible
    await expect(page.getByText('Check your sales, recent orders, and how your store is doing.')).toBeVisible();
  });

  test('Persona: Business Owner starts a Walkthrough from HelpWidget', async ({ page }) => {
    // Mock the window environment var to ensure InteractiveWalkthrough renders
    await page.addInitScript(() => {
        // Intercept process.env check if possible, or just let Playwright handle it
    });

    // We can't easily test Walkthrough rendering if process.env.OHC_E2E === 'true' is hiding it
    // Wait, the InteractiveWalkthrough hides itself if OHC_E2E === 'true' !
    // Let's test if the help widget buttons are there and click them.
    await page.goto('/dashboard');

    // Click help widget floating button
    const helpBtn = page.locator('button', { hasText: '?' }).first();
    // Sometimes it's a bubble with a question mark
    await page.locator('#help-widget-container button').first().click();

    // Check if Tours are available
    await expect(page.getByText('Interactive Tours')).toBeVisible();
    await expect(page.getByText('Tour: Set up your store')).toBeVisible();

    // We can't fully test Walkthrough popping up because of OHC_E2E=true in tests,
    // but we can verify the button navigates to the dashboard (or checkout).
    const tourBtn = page.locator('button', { hasText: 'Tour: Accept your first payment' });
    await tourBtn.click();

    // Verify it redirects
    await expect(page).toHaveURL(/.*\/checkout/);
  });

  test('Persona: Business Owner asks HelpChat a question', async ({ page }) => {
    await page.goto('/dashboard');

    // Open Help Chat widget
    // Help chat is the fixed bottom right button with a chat icon. We can select it by role/class or id if we added one.
    // Let's look for the HelpChat button
    const helpChatBtn = page.locator('button[aria-label="Open help chat"]');
    await helpChatBtn.click();

    // Mock API
    await page.route('/api/chat', async route => {
      await route.fulfill({ json: { text: "To connect Stripe, go to the setup page and follow the secure link." } });
    });

    // Type in chat
    const chatInput = page.getByPlaceholder(/Ask anything.../i);
    await chatInput.fill('How do I connect stripe?');
    await chatInput.press('Enter');

    // Verify response
    await expect(page.getByText('To connect Stripe, go to the setup page and follow the secure link.')).toBeVisible();
  });

  test('Persona: Business Owner views API Docs (Advanced)', async ({ page }) => {
    await page.goto('/api-docs');
    await expect(page.getByText('API Reference for advanced users integrating with OneHumanCorp.')).toBeVisible();
  });

  test('Persona: Business Owner views Changelog', async ({ page }) => {
    await page.goto('/changelog');
    await expect(page.getByRole('heading', { name: 'Release Notes & Changelog' })).toBeVisible();
    await expect(page.getByText('Interactive AI Store Builder:')).toBeVisible();
  });

});
