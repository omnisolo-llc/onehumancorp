import { test, expect } from './fixtures';

test.describe('Viral Milestone Sharing', () => {
    test('should allow a business owner to share their milestone', async ({ page }) => {
        // Go to dashboard
        await page.goto('/dashboard');

        // Wait for milestone section to be visible
        await expect(page.getByRole('heading', { name: 'Viral Milestone Sharing' })).toBeVisible();
        await expect(page.getByText('You hit 100 Orders!')).toBeVisible();

        // Click the Share button
        await page.getByRole('button', { name: 'Share Milestone & Get Free Month' }).click();

        // Verify the modal appears
        await expect(page.getByRole('heading', { name: 'Share Your Success' })).toBeVisible();

        // Verify the drafting state or directly the text area content
        const textarea = page.locator('textarea');
        await expect(textarea).toBeVisible();

        // Verify that the generated message is present in the text area. We will wait for the text to contain 'hit a new milestone'.
        await expect(textarea).toHaveValue(/hit a new milestone on OneHumanCorp!/);

        // Check that 'Share to X' link is visible and contains correct text
        const shareLink = page.getByRole('link', { name: 'Share to X' });
        await expect(shareLink).toBeVisible();
        const href = await shareLink.getAttribute('href');
        expect(href).toContain('twitter.com/intent/tweet');

        // Check that 'Copy Text' button is available
        const copyButton = page.getByRole('button', { name: 'Copy Text' });
        await expect(copyButton).toBeVisible();
    });
});
