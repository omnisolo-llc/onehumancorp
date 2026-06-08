import { test, expect } from '@playwright/test';

test.describe('Testimonial Widget Generator E2E', () => {
    test('User can configure testimonial and copy embed code', async ({ page }) => {
        await page.goto('/testimonial-widget');

        await expect(page.getByRole('heading', { name: 'Testimonial Widget 🌟' })).toBeVisible();

        await page.fill('input[placeholder="e.g. my-store"]', 'awesome-bakery');
        await page.fill('input[placeholder="e.g. Jane Doe"]', 'Maya The Baker');
        await page.fill('textarea', 'The cakes are absolutely amazing!');
        await page.selectOption('select', '4');

        await page.getByRole('button', { name: 'Dark' }).click();

        await page.getByRole('button', { name: 'Get Widget Code' }).click();

        const modalHeading = page.getByRole('heading', { name: 'Embed Testimonial' });
        await expect(modalHeading).toBeVisible();

        const embedTextarea = page.locator('textarea[readonly]');
        const embedValue = await embedTextarea.inputValue();
        expect(embedValue).toContain('<iframe');
        expect(embedValue).toContain('api/v1/growth/testimonial/embed');
        expect(embedValue).toContain('tenant=awesome-bakery');
        expect(embedValue).toContain('authorName=Maya%20The%20Baker');
        expect(embedValue).toContain('theme=dark');

        await page.getByRole('button', { name: 'Copy Code' }).click();

        await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();

        await page.getByRole('button', { name: 'Close' }).click();
        await expect(modalHeading).not.toBeVisible();
    });
});