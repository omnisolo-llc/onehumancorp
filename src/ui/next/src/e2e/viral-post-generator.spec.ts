import { test, expect } from '../../../../e2e/fixtures';
import { createGrowthOwner } from '../../../../e2e/growth_owner';

test.describe('Viral Post Generator Soft Paywall', () => {
    test('should show soft paywall modal when attempting to remove branding', async ({ page, baseURL }) => {
        if (!baseURL) throw new Error('The local browser fixture URL is required');
        const origin = new URL(baseURL);
        expect(['http:', 'https:']).toContain(origin.protocol);
        expect(['localhost', '127.0.0.1', '[::1]']).toContain(origin.hostname);
        const owner = await createGrowthOwner(page, baseURL);
        const planResponse = await page.request.get(new URL('/api/v1/billing/my-plan', origin).href, { headers: {
            'x-ohc-expected-user': owner.userId, 'x-ohc-expected-tenant': owner.tenantId,
        } });
        expect(planResponse.status()).toBe(200);
        const plan = await planResponse.json();
        expect(plan.error ?? null).toBeNull();
        expect([undefined, true]).toContain(plan.success);
        expect(typeof plan.current_plan).toBe('string');
        expect(plan.current_plan.toLowerCase()).toBe('free');

        // Go to the generator page
        await page.goto('/viral-post-generator');

        // Check if the page title is correct
        await expect(page.getByRole('heading', { name: 'Social Post Template' })).toBeVisible();
        await expect(page.getByText(/Create a local text template from your details/)).toBeVisible();

        // Fill in the product and benefit
        await page.fill('input[placeholder="e.g. Signature Coffee Blend"]', 'Super Nova');
        await page.fill('input[placeholder="e.g. a bold start to your morning"]', 'instant social proof');

        // Verify the checkbox is initially unchecked
        const checkbox = page.getByRole('checkbox', { name: /Remove "OmniSolo" branding/i });
        await expect(checkbox).not.toBeChecked();

        // Check the "Remove 'OmniSolo' branding" box
        await checkbox.click();

        // Verify the soft paywall modal opens
        const modalHeading = page.getByRole('heading', { name: 'Upgrade to Pro' });
        await expect(modalHeading).toBeVisible();

        // Verify the modal text
        await expect(page.locator('text=Make the post 100% white-labeled.')).toBeVisible();

        // The paired plan adapter exposes an honest capability check, not a grant.
        const trialRequests: string[] = [];
        page.on('request', request => { if (new URL(request.url()).pathname.includes('trial')) trialRequests.push(request.url()); });
        const pagesBefore = page.context().pages().length;
        await page.getByRole('button', { name: 'Check trial availability', exact: true }).click();
        await expect(page.getByText('Trial activation is unavailable because a durable grant is not verified. Check your current plan or review billing.', { exact: true })).toBeVisible();
        expect(trialRequests).toEqual([]);
        expect(page.context().pages()).toHaveLength(pagesBefore);
        await expect(modalHeading).toBeVisible();

        const closeButton = page.getByRole('button', { name: 'Close paywall', exact: true });
        // Wait for it to be visible first
        await expect(closeButton).toBeVisible();
        await closeButton.click();

        // Wait for modal to disappear
        await expect(modalHeading).not.toBeVisible();

        // After closing the modal without purchasing, the checkbox should ideally be unchecked
        await expect(checkbox).not.toBeChecked();

        // Generate the post with branding
        await page.click('button:has-text("Generate Post")');

        // Check the generated post section
        const generated = page.locator('.whitespace-pre-wrap');
        await expect(generated).toContainText('Just dropped something special!', { timeout: 10000 });
        await expect(generated).toContainText('Super Nova');
        await expect(generated).toContainText('instant social proof');

        // Ensure "OmniSolo" is in the text
        await expect(generated).toContainText('⚡ Powered by OmniSolo');
        await expect(generated).not.toContainText('.cloud.omnisolo.co');
    });
});
