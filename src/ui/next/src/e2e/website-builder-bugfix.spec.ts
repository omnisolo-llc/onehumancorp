import { test, expect, type Page } from '@playwright/test';

test.describe('Website Builder Tool (E2E Validation)', () => {

    test.beforeEach(async ({ page }) => {
        // Clear local storage to ensure fresh state
        await page.addInitScript(() => {
          window.localStorage.clear();
        });
    });

    async function navigateToWizard(page: Page) {
        await page.goto('/website-builder');
        await expect(page.getByRole('heading', { name: '10-Minute Setup Wizard' })).toBeVisible();
    }

    test('renders the initial step successfully and can use instant build', async ({ page }) => {
        await navigateToWizard(page);
        await page.getByRole('button', { name: 'Instant Build' }).click();
        await expect(page.getByRole('heading', { name: 'Tell us about your business' })).toBeVisible();
        await page.getByPlaceholder(/e.g. I run a local bakery/i).fill('Test Bakery');
        await page.getByRole('button', { name: 'Generate Storefront' }).click();
        await expect(page.getByText("Success! Your business is live!")).toBeVisible({ timeout: 10000 });
    });

    test('can enter business type and advance', async ({ page }) => {
        await navigateToWizard(page);
        await page.getByRole('button', { name: 'Start My Business' }).click();
        await expect(page.getByRole('heading', { name: 'What kind of business are you building?' })).toBeVisible();
        await page.getByRole('button', { name: 'Online Store' }).click();
        await expect(page.getByRole('heading', { name: 'Give your business a name' })).toBeVisible();
    });

    test('can enter business name', async ({ page }) => {
        await navigateToWizard(page);
        await page.getByRole('button', { name: 'Start My Business' }).click();
        await page.getByRole('button', { name: 'Online Store' }).click();

        await expect(page.getByRole('heading', { name: 'Give your business a name' })).toBeVisible();
        await page.getByPlaceholder('What is your business called?').fill('Maya Bakery');
        await page.getByPlaceholder("e.g. Maya's Cakes").fill('I bake custom vegan cakes for weddings and parties.');
        await page.getByRole('button', { name: 'Next' }).click();

        await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();
    });

    test('can select selling options', async ({ page }) => {
        await navigateToWizard(page);
        await page.getByRole('button', { name: 'Start My Business' }).click();
        await page.getByRole('button', { name: 'Online Store' }).click();

        await page.getByPlaceholder('What is your business called?').fill('Maya Bakery');
        await page.getByPlaceholder("e.g. Maya's Cakes").fill('I bake custom vegan cakes for weddings and parties.');
        await page.getByRole('button', { name: 'Next' }).click();

        await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();
        await page.getByText('Physical Products').click();
        await page.getByRole('button', { name: 'Next' }).click();

        await expect(page.getByRole('heading', { name: 'Product details' })).toBeVisible();
    });

    test('can skip product addition and reach agent selection', async ({ page }) => {
        await navigateToWizard(page);
        await page.getByRole('button', { name: 'Start My Business' }).click();
        await page.getByRole('button', { name: 'Online Store' }).click();

        await page.getByPlaceholder('What is your business called?').fill('Maya Bakery');
        await page.getByPlaceholder("e.g. Maya's Cakes").fill('I bake custom vegan cakes for weddings and parties.');
        await page.getByRole('button', { name: 'Next' }).click();

        await page.getByText('Physical Products').click();
        await page.getByRole('button', { name: 'Next' }).click();

        await expect(page.getByRole('heading', { name: 'Product details' })).toBeVisible();
        await page.getByPlaceholder('What is the name of this product?').fill('Vegan Cake');
        await page.getByPlaceholder('0.00').fill('45.00');
        await page.getByRole('button', { name: 'Next' }).click();

        await expect(page.getByRole('heading', { name: 'How do you want to receive payments?' })).toBeVisible();
        await page.getByRole('button', { name: 'Online' }).click();

        await expect(page.getByRole('heading', { name: 'Create your account' })).toBeVisible();
        await page.getByPlaceholder('e.g. Maya Smith').fill('Maya');
        await page.getByPlaceholder('you@email.com').fill('maya@example.com');
        await page.getByPlaceholder('Password').fill('password123');
        await page.getByRole('button', { name: 'Next' }).click();

        await expect(page.getByRole('heading', { name: 'Template selection' })).toBeVisible();
        await page.getByRole('button', { name: 'Modern' }).click();

        await page.getByRole('button', { name: 'Next' }).click();
        await expect(page.getByRole('heading', { name: 'Choose your domain' })).toBeVisible();
        await page.getByRole('button', { name: 'Free OHC Domain' }).click();

        await page.getByRole('button', { name: 'Next' }).click();
        await expect(page.getByRole('heading', { name: 'Review your choices' })).toBeVisible();

        await page.getByRole('button', { name: 'Publish my business' }).click();
        await expect(page.getByText("Success! Your business is live!")).toBeVisible({ timeout: 10000 });
    });
});
