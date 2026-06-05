import { test, expect } from './fixtures';

test.describe('Promoter Social Content Generator CUJ', () => {
  test('Owner adds a new product and approves Promoter social calendar', async ({ page, request }) => {
    test.skip(process.env.OHC_API_ONLY_E2E === 'true', 'API-only E2E tests do not run frontend routes');
    await page.goto('http://localhost:3000/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Login' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    await page.goto('http://localhost:3000/catalog');

    const addBtn = page.getByRole('button', { name: 'Add Product' }).first();
    if (await addBtn.isVisible()) {
        await addBtn.click();
    } else {
        await page.goto('http://localhost:3000/catalog/new');
    }

    await page.getByLabel('Name').fill('Summer Collection Dress');
    await page.getByLabel('Price').fill('89.99');

    const saveBtn = page.getByRole('button', { name: 'Save Product' }).first();
    if (await saveBtn.isVisible()) {
        await saveBtn.click();
    } else {
        await page.getByRole('button', { name: 'Save' }).first().click();
    }

    await page.waitForTimeout(2000);

    await page.goto('http://localhost:3000/team');
    await expect(page.getByRole('heading', { name: 'Your Team', exact: true })).toBeVisible();

    await page.getByRole('button', { name: 'The Promoter' }).first().click();

    await expect(page.getByRole('heading', { name: 'The Promoter' })).toBeVisible({ timeout: 5000 });

    const socialCalendarText = page.getByText('7-Day Social Calendar Generated').first();
    await expect(socialCalendarText).toBeVisible({ timeout: 15000 });

    await expect(page.getByText('The Generative Promoter has created a week of content')).toBeVisible();

    await page.getByRole('button', { name: 'Approve' }).first().click();

    await expect(page.getByText('7-Day Social Calendar Generated')).toBeHidden();
  });
});
