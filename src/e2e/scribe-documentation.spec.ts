import { test, expect } from './fixtures';

test.describe('Documentation UI Components', () => {

    test('Help Widget API fetches tooltips successfully', async ({ page, unlimitedAdminUser, loginAs }) => {
        // Log in to ensure valid tenant Context
        await loginAs(page, unlimitedAdminUser);

        // Wait for network requests to be processed
        await page.goto('/dashboard');
        await page.waitForLoadState('networkidle');

        // Check if help tooltips exist in the window namespace via eval
        const tooltips = await page.evaluate(() => window['OHC_TOOLTIPS']);
        expect(tooltips).toBeDefined();
    });

    test('Help Center page renders properly and displays articles', async ({ page, loginAs, unlimitedAdminUser }) => {
        await loginAs(page, unlimitedAdminUser);
        await page.goto('/help.html');

        await expect(page.locator('h1', { hasText: 'In-App Help Center' }).first()).toBeVisible({ timeout: 15000 });
    });

    test('Interactive tour via walkthrough works', async ({ page, loginAs, unlimitedAdminUser }) => {
        await loginAs(page, unlimitedAdminUser);
        await page.goto('/dashboard.html');

        const walkBtn = page.locator('#dashboard-walkthrough-btn');
        await expect(walkBtn).toBeVisible({ timeout: 15000 });
        await walkBtn.click();

        const overlay = page.locator('.ohc-walkthrough-overlay');
        await expect(overlay).toBeVisible({ timeout: 15000 });

        const bubble = page.locator('.ohc-walkthrough-bubble');
        await expect(bubble).toBeVisible({ timeout: 15000 });
        await expect(bubble).toContainText('Welcome');

        const closeBtn = page.locator('.ohc-walkthrough-close');
        await closeBtn.click();
        await expect(overlay).not.toBeVisible();
    });

    test('Help tooltip works on hover', async ({ page, loginAs, unlimitedAdminUser }) => {
        await loginAs(page, unlimitedAdminUser);
        await page.goto('/dashboard.html');

        const helpNavBtn = page.locator('#help-center-nav-btn');
        await expect(helpNavBtn).toBeVisible({ timeout: 15000 });
        await helpNavBtn.hover();

        const tooltip = page.locator('.ohc-tooltip').first();
        // Give it some time to fetch API and render
        await expect(tooltip).toBeVisible({ timeout: 15000 });
    });

    test('Ask AI functionality in floating help widget', async ({ page, loginAs, unlimitedAdminUser }) => {
        await loginAs(page, unlimitedAdminUser);
        await page.goto('/help.html');

        const chatBtn = page.locator('#ohc-floating-help-btn');
        await expect(chatBtn).toBeVisible({ timeout: 15000 });
        await chatBtn.click();

        const chatWidget = page.locator('#ohc-floating-help-widget');
        await expect(chatWidget).toBeVisible({ timeout: 15000 });

        const chatTab = page.locator('.ohc-help-tab[data-target="tab-chat"]');
        await chatTab.click();

        const chatInput = page.locator('#ohc-help-chat-input');
        await expect(chatInput).toBeVisible({ timeout: 15000 });
        await chatInput.fill('How do I reset my password?');

        const sendBtn = page.locator('#ohc-help-chat-send');
        await sendBtn.click();

        const messages = page.locator('#ohc-help-chat-messages');
        await expect(messages).toContainText('How do I reset my password?');
    });

    test('Help search functionality works', async ({ page, loginAs, unlimitedAdminUser }) => {
        await loginAs(page, unlimitedAdminUser);
        await page.goto('/help.html');

        const searchInput = page.locator('#search-input');
        await expect(searchInput).toBeVisible({ timeout: 15000 });
        await searchInput.fill('Welcome to One Human Corp');

        // Wait for search debouncing/results
        await page.waitForTimeout(1000);

        const searchResults = page.locator('#results');
        await expect(searchResults).toBeVisible({ timeout: 15000 });
    });

});
