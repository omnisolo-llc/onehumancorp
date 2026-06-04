import { test, expect } from '@playwright/test';

test.describe.skip('Website Builder Tool (E2E Validation)', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/website-builder');
        await page.evaluate(() => localStorage.clear());
        await page.goto('/website-builder');
    });

    test('renders the initial step successfully', async ({ page }) => {
        await expect(page.locator('h1', { hasText: '10-Minute Setup Wizard' }).first()).toBeVisible();
    });

    test('can enter business type and advance', async ({ page }) => {
        await page.getByRole('button', { name: 'Start My Business' }).click();

        await expect(page.getByText('What kind of business is this?')).toBeVisible();
        await page.getByRole('button', { name: 'Online Store' }).click();

        await expect(page.getByText('What is the name of your business?')).toBeVisible();
    });

    test('can enter business name', async ({ page }) => {
        await page.getByRole('button', { name: 'Start My Business' }).click();
        await page.getByRole('button', { name: 'Online Store' }).click();

        const nameInput = page.getByPlaceholder('Enter your business name');
        await expect(nameInput).toBeVisible();
        await nameInput.fill('Sweet Treats Bakery');

        const nextButton = page.getByRole('button', { name: 'Next' });
        await nextButton.click();

        await expect(page.getByText('What will you be selling?')).toBeVisible();
    });

    test('can select selling options', async ({ page }) => {
        await page.getByRole('button', { name: 'Start My Business' }).click();
        await page.getByRole('button', { name: 'Online Store' }).click();
        await page.getByPlaceholder('Enter your business name').fill('Sweet Treats');
        await page.getByRole('button', { name: 'Next' }).click();

        // Step 3
        const physicalProducts = page.getByText('Physical Products');
        await expect(physicalProducts).toBeVisible();
        // Since it's a label with a checkbox, click the text
        await physicalProducts.click();

        await page.getByRole('button', { name: 'Next' }).click();
        await expect(page.getByText('Add your first product (optional)')).toBeVisible();
    });

    test('can skip product addition and reach agent selection', async ({ page }) => {
        await page.getByRole('button', { name: 'Start My Business' }).click();
        await page.getByRole('button', { name: 'Online Store' }).click();
        await page.getByPlaceholder('Enter your business name').fill('Sweet Treats');
        await page.getByRole('button', { name: 'Next' }).click();
        await page.getByText('Physical Products').click();
        await page.getByRole('button', { name: 'Next' }).click();

        // Step 4: Skip product
        const skipButton = page.getByRole('button', { name: 'Skip for now' });
        await expect(skipButton).toBeVisible();
        await skipButton.click();

        await expect(page.getByText('How will you get paid?')).toBeVisible();
    });
});
