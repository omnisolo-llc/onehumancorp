import { test, expect } from '@playwright/test';

test.describe('Documentation Interactive Flows', () => {
    test('renders interactive walkthrough overlay', async ({ page }) => {
        await page.goto('/dashboard?test_walkthrough=true');

        const helpButton = page.locator('#help-widget-container button').first();
        await expect(helpButton).toBeVisible();
        await helpButton.click();

        const tourButton = page.locator('button', { hasText: 'Tour: Accept your first payment' });
        await expect(tourButton).toBeVisible();
        await tourButton.click();

        const walkthroughDialog = page.getByRole('dialog', { name: /walkthrough step/i });
        await expect(walkthroughDialog).toBeVisible();

        const nextOrFinishBtn = walkthroughDialog.locator('button', { hasText: /(Next|Finish)/ });
        await expect(nextOrFinishBtn).toBeVisible();
    });

    test('Help chat opens up and interacts', async ({ page }) => {
        await page.goto('/dashboard?test_chat=true');

        const chatButton = page.locator('button[aria-label="Open help chat"]');
        await expect(chatButton).toBeVisible();
        await chatButton.click();

        const inputField = page.locator('input[placeholder="Ask me anything..."]');
        const sendButton = page.locator('button[aria-label="Send message"]');

        await inputField.fill('How do I add a new product?');
        await sendButton.click();

        const agentReply = page.locator('text=I am your AI Help Agent!');
        await expect(agentReply).toBeVisible({ timeout: 10000 });
    });
});
