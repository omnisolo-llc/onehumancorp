import { test, expect } from '@playwright/test';

test.describe('Interactive Poll Generator Growth Loop', () => {
    test('should generate an interactive poll embed code and verify the widget', async ({ page }) => {
        // Start at the dashboard
        await page.goto('/dashboard');

        // Ensure "Growth & Virality" section is visible
        await expect(page.locator('h2.app-panel-title', { hasText: 'Growth & Virality' })).toBeVisible();

        // 1. Navigate directly to the generator as the link might not be added yet
        await page.goto('/interactive-poll-generator');

        // Verify the page loaded correctly
        await expect(page.locator('h1')).toContainText('Interactive Poll Generator');

        // 2. Configure the poll
        // Fill in the question
        await page.fill('input[placeholder="e.g. What flavor should we make next?"]', 'Which new feature should we build?');

        // Wait for the preview to update
        await expect(page.locator('h3:has-text("Which new feature should we build?")')).toBeVisible();

        // Update the options
        const optionInputs = page.locator('input[placeholder^="Option"]');
        await optionInputs.nth(0).fill('AI Analytics');
        await optionInputs.nth(1).fill('Mobile App');
        await optionInputs.nth(2).fill('Dark Mode');

        // Check require email
        await page.click('text=Require Email to Vote');

        // Generate the embed code
        await page.click('button:has-text("Generate Embed Code")');

        // 3. Verify the generated embed code
        const codeBlock = page.locator('pre');
        await expect(codeBlock).toBeVisible();
        const embedCode = await codeBlock.textContent();

        // Ensure the iframe points to the correct endpoint
        expect(embedCode).toContain('/api/v1/growth/interactive-poll/embed');
        expect(embedCode).toContain('Which%20new%20feature%20should%20we%20build%3F');

        // 4. Test the actual embed endpoint
        // Extract the iframe src URL from the embed code
        const srcMatch = embedCode?.match(/src="([^"]+)"/);
        expect(srcMatch).toBeTruthy();

        if (srcMatch) {
            const embedUrl = srcMatch[1];

            // Navigate to the embed URL
            await page.goto(embedUrl);

            // Verify the poll rendered correctly
            await expect(page.locator('h3')).toContainText('Which new feature should we build?');
            await expect(page.locator('button:has-text("AI Analytics")')).toBeVisible();
            await expect(page.locator('button:has-text("Mobile App")')).toBeVisible();
            await expect(page.locator('button:has-text("Dark Mode")')).toBeVisible();

            // Verify email input is present (since we checked the box)
            await expect(page.locator('input[type="email"]')).toBeVisible();

            // The vote button should be disabled initially
            const voteBtn = page.locator('button:has-text("Vote Now")');
            await expect(voteBtn).toBeDisabled();

            // Select an option
            await page.click('button:has-text("AI Analytics")');

            // Fill email
            await page.fill('input[type="email"]', 'test@example.com');

            // Button should now be enabled
            await expect(voteBtn).toBeEnabled();

            // Submit vote
            await voteBtn.click();

            // Verify success state
            await expect(page.locator('h3')).toContainText('Thanks for voting!');
        }
    });
});
