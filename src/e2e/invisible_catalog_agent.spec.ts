import { test, expect } from './fixtures';

test.describe('Invisible Catalog Agent', () => {
    test('CUJ: Zero-touch storefront onboarding via video scan', async ({ page }) => {
        // Since E2E user must navigate like a real business owner

        await page.goto('/');

        // Login as an existing user
        await page.getByPlaceholder('Email or Username').fill('maya@example.com');
        await page.locator('input[type="password"]').fill('password123');
        await page.getByRole('button', { name: 'Login' }).click();

        // Ensure we hit the dashboard
        await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 10000 });

        // Navigate to the invisible catalog video scan page
        await page.locator('a#nav-catalog-scan').click();

        // Ensure catalog scan screen is visible
        await expect(page.getByRole('heading', { name: 'Invisible Catalog Agent' })).toBeVisible();

        // Start scan
        await page.locator('input#catalog-video-url').fill('https://example.com/demo.mp4');
        await page.locator('button#btn-start-scan').click();

        // Wait for processing to complete and drafts to appear
        await expect(page.locator('#scan-status-text')).toHaveText('COMPLETED', { timeout: 15000 });

        // Verify drafts are rendered
        await expect(page.locator('.draft-item').first()).toBeVisible();

        // We have two elements we want to review
        const firstDraft = page.locator('.draft-item').nth(0);
        const secondDraft = page.locator('.draft-item').nth(1);

        // Approve the first item
        const firstDraftName = await firstDraft.locator('.draft-name').textContent();
        await firstDraft.locator('.btn-approve').click();

        // Discard the second item
        await secondDraft.locator('.btn-discard').click();

        // The list should update (reload via poll in JS)
        // Check that the items we approved/discarded no longer appear in the UI
        await expect(page.locator(`.draft-name:has-text("${firstDraftName}")`)).not.toBeVisible();
    });
});
