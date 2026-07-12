import { test, expect } from '../../../../e2e/fixtures';
import { e2eUrl } from './fixtures';

test.describe('Real Estate Onboarding CUJ', () => {
    test('Elena can select Real Estate in the wizard and launch her workspace', async ({ page }) => {
        // Step 1: Start wizard
        await page.goto(e2eUrl('/website-builder'));
        await page.waitForLoadState('networkidle');

        await page.getByRole('button', { name: 'Start My Business' }).click();

        // Step 2: Choose Real Estate
        await expect(page.getByRole('heading', { name: 'What kind of business are you building?' })).toBeVisible();
        await page.getByRole('button', { name: 'Real Estate' }).click();

        // Step 3: Enter name
        await expect(page.getByRole('heading', { name: 'Give your business a name' })).toBeVisible();
        await page.getByPlaceholder('What is your business called?').fill('Elena Rentals');
        await page.getByRole('button', { name: 'Next' }).click();

        // Step 4: Add a listing/product
        await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();

        // Wait for the labels to be present, Real estate default products might not be "Physical Products" but we can check if we skip or add one
        const physicalProductsLabel = page.locator('label').filter({ hasText: 'Physical Products' });
        if (await physicalProductsLabel.isVisible()) {
            await physicalProductsLabel.click();
        }
        await page.getByRole('button', { name: 'Next' }).click();

        // Step 5: Product info
        await expect(page.getByRole('heading', { name: 'Add your first product' })).toBeVisible();
        await page.getByPlaceholder('e.g. Handmade Ceramic Mug').fill('15 Unit Apartment Complex');
        await page.getByPlaceholder('e.g. 24.00').fill('1500');
        await page.getByRole('button', { name: 'Next' }).click();

        // Step 6: Payment method
        await expect(page.getByRole('heading', { name: 'How do you want to get paid?' })).toBeVisible();
        const onlineCardLabel = page.locator('label').filter({ hasText: 'Online Card Payments' });
        await onlineCardLabel.click();
        await page.getByRole('button', { name: 'Next' }).click();

        // Step 7: Create admin account
        await expect(page.getByRole('heading', { name: 'Create your owner account' })).toBeVisible();
        await page.getByPlaceholder('Your Name').fill('Elena Property Manager');
        await page.getByPlaceholder('Email Address').fill('elena_pm@example.com');
        await page.getByPlaceholder('Password').fill('SecurePassword123!');
        await page.getByRole('button', { name: 'Next' }).click();

        // Wait for Launch
        await expect(page.getByRole('button', { name: 'Approve & Publish' })).toBeVisible();
    });

    test('Elena can use the zero-click chat onboarding to create a Real Estate workspace', async ({ page }) => {
        await page.goto(e2eUrl('/onboarding/zero-click'));
        await page.waitForLoadState('networkidle');

        // Check if the chat interface loaded
        await expect(page.getByText("Hi there! I'm your OHC setup assistant. What kind of business do you want to build or manage today?")).toBeVisible();

        // Check if our chip is there
        const chip = page.getByRole('button', { name: 'I manage 15 long-term apartment rentals' });
        await expect(chip).toBeVisible();

        // Click the chip
        await chip.click();

        // Send message
        await page.getByTestId('generate-storefront-btn').click();

        // It should ask about maintenance requests
        await expect(page.getByText(/maintenance requests/i)).toBeVisible({ timeout: 15000 });

        // Elena replies "Yes"
        await page.locator('input[placeholder*="home baker"]').fill('Yes please, I want to handle maintenance requests.');
        await page.getByTestId('generate-storefront-btn').click();

        // It should say "Give me a minute" and then Provisioning overlay should appear
        await expect(page.getByText('Building Your Business...')).toBeVisible({ timeout: 15000 });
    });
});
