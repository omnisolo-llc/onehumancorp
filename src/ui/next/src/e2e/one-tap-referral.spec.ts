import { test, expect } from '@playwright/test';

test.describe('OneTapReferral Growth Loop', () => {
    test('OneTapReferral component is rendered and functional', async ({ page }) => {
        // Go to a page where OneTapReferral is rendered
        await page.goto('http://localhost:3000/dashboard');

        // We can test the presence of the component directly
        const referralLink = page.locator('text=Refer & Earn $50');
        await expect(referralLink).toBeVisible();

        const copyButton = page.locator('button', { hasText: 'Copy Link' });
        await expect(copyButton).toBeVisible();
    });

    test('OneTap email invite form submits successfully on the referrals page', async ({ page }) => {
        await page.goto('http://localhost:3000/referrals');

        // Check if the "Invite via Email" heading is visible
        await expect(page.locator('h4', { hasText: 'Invite via Email' })).toBeVisible();

        // Fill in the email
        const emailInput = page.getByPlaceholder('friend@example.com');
        await expect(emailInput).toBeVisible();
        await emailInput.fill('test@example.com');

        // Click the send button
        const sendButton = page.getByRole('button', { name: 'Send Invite' });
        await expect(sendButton).toBeEnabled();
        await sendButton.click();

        // Verify success message appears
        await expect(page.locator('p', { hasText: 'Invite sent successfully!' })).toBeVisible();
    });
});
