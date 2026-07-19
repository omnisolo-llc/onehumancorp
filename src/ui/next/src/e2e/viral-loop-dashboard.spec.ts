import { test, expect } from '../../../../e2e/fixtures';

test.describe('Viral Loop Dashboard Widget', () => {
    test('dashboard surfaces viral loop metrics correctly and increments on invite generation', async ({ page }) => {
        // Go through the login flow
        await page.goto('/login');
        await page.fill('input[type="text"]', 'test-user');
        await page.fill('input[type="password"]', 'test-pass');
        await Promise.all([page.waitForNavigation(), page.getByRole('button', { name: 'Log In' }).click()]);

        const widgetHeader = page.locator('text=Viral Loop Performance');
        await expect(widgetHeader).toBeVisible({ timeout: 15000 });

        const invitesSentLabel = page.locator('text=Invites Sent');
        await expect(invitesSentLabel).toBeVisible();
        const numberLocator = invitesSentLabel.locator('..').locator('.text-3xl');

        await expect(numberLocator).not.toBeEmpty();
        let initialInvitesSentText = await numberLocator.innerText();

        for (let i = 0; i < 5; i++) {
            if (initialInvitesSentText.trim() !== '') break;
            await page.waitForTimeout(500);
            initialInvitesSentText = await numberLocator.innerText();
        }

        const initialInvitesSent = parseInt(initialInvitesSentText, 10);
        expect(isNaN(initialInvitesSent)).toBe(false);

        await page.goto('/referrals');
        await page.goto('/dashboard');

        const generateInviteBtn = page.locator('button:has-text("Get My Invite Link")');
        await expect(generateInviteBtn).toBeVisible();
        await generateInviteBtn.click();

        const copyInput = page.locator('#dashboard-invite-link');
        await expect(copyInput).toBeVisible({ timeout: 15000 });

        await page.waitForTimeout(1000);

        await page.reload();
        await expect(widgetHeader).toBeVisible();

        const newInvitesSentLabel = page.locator('text=Invites Sent');
        await expect(newInvitesSentLabel).toBeVisible();
        const newNumberLocator = newInvitesSentLabel.locator('..').locator('.text-3xl');

        await expect(newNumberLocator).not.toBeEmpty();

        let newInvitesSentText = await newNumberLocator.innerText();
        for (let i = 0; i < 5; i++) {
            if (newInvitesSentText.trim() !== '') break;
            await page.waitForTimeout(500);
            newInvitesSentText = await newNumberLocator.innerText();
        }

        const newInvitesSent = parseInt(newInvitesSentText, 10);
        expect(newInvitesSent).toBeGreaterThanOrEqual(initialInvitesSent + 1);
    });
});
