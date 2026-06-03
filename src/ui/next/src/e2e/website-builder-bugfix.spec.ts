import { test, expect } from '@playwright/test';

test.describe('Website Builder Tool (E2E Validation)', () => {
    test('renders the initial step successfully', async ({ page }) => {
        await page.goto('/website-builder');
        await expect(page.locator('h1', { hasText: 'Website Builder' }).or(page.locator('h1', { hasText: 'What kind of business are you building?' })).first()).toBeVisible();
    });

    test('can enter business type and advance', async ({ page }) => {
        await page.goto('/website-builder');

        const typeInput = page.getByPlaceholder('e.g., Coffee Shop, Marketing Agency, Bakery');
        await expect(typeInput).toBeVisible();
        await typeInput.fill('Bakery');

        const nextButton = page.getByRole('button', { name: 'Next' });
        await expect(nextButton).toBeVisible();
        await nextButton.click();

        await expect(page.getByText('What is the name of your business?')).toBeVisible();
    });

    test('can enter business name', async ({ page }) => {
        await page.goto('/website-builder');

        // Skip first step
        const typeInput = page.getByPlaceholder('e.g., Coffee Shop, Marketing Agency, Bakery');
        await expect(typeInput).toBeVisible();
        await typeInput.fill('Bakery');
        await page.getByRole('button', { name: 'Next' }).click();

        // Step 2
        const nameInput = page.getByPlaceholder('Enter your business name');
        await expect(nameInput).toBeVisible();
        await nameInput.fill('Sweet Treats Bakery');

        const nextButton = page.getByRole('button', { name: 'Next' });
        await nextButton.click();

        await expect(page.getByText('What will you be selling?')).toBeVisible();
    });

    test('can select selling options', async ({ page }) => {
        await page.goto('/website-builder');

        // Skip to step 3
        await page.getByPlaceholder('e.g., Coffee Shop, Marketing Agency, Bakery').fill('Bakery');
        await page.getByRole('button', { name: 'Next' }).click();
        await page.getByPlaceholder('Enter your business name').fill('Sweet Treats');
        await page.getByRole('button', { name: 'Next' }).click();

        // Step 3
        const physicalProducts = page.getByText('Physical Products');
        await expect(physicalProducts).toBeVisible();
        await physicalProducts.click();

        await page.getByRole('button', { name: 'Next' }).click();
        await expect(page.getByText('Add your first product (optional)')).toBeVisible();
    });

    test('can skip product addition and reach agent selection', async ({ page }) => {
        await page.goto('/website-builder');

        // Skip to step 4
        await page.getByPlaceholder('e.g., Coffee Shop, Marketing Agency, Bakery').fill('Bakery');
        await page.getByRole('button', { name: 'Next' }).click();
        await page.getByPlaceholder('Enter your business name').fill('Sweet Treats');
        await page.getByRole('button', { name: 'Next' }).click();
        await page.getByText('Physical Products').click();
        await page.getByRole('button', { name: 'Next' }).click();

        // Step 4: Skip product
        const skipButton = page.getByRole('button', { name: 'Skip for now' });
        await expect(skipButton).toBeVisible();
        await skipButton.click();

        await expect(page.getByText('Pick your AI Agents')).toBeVisible();
    });
});
