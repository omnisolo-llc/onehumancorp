import { expect } from '@playwright/test';
import { test } from './fixtures';

test.describe('In-Person Payment (POS) Flow', () => {
  test('should complete a tap-to-pay transaction and sync natively', async ({ page, context }) => {
    // Navigate to the POS terminal page
    await page.goto('/pos/terminal');

    // Wait for the UI to load
    await expect(page.locator('h1', { hasText: 'Terminal Locked' })).toBeVisible({ timeout: 25000 });

    // Click inside the body to ensure interaction context
    await page.mouse.click(10, 10);
    await page.waitForTimeout(1000);

    // Enter PIN: 1234
    await page.getByRole('button', { name: '1', exact: true }).click();
    await page.getByRole('button', { name: '2', exact: true }).click();
    await page.getByRole('button', { name: '3', exact: true }).click();
    await page.getByRole('button', { name: '4', exact: true }).click();

    // Wait for the backend response to succeed and show staff name
    await expect(page.locator('h1', { hasText: 'Carlos' })).toBeVisible({ timeout: 10000 });

    // Ensure the catalog loaded natively from backend
    await expect(page.locator('text=Vegan Celebration Cake')).toBeVisible();

    // Trigger New Order via a product select
    await page.locator('text=Vegan Celebration Cake').click();

    // Ensure we are clocked in first
    const clockInBtn = page.getByRole('button', { name: 'Clock In' });
    if (await clockInBtn.isVisible()) {
        await clockInBtn.click();
        await expect(page.locator('h2', { hasText: 'Clocked In' })).toBeVisible();
    }

    // Connect mock reader
    const discoverBtn = page.locator('text=Discover Readers');
    if (await discoverBtn.isVisible()) {
        await discoverBtn.click();
        await page.locator('text=Connect').first().click();
        await expect(page.locator('text=Collect Payment $39.99')).toBeVisible({ timeout: 10000 });
    }

    // Now process the payment (which acquires a Redis lock natively through the backend)
    await page.locator('text=Collect Payment $39.99').click();

    // Check loading/processing state
    await expect(page.getByRole('button', { name: 'Processing...' })).toBeVisible({ timeout: 10000 });

    // Ensure intent completes
    await expect(page.locator('text=Payment successful!')).toBeVisible({ timeout: 15000 });
  });
});
