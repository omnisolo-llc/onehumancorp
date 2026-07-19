import { expect } from '@playwright/test';
import { test } from './fixtures';

test.describe('Viral NPS Feedback Generator Loop E2E', () => {
    test('should allow member to customize NPS feedback widget and verify viral branding', async ({ memberPage }) => {
        // Navigate to Dashboard
        await memberPage.goto('/dashboard.html');
        let content = await memberPage.content();
        if (!content.includes('OneHumanCorp')) {
            await memberPage.goto('/tauri_out/dashboard.html');
            content = await memberPage.content();
        }
        if (!content.includes('OneHumanCorp')) {
            await memberPage.goto('/ui/dashboard.html');
            content = await memberPage.content();
        }
        if (!content.includes('OneHumanCorp')) {
            await memberPage.goto('/dashboard');
        }

        // Click NPS Feedback Generator
        await memberPage.click('#nps-feedback-link');

        // Wait for NPS Feedback page
        await expect(memberPage.locator('h1', { hasText: 'NPS Feedback Generator' })).toBeVisible();

        // Edit product name
        const productInput = memberPage.locator('#productName');
        await productInput.fill('Awesome Playwright Tests');

        // Select 'dark' theme
        const themeSelect = memberPage.locator('#widgetTheme');
        await themeSelect.selectOption('dark');

        // Verify preview updates
        const previewProductName = memberPage.locator('#previewProductName');
        await expect(previewProductName).toHaveText('Awesome Playwright Tests');

        const previewContainer = memberPage.locator('#widget-preview-container');
        await expect(previewContainer).toHaveClass(/preview-dark/);

        // Verify Powered by link exists and is visible in preview
        const previewPoweredBy = memberPage.locator('#previewBranding');
        await expect(previewPoweredBy).toBeVisible();
        await expect(previewPoweredBy).toHaveText('⚡ Powered by OHC');

        // Ensure the referral parameter is present
        await expect(previewPoweredBy).toHaveAttribute('href', /\/api\/v1\/growth\/referrals\/click\?target=\/setup.html&ref=/);

        // Check the generated embed code
        const codeOutput = memberPage.locator('#codeOutput');
        let generatedHtml = await codeOutput.textContent();
        expect(generatedHtml).toContain('Awesome Playwright Tests');
        expect(generatedHtml).toContain('⚡ Powered by OHC');

        // Try to remove branding without pro
        // First ensure user is not pro
        await memberPage.evaluate(() => { localStorage.setItem('has_pro', 'false'); window.dispatchEvent(new Event('storage')); });
        const removeBrandingToggle = memberPage.locator('label', { hasText: 'Remove branding' });
        await removeBrandingToggle.click();

        // Verify paywall modal shows
        const paywallModal = memberPage.locator('#paywallModal');
        await expect(paywallModal).toHaveClass(/active/);
        await expect(paywallModal).toBeVisible();
        await expect(memberPage.locator('h3', { hasText: 'Pro Feature' })).toBeVisible();

        // Dismiss modal
        await memberPage.click('#keepBrandingBtn');
        await expect(paywallModal).not.toHaveClass(/active/);

        // Now test as a pro user
        await memberPage.evaluate(() => { localStorage.setItem('has_pro', 'true'); window.dispatchEvent(new Event('storage')); });

        // Click the toggle again
        await removeBrandingToggle.click();

        // Modal should not appear
        await expect(paywallModal).not.toHaveClass(/active/);

        // Verify Powered by link is hidden in preview
        const brandingContainer = memberPage.locator('#previewBrandingContainer');
        await expect(brandingContainer).toHaveCSS('display', 'none');

        // Verify generated embed code no longer has the watermark
        generatedHtml = await codeOutput.textContent();
        expect(generatedHtml).not.toContain('⚡ Powered by OHC');
    });

    test('should allow owner to create an embeddable widget', async ({ page }) => {
        // Navigate to dashboard
        await page.goto('/dashboard.html');
        let content = await page.content();
        if (!content.includes('OneHumanCorp')) {
            await page.goto('/tauri_out/dashboard.html');
            content = await page.content();
        }
        if (!content.includes('OneHumanCorp')) {
            await page.goto('/ui/dashboard.html');
            content = await page.content();
        }
        if (!content.includes('OneHumanCorp')) {
            await page.goto('/dashboard');
        }

        // Click the NPS Feedback link
        await page.click('#nps-feedback-link');

        // Wait for page
        await expect(page.locator('h1', { hasText: 'NPS Feedback Generator' })).toBeVisible();

        // Verify preview works
        const previewProductName = page.locator('#previewProductName');
        await expect(previewProductName).toBeVisible();

        // Set input
        await page.fill('#productName', 'Test Admin Product');

        // Check if copy button works
        await page.click('#copyBtn');
        await expect(page.locator('#copyBtn')).toHaveText('Copied!');
    });

    test('should show correct default content on load', async ({ memberPage }) => {
        await memberPage.goto('/nps-feedback-generator.html');
        let content = await memberPage.content();
        if (!content.includes('OneHumanCorp')) {
            await memberPage.goto('/tauri_out/nps-feedback-generator.html');
            content = await memberPage.content();
        }
        if (!content.includes('OneHumanCorp')) {
            await memberPage.goto('/ui/nps-feedback-generator.html');
        }

        await expect(memberPage.locator('h1', { hasText: 'NPS Feedback Generator' })).toBeVisible();

        // Verify defaults
        await expect(memberPage.locator('#productName')).toHaveValue('Our Service');
        await expect(memberPage.locator('#widgetTheme')).toHaveValue('light');

        // Preview defaults
        await expect(memberPage.locator('#previewProductName')).toHaveText('Our Service');
        await expect(memberPage.locator('#widget-preview-container')).toHaveClass(/preview-light/);
    });

    test('should toggle theme and update HTML accordingly', async ({ page, adminUser, loginAs }) => {
        await loginAs(page, adminUser);
        await page.goto('/nps-feedback-generator.html');
        let content = await page.content();
        if (!content.includes('OneHumanCorp')) {
            await page.goto('/tauri_out/nps-feedback-generator.html');
            content = await page.content();
        }
        if (!content.includes('OneHumanCorp')) {
            await page.goto('/ui/nps-feedback-generator.html');
        }

        await expect(page.locator('h1', { hasText: 'NPS Feedback Generator' })).toBeVisible();

        // Check light mode HTML
        let htmlOutput = await page.locator('#codeOutput').textContent();
        expect(htmlOutput).toContain('background-color: #ffffff');
        expect(htmlOutput).toContain('color: #111827');

        // Switch to dark mode
        await page.selectOption('#widgetTheme', 'dark');

        // Check dark mode HTML
        htmlOutput = await page.locator('#codeOutput').textContent();
        expect(htmlOutput).toContain('background-color: #1f2937');
        expect(htmlOutput).toContain('color: #f9fafb');
    });

    test('should persist Pro setting after dismissing modal', async ({ memberPage }) => {
        await memberPage.goto('/nps-feedback-generator.html');
        let content = await memberPage.content();
        if (!content.includes('OneHumanCorp')) {
            await memberPage.goto('/tauri_out/nps-feedback-generator.html');
            content = await memberPage.content();
        }
        if (!content.includes('OneHumanCorp')) {
            await memberPage.goto('/ui/nps-feedback-generator.html');
        }

        await expect(memberPage.locator('h1', { hasText: 'NPS Feedback Generator' })).toBeVisible();

        await memberPage.evaluate(() => { localStorage.setItem('has_pro', 'false'); window.dispatchEvent(new Event('storage')); });

        const toggle = memberPage.locator('label', { hasText: 'Remove branding' });
        await toggle.click();

        await expect(memberPage.locator('#paywallModal')).toHaveClass(/active/);

        // Click upgrade button (which dismisses modal)
        await memberPage.click('#upgradeBtn');
        await expect(memberPage.locator('#paywallModal')).not.toHaveClass(/active/);

        // Verify checkbox was reset
        await expect(memberPage.locator('#removeBranding')).not.toBeChecked();
    });
});
