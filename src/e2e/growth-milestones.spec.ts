import { expect } from '@playwright/test';
import { test, adminPage } from './fixtures';

test.describe('Success Milestones UI', () => {
    test('renders the success milestones page and loads a preview card', async ({ page }) => {
        // We know we are logged in from adminPage fixture.
        page = await adminPage(page);
        await page.goto('/milestones.html');

        // Check for the main title
        await expect(page.locator('h1.app-title')).toHaveText(/Success Milestones/);

        // Check for the "Your Achievements" section
        await expect(page.locator('h2.section-title').first()).toHaveText('Your Achievements');

        // Check that at least one milestone rendered
        await expect(page.locator('.milestone-item').first()).toBeVisible();

        // The first milestone is auto-selected, verify the share section is visible
        await expect(page.locator('#share-container')).toBeVisible();
        await expect(page.locator('#empty-state')).not.toBeVisible();

        // Check the card preview image loaded successfully
        const previewImage = page.locator('#card-preview-img');
        await expect(previewImage).toBeVisible();

        // Assert the image source has the expected path
        const src = await previewImage.getAttribute('src');
        expect(src).toContain('/api/v1/growth/milestone/card');

        // Wait for the SVG to render if there is some network latency
        await expect(previewImage).toHaveAttribute('src', /\/api\/v1\/growth\/milestone\/card/);
    });
});
