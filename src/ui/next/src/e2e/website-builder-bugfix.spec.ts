import { test, expect } from '@playwright/test';

test.describe('Website Builder Tool (E2E Validation)', () => {
    test('renders the initial step successfully', async ({ page }) => {
        await page.goto('/website-builder');
        await expect(page.locator('h1', { hasText: 'Website Builder' }).or(page.locator('h1', { hasText: '10-Minute Setup Wizard' })).first()).toBeVisible();
    });

    test('can enter business type and advance', async ({ page }) => {
        await page.goto('/website-builder');

        await page.getByRole('button', { name: 'Start My Business' }).click();

        await expect(page.getByRole('heading', { name: 'What kind of business are you building?' })).toBeVisible();
        await page.getByRole('button', { name: 'Online Store' }).click();

        await expect(page.getByRole('heading', { name: 'Give your business a name' })).toBeVisible();
    });

    test('can enter business name with validation', async ({ page }) => {
        await page.goto('/website-builder');

        // Skip first step
        await page.getByRole('button', { name: 'Start My Business' }).click();
        await page.getByRole('button', { name: 'Online Store' }).click();

        // Step 2
        const nameInput = page.getByPlaceholder('What is your business called?');
        await expect(nameInput).toBeVisible();

        // Test validation
        await nameInput.fill('A');
        const nextButton = page.getByRole('button', { name: 'Next' });
        await nextButton.click();

        await expect(page.getByText('Business Name must be at least 3 characters.')).toBeVisible();

        // Proceed with valid name
        await nameInput.fill('Sweet Treats Bakery');
        await nextButton.click();

        await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();
    });

    test('can select selling options', async ({ page }) => {
        await page.goto('/website-builder');

        // Skip to step 3
        await page.getByRole('button', { name: 'Start My Business' }).click();
        await page.getByRole('button', { name: 'Online Store' }).click();
        await page.getByPlaceholder('What is your business called?').fill('Sweet Treats');
        await page.getByRole('button', { name: 'Next' }).click();

        // Step 3
        const physicalProducts = page.getByText('Physical Products');
        await expect(physicalProducts).toBeVisible();
        await physicalProducts.click();

        await page.getByRole('button', { name: 'Next' }).click();
        await expect(page.getByRole('heading', { name: 'Add your first product (optional)' })).toBeVisible();
    });

    test('can skip product addition and reach payment selection', async ({ page }) => {
        await page.goto('/website-builder');

        // Skip to step 4
        await page.getByRole('button', { name: 'Start My Business' }).click();
        await page.getByRole('button', { name: 'Online Store' }).click();
        await page.getByPlaceholder('What is your business called?').fill('Sweet Treats');
        await page.getByRole('button', { name: 'Next' }).click();
        await page.getByText('Physical Products').click();
        await page.getByRole('button', { name: 'Next' }).click();

        // Step 4: Skip product
        const skipButton = page.getByRole('button', { name: 'Skip for now' });
        await expect(skipButton).toBeVisible();
        await skipButton.click();

        await expect(page.getByRole('heading', { name: 'How do you want to receive payments?' })).toBeVisible();
    });
});
