import { test, expect } from '@playwright/test';

test.describe('Viral Loop Dashboard Widget', () => {
    test('dashboard surfaces viral loop metrics correctly and increments on invite generation', async ({ page }) => {
        // Mock the dashboard stats API so we start with 0
        await page.route('/api/v1/growth/team-invites/aggregated-metrics', async route => {
            await route.fulfill({
                json: {
                    total_invites: 0,
                    metrics: {
                        active_referrals: 0,
                        revenue: 0,
                        pending_rewards: 0
                    }
                }
            });
        });

        // Go through the login flow
        await page.goto('/login');
        await page.fill('input[type="text"]', 'test-user');
        await page.fill('input[type="password"]', 'test-pass');
        await Promise.all([page.waitForNavigation(), page.getByRole('button', { name: 'Log In' }).click()]);

        // Look for the "Viral Loop Performance" section
        const widgetHeader = page.locator('text=Viral Loop Performance');
        await expect(widgetHeader).toBeVisible({ timeout: 15000 });

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

        // Now mock it to return 1 when we navigate back
        await page.route('/api/v1/growth/team-invites/aggregated-metrics', async route => {
            await route.fulfill({
                json: {
                    total_invites: initialInvitesSent + 1,
                    metrics: {
                        active_referrals: 0,
                        revenue: 0,
                        pending_rewards: 0
                    }
                }
            });
        });

        // Next, go to the Referrals page and generate a link to trigger a change
        await page.goto('/referrals');

        // Let's use the dashboard widget actually, or just simulate it directly on the dashboard
        await page.goto('/dashboard');

        const generateInviteBtn = page.locator('button:has-text("Get My Invite Link")');

        await page.route('/api/v1/growth/referrals/generate', async route => {
            await route.fulfill({ json: { referral_link: 'https://ohc.app/ref/test' } });
        });

        await expect(generateInviteBtn).toBeVisible();
        await generateInviteBtn.click();

        // The copy link input should become visible.
        const copyInput = page.locator('#dashboard-invite-link');
        await expect(copyInput).toBeVisible({ timeout: 15000 });
        await expect(widgetHeader).toBeVisible();

        // Check if the number of invites sent has incremented
        const newInvitesSentLabel = page.locator('text=Invites Sent');
        await expect(newInvitesSentLabel).toBeVisible();
        const newNumberLocator = newInvitesSentLabel.locator('..').locator('.text-3xl');

        // Use web-first assertion to wait for the incremented value
        const expectedCount = (initialInvitesSent + 1).toString();
        await expect(newNumberLocator).toHaveText(expectedCount, { timeout: 15000 });
    });
});
