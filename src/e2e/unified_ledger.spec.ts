import { test, expect } from './fixtures';

test.describe('Recorded ledger activity and truthful balance availability', () => {

    test('dashboard opens the actual authenticated statement without inventing a balance', async ({ memberPage: page }) => {
        const writes: string[] = [];
        page.on('request', request => {
            if (new URL(request.url()).pathname.startsWith('/api/v1/ledger/')
                && !['GET', 'HEAD'].includes(request.method())) writes.push(request.method());
        });
        await page.goto('/dashboard');
        const financialsCard = page.getByTestId('dashboard-financials-card');
        await expect(financialsCard).toBeVisible();
        await expect(financialsCard.getByText('Balance unavailable', { exact: true })).toBeVisible();
        await expect(financialsCard).not.toContainText('$1,500.00');
        const read = page.waitForResponse(response =>
            new URL(response.url()).pathname === '/api/v1/ledger/entries'
            && response.request().method() === 'GET');
        await financialsCard.getByRole('link', { name: 'Recent Activity', exact: true }).click();
        await expect(page).toHaveURL(/\/dashboard\/ledger(?:\?.*)?$/);
        const response = await read;
        expect(response.status()).toBe(200);
        const body = await response.json();
        expect(Array.isArray(body.entries)).toBe(true);
        await expect(page.getByRole('heading', { name: 'Ledger Statement', exact: true })).toBeVisible();
        if (body.entries.length === 0) {
            await expect(page.getByText('No recent activity.', { exact: true })).toBeVisible();
        } else {
            await expect(page.getByRole('table')).toBeVisible();
            for (const entry of body.entries) {
                expect(typeof entry.entry_type).toBe('string');
                const amount = `${entry.currency.toUpperCase()} ${new Intl.NumberFormat('en-US', { maximumFractionDigits: 20 }).format(entry.amount)}`;
                await expect(page.getByRole('cell', { name: amount, exact: true }).first()).toBeVisible();
            }
        }
        expect(writes).toEqual([]);
    });

    test('accountant page explains an unavailable balance instead of a fabricated answer', async ({ memberPage: page }) => {
        await page.goto('/agent/chat');
        await page.fill('textarea', 'What is my current ledger balance?');
        await page.click('button[aria-label="Send message"]');

        await expect(page.getByText(/A verified balance is unavailable here/)).toBeVisible();
        await expect(page.getByText(/1500\.00|current verified ledger balance/)).toHaveCount(0);
        await expect(page.getByRole('link', { name: 'View ledger statement' })).toHaveAttribute('href', '/dashboard/ledger');
    });
});
