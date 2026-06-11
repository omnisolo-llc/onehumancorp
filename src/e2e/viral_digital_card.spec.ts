import { test, expect } from '@playwright/test';

test.describe('Viral Digital Business Card Flow', () => {
  test('User can generate and view a digital business card', async ({ page, request }) => {
    // Navigate to generator page
    await page.goto('http://localhost:3000/digital-card');

    // Fill out form
    await page.fill('input[name="name"]', 'John Doe');
    await page.fill('input[name="title"]', 'Software Engineer');
    await page.fill('input[name="company"]', 'OneHumanCorp');
    await page.fill('input[name="email"]', 'john.doe@example.com');
    await page.fill('input[name="phone"]', '123-456-7890');
    await page.fill('input[name="website"]', 'https://example.com');
    await page.fill('textarea[name="bio"]', 'I write code.');
    await page.selectOption('select[name="theme"]', 'dark');

    // Submit form (this might fail if the DB seed doesn't have a default tenant,
    // but the backend uses the first tenant found in this prototype setup)
    await page.click('button[type="submit"]');

    // Wait for navigation
    await page.waitForURL(/\/card\/[a-zA-Z0-9-]{36}/);

    // Verify card content
    await expect(page.locator('h1')).toHaveText('John Doe');
    await expect(page.locator('text=Software Engineer')).toBeVisible();
    await expect(page.locator('text=OneHumanCorp')).toBeVisible();
    await expect(page.locator('text="I write code."')).toBeVisible();

    // Verify powered by link
    const poweredByLink = page.locator('a[href*="ref=digital_card"]');
    await expect(poweredByLink).toBeVisible();
    await expect(poweredByLink).toHaveText(/Powered by\s*OHC/);
  });
});
