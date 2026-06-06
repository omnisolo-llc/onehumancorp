import { test, expect } from '@playwright/test';

test.describe('Growth Feature: Social Proof Nudge', () => {
    test('renders widget script correctly and shows the nudge on screen', async ({ page }) => {
        // Use the default user flow to get to dashboard
        await page.goto('/dashboard');

        const socialProofLink = page.locator('text=Social Proof Nudge');
        await expect(socialProofLink).toBeVisible();
        await socialProofLink.click();

        // Wait for page load
        await expect(page.locator('h1', { hasText: 'Social Proof Nudge' })).toBeVisible();

        // Ensure the live preview has the data
        const livePreview = page.locator('.animate-fade-in-up');
        await expect(livePreview).toBeVisible();
        await expect(livePreview).toContainText('Someone');
        await expect(livePreview).toContainText('purchased');

        // Check the code snippet
        const codeSnippet = page.locator('#embed-code');
        await expect(codeSnippet).toBeVisible();

        const snippetText = await codeSnippet.innerText();
        expect(snippetText).toContain('<script src="https://ohc.app/widgets/social-proof.js" async></script>');
        expect(snippetText).toContain('data-product="A product"');
        expect(snippetText).toContain('data-location="Someone"');

        // Create a blank HTML file that renders the embed code using Playwright's setContent
        // We do this to ensure that `social-proof.js` loads properly on a fresh page.
        // We include a script tag manually here for local testing.

        await page.route('https://ohc.app/widgets/social-proof.js', async route => {
            const fs = require('fs');
            const script = fs.readFileSync('public/widgets/social-proof.js');
            await route.fulfill({ body: script, contentType: 'application/javascript' });
        });

        await page.setContent(`
            <!DOCTYPE html>
            <html>
                <head>
                    <title>Test Page</title>
                </head>
                <body>
                    <div id="ohc-social-proof" data-product="A product" data-location="Someone" data-time="just now" data-theme="light" data-branding="true"></div>
                    <script src="https://ohc.app/widgets/social-proof.js" async></script>
                </body>
            </html>
        `);

        // Ensure widget mounts
        const widgetContainer = page.locator('#ohc-social-proof-widget');
        await expect(widgetContainer).toBeVisible();

        // Ensure data rendered
        await expect(widgetContainer).toContainText('Someone');
        await expect(widgetContainer).toContainText('purchased');
        await expect(widgetContainer).toContainText('A product');
        await expect(widgetContainer).toContainText('just now');
        await expect(widgetContainer).toContainText('Powered by OHC');

        // Check close button functionality
        const closeBtn = page.locator('#ohc-social-proof-close');
        await expect(closeBtn).toBeVisible();
        await closeBtn.click();

        // It should animate out (takes 500ms)
        await page.waitForTimeout(600);
        await expect(widgetContainer).toBeHidden();
    });
});
