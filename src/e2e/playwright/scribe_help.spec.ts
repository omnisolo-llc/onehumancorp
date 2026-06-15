import { test, expect } from '@playwright/test';

/**
 * Persona: Maya - Home Baker
 * Goal: Maya needs to understand how to use OHC to manage her cake orders.
 * Journey: Maya logs in, explores the dashboard, uses a tooltip to understand a button,
 *          starts a walkthrough, and searches the help center.
 */

test.describe('Scribe: In-App Help & Documentation', () => {
    test.beforeEach(async ({ page }) => {
        // Assume basic login or direct access if local dev server allows
        await page.goto('/dashboard.html');
    });

    test('Maya can use contextual tooltips', async ({ page }) => {
        const dashboardTitle = page.locator('#dashboard-title');
        await dashboardTitle.hover();

        const tooltip = page.locator('.ohc-tooltip');
        await expect(tooltip).toBeVisible();
        // Since we unified it, the ID 'dashboard-title' might not have a tooltip in registry,
        // but 'dashboard-walkthrough-btn' does.

        await page.locator('#dashboard-walkthrough-btn').hover();
        await expect(tooltip).toContainText('Take a tour');
    });

    test('Maya can complete an interactive walkthrough', async ({ page }) => {
        await page.locator('#dashboard-walkthrough-btn').click();

        const bubble = page.locator('.ohc-walkthrough-bubble');
        await expect(bubble).toBeVisible();
        await expect(bubble).toContainText('Welcome');

        await bubble.locator('.wt-next').click();
        await expect(bubble).toContainText('AI Savings');

        await bubble.locator('.wt-next').click();
        await expect(bubble).not.toBeVisible();
    });

    test('Maya can search and view help articles', async ({ page }) => {
        await page.goto('/help.html');

        await page.fill('#search-input', 'Adding Products');
        await expect(page.locator('.card')).toHaveCount(1);
        await expect(page.locator('.card h3')).toContainText('Adding Products');

        await page.locator('.card').first().click();
        await expect(page.url()).toContain('help_article.html');
        await expect(page.locator('h1')).toContainText('Managing My Store');
    });

    test('Maya can use the floating help chat', async ({ page }) => {
        await page.goto('/dashboard.html');

        const helpBtn = page.locator('#ohc-floating-help-btn');
        await helpBtn.click();

        const widget = page.locator('#ohc-floating-help-widget');
        await expect(widget).toBeVisible();

        await widget.locator('.ohc-tab-btn', { hasText: 'Ask AI' }).click();
        await widget.locator('#ohc-chat-input').fill('How do I set up my store?');
        await widget.locator('#ohc-chat-input').press('Enter');

        // Wait for AI response
        await expect(widget.locator('.ohc-msg.agent').nth(1)).toBeVisible({ timeout: 10000 });
        await expect(widget.locator('.ohc-msg.agent').last()).toContainText('Based on our help center');
    });
});
