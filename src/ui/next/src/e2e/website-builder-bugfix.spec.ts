import { test, expect, type Page } from '../../../../e2e/fixtures';

test.describe('Website Builder Tool (E2E Validation)', () => {

    test('renders the initial step successfully', async ({ page }) => {
        await page.goto('/website-builder');
        await expect(page.getByRole('heading', { name: 'Setup Assistant' })).toBeVisible();
    });

    test('can enter business type and advance', async ({ page }) => {
        await page.goto('/website-builder');
        await expect(page.getByRole('heading', { name: 'Setup Assistant' })).toBeVisible();
        await page.getByRole('button', { name: 'Start My Business' }).click();

        await expect(page.getByRole('heading', { name: 'What kind of business are you building?' })).toBeVisible();
        await page.getByRole('button', { name: 'Online Store' }).click();

        await expect(page.getByRole('heading', { name: 'Give your business a name' })).toBeVisible();
    });

    test('can enter business name', async ({ page }) => {
        await page.goto('/website-builder');
        await page.getByRole('button', { name: 'Start My Business' }).click();
        await page.getByRole('button', { name: 'Online Store' }).click();

        await expect(page.getByRole('heading', { name: 'Give your business a name' })).toBeVisible();

        const businessNameInput = page.getByPlaceholder('What is your business called?');
        await businessNameInput.fill('My Test Business');

        await page.getByRole('button', { name: 'Next' }).click();

        await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();
    });

    test('can select selling options', async ({ page }) => {
        await page.goto('/website-builder');
        await page.getByRole('button', { name: 'Start My Business' }).click();
        await page.getByRole('button', { name: 'Online Store' }).click();
        await page.getByPlaceholder('What is your business called?').fill('My Test Business');
        await page.getByRole('button', { name: 'Next' }).click();

        await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();

        // Select Physical Products
        const physicalProductsLabel = page.locator('label').filter({ hasText: 'Physical Products' });
        await physicalProductsLabel.locator('input[type="checkbox"]').check();

        await page.getByRole('button', { name: 'Next' }).click();

        await expect(page.getByRole('heading', { name: 'Product details' })).toBeVisible();
    });

    test('can skip product addition and reach agent selection', async ({ page }) => {
        await page.goto('/website-builder');
        await page.getByRole('button', { name: 'Start My Business' }).click();
        await page.getByRole('button', { name: 'Online Store' }).click();
        await page.getByPlaceholder('What is your business called?').fill('My Test Business');
        await page.getByRole('button', { name: 'Next' }).click();
        const physicalProductsLabel = page.locator('label').filter({ hasText: 'Physical Products' });
        await physicalProductsLabel.locator('input[type="checkbox"]').check();
        await page.getByRole('button', { name: 'Next' }).click();

        await expect(page.getByRole('heading', { name: 'Product details' })).toBeVisible();

        await page.getByPlaceholder('What is the name of this product?').fill('Test Product');
        await page.getByPlaceholder('0.00').fill('19.99');
        await page.getByRole('button', { name: 'Next' }).click();

        await expect(page.getByRole('heading', { name: 'How do you want to receive payments?' })).toBeVisible();
    });
});
