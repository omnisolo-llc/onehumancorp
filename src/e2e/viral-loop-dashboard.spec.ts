import { test, expect } from './fixtures';

test.describe('Viral Loop Dashboard Widget', () => {
    test('dashboard surfaces viral loop metrics correctly and increments on invite generation', async ({ page }) => {
        // Go through the login flow
        await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 20000 });

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

        const generateInviteBtn = page.locator('#dashboard-invite-btn');
        const cloudInviteBtn = page.locator('#generate-link-btn');

        if (await generateInviteBtn.isVisible()) {
            await generateInviteBtn.click();
        } else if (await cloudInviteBtn.isVisible()) {
            await cloudInviteBtn.click();
        }

        const copyInput = page.locator('#dashboard-invite-link, #referral-link').first();
        await expect(copyInput).toBeVisible();
        await expect(copyInput).not.toHaveValue('');

        // Give the backend a small amount of time to actually insert the invite
        await page.waitForTimeout(1000);

        await page.reload();
        await expect(widgetHeader).toBeVisible({ timeout: 15000 });

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

        // Ensure that the real backend logic persists and increments the counter
        expect(newInvitesSent).toBeGreaterThanOrEqual(initialInvitesSent + 1);
    });
});
