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
    });
});
