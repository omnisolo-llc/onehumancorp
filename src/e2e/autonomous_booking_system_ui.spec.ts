import { test, expect } from './fixtures';

test('Owner creates a new calendar event for Leo', async ({ page, adminUser, loginAs }) => {
    await loginAs(adminUser);

    await page.goto('/calendar');

    await expect(page.locator('h1')).toHaveText(/Calendar/i);

    await page.locator('button:has-text("Add Event")').click();
    await page.locator('input[name="title"]').fill('Guitar Lesson with Test User');
    await page.locator('button:has-text("Save")').click();

    await expect(page.locator('.toast')).toHaveText(/Event Created/i);
});
