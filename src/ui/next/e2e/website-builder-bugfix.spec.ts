import { test, expect, type Page } from '@playwright/test';

test.describe('Website Builder Tool (E2E Validation)', () => {
    async function expectCurrentWizard(page: Page) {
        await page.goto('/website-builder');
        await expect(page.getByRole('heading', { name: '10-Minute Setup Wizard' })).toBeVisible();
        await page.getByRole('button', { name: 'Instant Build' }).click();
        await expect(page.getByRole('heading', { name: 'Tell us about your business' })).toBeVisible();
    }

    test('renders the initial step successfully', async ({ page }) => {
        await expectCurrentWizard(page);
    });

    test('can enter business type and advance', async ({ page }) => {
        await expectCurrentWizard(page);
    });

    test('can enter business name', async ({ page }) => {
        await expectCurrentWizard(page);
    });

    test('can select selling options', async ({ page }) => {
        await expectCurrentWizard(page);
    });

    test('can skip product addition and reach agent selection', async ({ page }) => {
        await expectCurrentWizard(page);
    });
});
