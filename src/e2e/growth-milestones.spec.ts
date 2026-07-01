import { expect } from '@playwright/test';
import { test } from './fixtures';

test.describe('Success Milestones UI', () => {
    test('renders the success milestones page and loads a preview card', async ({ adminPage }) => {
        // We know we are logged in from adminPage fixture.
        await adminPage.goto('/milestones.html');

        // Check for the main title
        await expect(adminPage.locator('h1.app-title')).toHaveText(/Success Milestones/);

        // Check for the "Your Achievements" section
        await expect(adminPage.locator('h2.section-title').first()).toHaveText('Your Achievements');

        // Check that at least one milestone rendered
        await expect(adminPage.locator('.milestone-item').first()).toBeVisible();

        // The first milestone is auto-selected, verify the share section is visible
        await expect(adminPage.locator('#share-container')).toBeVisible();
        await expect(adminPage.locator('#empty-state')).not.toBeVisible();

        // Check the card preview image loaded successfully
        const previewImage = adminPage.locator('#card-preview-img');
        await expect(previewImage).toBeVisible();

        // Assert the image source has the expected path
        const src = await previewImage.getAttribute('src');
        expect(src).toContain('/api/v1/growth/milestone/card');

        // Wait for the SVG to render if there is some network latency
        await expect(previewImage).toHaveAttribute('src', /\/api\/v1\/growth\/milestone\/card/);
    });
});
