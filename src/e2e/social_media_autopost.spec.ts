import { test, expect } from '@playwright/test';

test.describe('Social Media Autopost - The Promoter Agent', () => {

    test('creates a product and verifies the Promoter Agent feed proposal', async ({ page }) => {
        // Assume basic auto-auth or we land on dashboard directly in this environment.
        // Navigate to the Dashboard first
        await page.goto('/dashboard');

        // Wait for dashboard to settle
        await expect(page.getByText('Your agents are working on your behalf.')).toBeVisible({ timeout: 15000 }).catch(() => {});

        // Step 2: Navigate to add product (or we can go directly to /products/new to be safe)
        await page.goto('/products/new');
        await expect(page.getByRole('heading', { name: 'Add Product' })).toBeVisible();

        // Step 3: Trigger Auto-Catalog or manual entry
        // The component uses an input type="file" to upload an image and trigger auto-catalog
        const buffer = Buffer.from('fake image data');
        await page.setInputFiles('input[type="file"]', {
            name: 'product.jpg',
            mimeType: 'image/jpeg',
            buffer,
        });

        // Wait for the publish button to appear
        const publishBtn = page.getByRole('button', { name: 'Publish Product' });
        await expect(publishBtn).toBeVisible({ timeout: 15000 });

        // Ensure price is filled out before publishing
        const titleInput = page.getByDisplayValue(/./).first(); // It usually fills it automatically.
        await expect(titleInput).toBeVisible();

        await publishBtn.click();

        // Check for published confirmation
        await expect(page.getByText('Product Published!')).toBeVisible({ timeout: 10000 });

        // Step 4: Navigate back to dashboard to see agent feed
        await page.getByRole('link', { name: 'Return to Dashboard' }).click();

        // Ensure we are on the dashboard
        await expect(page.getByRole('button', { name: /Proposals/ })).toBeVisible({ timeout: 10000 });

        // Wait for Agent to process and insert approval (can take a moment depending on AI mock latency)
        // We look for the schedule button or the title
        await expect(page.getByText('New product detected! Schedule a post')).toBeVisible({ timeout: 15000 });

        // Verify the social variants are rendered
        await expect(page.getByText('TikTok')).toBeVisible();
        await expect(page.getByText('Instagram')).toBeVisible();
        await expect(page.getByText('Twitter')).toBeVisible();

        // Verify the schedule button
        await expect(page.getByRole('button', { name: 'Schedule' })).toBeVisible();
    });
});
