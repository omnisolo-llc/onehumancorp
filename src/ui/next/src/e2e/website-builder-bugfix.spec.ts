import { test, expect } from '@playwright/test';

test.describe('Website Builder Tool (E2E Validation)', () => {
    test('renders the initial step successfully', async ({ page }) => {
        await page.goto('/website-builder');
        await expect(page.locator('h1', { hasText: '10-Minute Setup Wizard' }).or(page.locator('h1', { hasText: 'What kind of business are you building?' }))).toBeVisible();
    });

    test('can enter business type and advance', async ({ page }) => {
        await page.goto('/website-builder');

        await page.getByRole('button', { name: 'Start My Business' }).click();

        await expect(page.getByText('What kind of business are you building?')).toBeVisible();
        await page.getByRole('button', { name: 'Online Store' }).click();

        await expect(page.getByText('Give your business a name')).toBeVisible();
    });

    test('can enter business name', async ({ page }) => {
        await page.goto('/website-builder');

        // Skip first step
        await page.getByRole('button', { name: 'Start My Business' }).click();
        await page.getByRole('button', { name: 'Online Store' }).click();

        // Step 2
        const nameInput = page.getByPlaceholder('What is your business called?');
        await expect(nameInput).toBeVisible();
        await nameInput.fill('Sweet Treats Bakery');

        const nextButton = page.getByRole('button', { name: 'Next' });
        await nextButton.click();

        await expect(page.getByText('What do you sell?')).toBeVisible();
    });

    test('can select selling options', async ({ page }) => {
        await page.goto('/website-builder');

        // Skip to step 3
        await page.getByRole('button', { name: 'Start My Business' }).click();
        await page.getByRole('button', { name: 'Online Store' }).click();
        await page.getByPlaceholder('What is your business called?').fill('Sweet Treats Bakery');
        await page.getByRole('button', { name: 'Next' }).click();

        // Step 3
        const physicalProducts = page.getByText('Physical Products');
        await expect(physicalProducts).toBeVisible();
        await physicalProducts.click();

        await page.getByRole('button', { name: 'Next' }).click();
        await expect(page.getByText('Product details')).toBeVisible();
    });

    test('can skip product addition and reach agent selection', async ({ page }) => {
        await page.goto('/website-builder');

        // Skip to step 4
        await page.getByRole('button', { name: 'Start My Business' }).click();
        await page.getByRole('button', { name: 'Online Store' }).click();
        await page.getByPlaceholder('What is your business called?').fill('Sweet Treats Bakery');
        await page.getByRole('button', { name: 'Next' }).click();
        await page.getByText('Physical Products').click();
        await page.getByRole('button', { name: 'Next' }).click();

        // Step 4: Add product
        await page.getByPlaceholder('What is the name of this product?').fill('Cookies');
        await page.getByPlaceholder('0.00').fill('10.00');
        await page.getByRole('button', { name: 'Next' }).click();

        await expect(page.getByText('How do you want to receive payments?')).toBeVisible();
    });
});
