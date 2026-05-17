import { test, expect } from '@playwright/test';

test.describe('Business Setup Wizard Comprehensive Flow', () => {
  test('traverses the current wizard from welcome to launch', async ({ page }) => {
    await page.goto('/website-builder');
    await page.getByRole('button', { name: /Start My Business Next/ }).click();
    await page.getByRole('button', { name: /Creative/ }).click();
    await page.getByPlaceholder('What is your business called?').fill('Alex Art');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByLabel(/Services/).check();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByPlaceholder('What is the name of this product?').fill('Portrait Session');
    await page.getByPlaceholder('0.00').fill('120');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Both Online/ }).click();
    await page.getByPlaceholder('e.g. Maya Smith').fill('Alex Artist');
    await page.getByPlaceholder('you@email.com').fill('alex@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: 'Modern' }).click();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Connect Custom Domain/ }).click();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Publish my business/ }).click();

    await expect(page.getByText('Your business is now live!')).toBeVisible();
  });
});
