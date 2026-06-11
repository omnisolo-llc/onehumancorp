import { test, expect } from './fixtures';

test.describe('Viral Interactive Widget Embed', () => {
    test('renders embed builder, copies code, and verify public embed route and referral loop', async ({ page, adminUser, loginAs }) => {
        // Step 1: Login
        await loginAs(page, adminUser);

        // Step 2: Navigate to Embed Builder directly or through Dashboard
        await page.goto('/dashboard');

        // Wait for the Dashboard to load and find the link
        const widgetLink = page.locator('a', { hasText: 'Interactive Embed' });
        await expect(widgetLink).toBeVisible();
        await widgetLink.click();

        // Step 3: Verify Embed Builder Page
        await page.waitForURL('**/embed-builder');
        await expect(page.locator('text=Interactive Embed Builder')).toBeVisible();

        // Check config options
        await expect(page.locator('label:has-text("Workspace ID")')).toBeVisible();
        await expect(page.locator('button:has-text("booking")')).toBeVisible();
        await expect(page.locator('button:has-text("quote")')).toBeVisible();

        // Check Live Preview iframe
        const iframe = page.locator('iframe[title="Preview"]');
        await expect(iframe).toBeVisible();

        // Ensure "Powered by OHC" appears outside the iframe in the preview container
        await expect(page.locator('a:has-text("⚡ Powered by OHC")').first()).toBeVisible();

        // Step 4: Validate Embed Code Generation & Copy
        const embedCodePre = page.locator('pre code');
        const generatedHtml = await embedCodePre.textContent();
        expect(generatedHtml).toContain('<iframe');
        expect(generatedHtml).toContain('⚡ Powered by OHC');

        // Test copy button
        await page.locator('button:has-text("Copy Code")').click();
        await expect(page.locator('button:has-text("Copied!")')).toBeVisible();

        // Step 5: Test the Public Embed Route directly
        // The iframe src looks like: /embed/widget?tenant_id=...&type=intake&theme=light
        await page.goto('/embed/widget?tenant_id=test-tenant-123&type=booking&theme=light');

        // Wait for Widget to load
        await expect(page.locator('text=Book an Appointment')).toBeVisible();

        // Check form elements
        await expect(page.locator('label:has-text("Name")')).toBeVisible();
        await expect(page.locator('label:has-text("Email")')).toBeVisible();
        await expect(page.locator('label:has-text("Preferred Date")')).toBeVisible();

        // Submit form
        await page.locator('input[type="text"]').fill('Jane Doe');
        await page.locator('input[type="email"]').fill('jane@example.com');
        await page.locator('input[type="date"]').fill('2025-10-10');
        await page.locator('textarea').fill('Test booking from playwright');
        await page.locator('button[type="submit"]').click();

        // Verify Success State
        await expect(page.locator('text=Success!')).toBeVisible();

        // Step 6: Verify the public referral loop footer in the generated code
        expect(generatedHtml).toContain('/api/v1/growth/referrals/click?target=/onboarding&ref=');
    });
});
