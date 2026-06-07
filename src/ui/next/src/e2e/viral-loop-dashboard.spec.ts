import { test, expect } from '@playwright/test';

test.describe('Viral Loop Dashboard Widget', () => {
    test('dashboard surfaces viral loop metrics correctly and increments on invite generation', async ({ page }) => {
        // Go through the login flow
        await page.goto('/login');
        await page.fill('input[type="text"]', 'test-user');
        await page.fill('input[type="password"]', 'test-pass');
        await page.click('button[type="submit"]');

        // Look for the "Viral Loop Performance" section
        const widgetHeader = page.locator('text=Viral Loop Performance');
        await expect(widgetHeader).toBeVisible({ timeout: 15000 });

        // Check initial state of the new stats
        const activeReferralsLabel = page.locator('text=Active Referrals');
        await expect(activeReferralsLabel).toBeVisible();
        const activeReferralsLocator = activeReferralsLabel.locator('..').locator('.text-3xl');
        // Let's not strictly require 0 as db might be dirty from other tests, but let's grab it
        await expect(activeReferralsLocator).not.toBeEmpty();

        let initialActiveReferralsText = await activeReferralsLocator.innerText();
        for (let i = 0; i < 5; i++) {
            if (initialActiveReferralsText.trim() !== '') break;
            await page.waitForTimeout(500);
            initialActiveReferralsText = await activeReferralsLocator.innerText();
        }
        const initialActiveReferrals = parseInt(initialActiveReferralsText, 10);
        expect(isNaN(initialActiveReferrals)).toBe(false);

        // Get the initial number of Invites Sent
        const invitesSentLabel = page.locator('text=Invites Sent');
        await expect(invitesSentLabel).toBeVisible();
        const numberLocator = invitesSentLabel.locator('..').locator('.text-3xl');

        // Wait for it to not be empty (if it's a number)
        await expect(numberLocator).not.toBeEmpty();
        let initialInvitesSentText = await numberLocator.innerText();

        // Sometimes innerText might be initially empty before hydrated, retry a bit
        for (let i = 0; i < 5; i++) {
            if (initialInvitesSentText.trim() !== '') break;
            await page.waitForTimeout(500);
            initialInvitesSentText = await numberLocator.innerText();
        }

        const initialInvitesSent = parseInt(initialInvitesSentText, 10);
        expect(isNaN(initialInvitesSent)).toBe(false);

        // Simulate creating a referral and converting it to see if Active Referrals updates
        const generateRes = await page.request.post('/api/v1/growth/referrals/generate', {});
        expect(generateRes.ok()).toBeTruthy();
        const generateJson = await generateRes.json();
        const refId = generateJson.referral_link.split('/').pop();

        const convertRes = await page.request.post('/api/v1/growth/referrals/convert', {
            data: { id: refId }
        });
        expect(convertRes.ok()).toBeTruthy();

        // Next, go to the Team page and generate an invite to trigger a change
        await page.goto('/team');
        const generateInviteBtn = page.locator('button:has-text("Invite to Cloud Team")');
        await expect(generateInviteBtn).toBeVisible();
        await generateInviteBtn.click();

        // The copy link input should become visible
        const copyInput = page.locator('#cloud-bridge-invite-link');
        await expect(copyInput).toBeVisible();

        // Go back to the dashboard and ensure the widget still renders
        await page.goto('/dashboard');
        await expect(widgetHeader).toBeVisible();

        // Check if the number of invites sent has incremented
        const newInvitesSentLabel = page.locator('text=Invites Sent');
        await expect(newInvitesSentLabel).toBeVisible();
        const newNumberLocator = newInvitesSentLabel.locator('..').locator('.text-3xl');

        // Use web-first assertion to wait for the incremented value
        const expectedCount = (initialInvitesSent + 1).toString();
        await expect(newNumberLocator).toHaveText(expectedCount, { timeout: 10000 });

        // Check if the number of active referrals has incremented
        const expectedActiveReferrals = (initialActiveReferrals + 1).toString();
        const newActiveReferralsLocator = page.locator('text=Active Referrals').locator('..').locator('.text-3xl');
        await expect(newActiveReferralsLocator).toHaveText(expectedActiveReferrals, { timeout: 10000 });

        // Check Revenue and Rewards correctly map
        // Each conversion = $50 revenue, $10 rewards
        const revenueLabel = page.locator('text=Revenue from Referrals');
        await expect(revenueLabel).toBeVisible();
        const expectedRevenue = "$" + ((initialActiveReferrals + 1) * 50).toFixed(2);
        const revenueLocator = revenueLabel.locator('..').locator('.text-3xl');
        await expect(revenueLocator).toHaveText(expectedRevenue, { timeout: 10000 });

        const pendingRewardsLabel = page.locator('text=Pending Rewards');
        await expect(pendingRewardsLabel).toBeVisible();
        const expectedRewards = "$" + ((initialActiveReferrals + 1) * 10).toFixed(2);
        const rewardsLocator = pendingRewardsLabel.locator('..').locator('.text-3xl');
        await expect(rewardsLocator).toHaveText(expectedRewards, { timeout: 10000 });
    });
});
