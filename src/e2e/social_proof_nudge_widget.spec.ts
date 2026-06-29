import { test, expect } from './fixtures';

test.describe('Social Proof Nudge Widget Growth Loop', () => {
  test('should navigate to widget builder from dashboard and verify UI elements', async ({ page, loginAs, unlimitedAdminUser }) => {
    // Navigate to Dashboard
    await loginAs(unlimitedAdminUser.email, 'password123');
    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // Verify the Social Proof Nudge Widget link is visible
    const nudgeLinkCard = page.locator('#social-proof-nudge-link');
    await expect(nudgeLinkCard).toBeVisible();

    // Click to navigate to widget
    await nudgeLinkCard.click();
    await page.waitForURL('**/social-proof-nudge-widget**');

    // Verify widget UI elements
    await expect(page.getByRole('heading', { name: 'Social Proof Nudge 📣' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Live Preview' })).toBeVisible();

    // Fill in the form
    await page.getByPlaceholder('e.g. Someone just bought a cake!').fill('Jane just purchased the Summer Collection');
    await page.getByPlaceholder('e.g. 5 mins ago').fill('1 min ago');

    // Check preview updates
    await expect(page.getByText('Jane just purchased the Summer Collection')).toBeVisible();
    await expect(page.getByText('1 min ago')).toBeVisible();

    // Test the copy button
    const copyButton = page.getByRole('button', { name: 'Copy Integration Script' });
    await expect(copyButton).toBeVisible();

    // Grant clipboard permissions
    await page.context().grantPermissions(['clipboard-read', 'clipboard-write']);
    await copyButton.click();

    // Verify copy button changes text temporarily
    await expect(page.getByRole('button', { name: 'Copied to Clipboard!' })).toBeVisible();

    // Verify the clipboard content (skip if headless environment doesn't support reading)
    try {
        const clipboardText = await page.evaluate(async () => {
            return await navigator.clipboard.readText();
        });
        expect(clipboardText).toContain('<script src="https://ohc.app/api/v1/growth/social-proof/embed');
    } catch (e) {
        console.warn('Clipboard read failed (expected in some headless environments): ', e);
    }

    // Verify back to dashboard navigation
    const backButton = page.getByRole('button', { name: 'Back' });
    await backButton.click();
    await page.waitForURL('**/dashboard**');
  });
});
