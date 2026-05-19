import { test, expect } from './fixtures';

test.describe('Wizard and Tone Tuning Improvements', () => {
  test('validates form inputs on step 7', async ({ page }) => {
    await page.goto('/website-builder');
    // Start wizard
    await page.getByRole('button', { name: /Start My Business Next/ }).click();
    await page.getByRole('button', { name: /Creative/ }).click();

    // Step 3
    await page.getByPlaceholder('What is your business called?').fill('Test Business');
    await page.getByRole('button', { name: /Next/ }).click();

    // Step 4
    await page.getByText('Physical Products', { exact: true }).click();
    await page.getByRole('button', { name: /Next/ }).click();

    // Step 5
    await page.getByPlaceholder('What is the name of this product?').fill('Product');
    await page.getByPlaceholder('0.00').fill('10');
    await page.getByRole('button', { name: /Next/ }).click();

    // Step 6
    await page.getByRole('button', { name: 'Online' }).click();

    // Step 7
    // Ensure that it stays on step 7 if we don't fill out the inputs
    const handleDialog = async (dialog) => {
        expect(dialog.message()).toBe('Please fill out all account fields');
        await dialog.accept();
    };
    page.on('dialog', handleDialog);
    await page.getByRole('button', { name: 'Next →' }).click();

    // Expect we are still on step 7
    await expect(page.getByRole('heading', { name: 'Create your account' })).toBeVisible();
    page.off('dialog', handleDialog);
  });

  test('updates tone setting for Marketing Pro', async ({ page }) => {
    await page.goto('/agents');

    // Open Marketing Pro settings
    await page.getByRole('heading', { name: 'Marketing Pro' }).click();

    // Expect tone tuning select to be visible
    await expect(page.getByText('Tone Tuning')).toBeVisible();

    const handleDialog = async (dialog) => {
        expect(dialog.message()).toBe('Tone updated to humorous for ambassador.');
        await dialog.accept();
    };
    page.on('dialog', handleDialog);

    // Select an option
    await page.locator('#tone-tuning-select').selectOption('humorous');
    page.off('dialog', handleDialog);
  });
});
