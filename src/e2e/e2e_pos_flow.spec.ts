import { expect } from '@playwright/test';
import { test } from './fixtures';

test.describe('End-to-End In-Person Payment Flow', () => {
  test('should process a POS terminal payment successfully', async ({ page }) => {
    // 1. Navigate: Go to the POS terminal page
    await page.goto('/pos/terminal');

    // 2. Unlock the terminal with PIN 1234
    // Set the localStorage directly before reloading to bypass the mock data missing issue
    await page.evaluate(() => {
        window.localStorage.setItem('ohc_offline_staff', JSON.stringify([{
            id: 'e2e-team-member',
            name: 'Carlos',
            role: 'Manager',
            pin_hash: '1234'
        }]));
    });

    await page.reload();

    await page.getByRole('button', { name: '1' }).click();
    await page.getByRole('button', { name: '2' }).click();
    await page.getByRole('button', { name: '3' }).click();
    await page.getByRole('button', { name: '4' }).click();

    // After unlocking, we should see the staff's name and Clock In button
    await expect(page.getByText('Carlos')).toBeVisible();

    // Clock in
    await page.getByRole('button', { name: 'Clock In' }).click();
    await expect(page.getByText('Clocked In')).toBeVisible();

    // Track network requests
    const tokenRequestPromise = page.waitForRequest(request =>
        request.url().includes('/api/v1/payments/terminal/token') && request.method() === 'GET'
    );

    // We expect the intent API to be called when the token returns successfully
    const intentRequestPromise = page.waitForRequest(request =>
        request.url().includes('/api/v1/payments/terminal/intent') && request.method() === 'POST'
    );

    page.on('dialog', async dialog => {
        await dialog.accept();
    });

    await page.getByRole('button', { name: 'New Order' }).click();

    // We expect the terminal token API to be called
    await tokenRequestPromise;

    // We expect the intent API to be called
    const intentReq = await intentRequestPromise;
    const postData = intentReq.postDataJSON();
    expect(postData.amount_cents).toBe(5000);
    expect(postData.currency).toBe('USD');

    // Dashboard verification can be added if needed, but first we must wire up the frontend
  });
});
