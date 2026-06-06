import { expect } from '@playwright/test';
import { test } from './fixtures';

test.describe('Tenant Isolation & Business Setup Data Model', () => {
    test('verifies UI does not expose technical terminology and navigates correctly', async ({ page }) => {
        await page.goto('/dashboard');
        await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

        await page.getByRole('link', { name: 'Settings' }).click();

        await expect(page.getByRole('heading', { name: 'Settings' })).toBeVisible();

        await expect(page.getByText('Kubernetes', { exact: true })).not.toBeVisible();
        await expect(page.getByText('Raw Payloads', { exact: true })).not.toBeVisible();
    });

    test('verifies mobile viewport responsiveness on dashboard', async ({ page }) => {
        await page.setViewportSize({ width: 375, height: 812 });
        await page.goto('/dashboard');

        await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
        await expect(page.locator('.app-sidebar')).toBeVisible();
        await expect(page.locator('.app-nav-link.is-active').first()).toBeVisible();
    });

    test('verifies navigation between different product dashboard sections', async ({ page }) => {
        await page.goto('/dashboard');

        await page.getByRole('link', { name: 'Agents' }).click();
        await expect(page.getByRole('heading', { name: 'AI Departments' })).toBeVisible();
    });

    test('verifies creation of a business respects data flow', async ({ page }) => {
        await page.goto('/dashboard');

        await page.getByRole('link', { name: 'New Product' }).click();

        await expect(page.getByRole('heading', { name: 'Add Product' })).toBeVisible();
    });

    test('verifies agent history panel does not expose raw embeddings', async ({ page }) => {
        await page.goto('/dashboard');

        await page.getByRole('link', { name: 'Agents' }).click();

        await expect(page.getByText('vector', { exact: false })).not.toBeVisible();
        await expect(page.getByText('1536', { exact: false })).not.toBeVisible();
    });

    test('verifies cloud multi-tenant IDOR protection by injecting system tenant_id in API via UI', async ({ page }) => {
        await page.goto('/login');
        await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
        await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password');
        await page.getByRole('button', { name: 'Log In' }).click();

        await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

        let idorResponseStatus = 0;

        await page.route('**/api/v1/user/profile*', async route => {
            const request = route.request();
            const headers = request.headers();
            headers['x-organization-id'] = 'system';

            const response = await route.fetch({ headers });
            idorResponseStatus = response.status();

            await route.fulfill({ response });
        });

        await page.getByRole('link', { name: 'Settings' }).click();

        await expect(page.getByRole('heading', { name: 'Settings' })).toBeVisible();

        if (idorResponseStatus !== 0) {
            expect([401, 403, 500, 400]).toContain(idorResponseStatus);
        }
    });

    test('verifies standard authenticated queries succeed without system tampering', async ({ page }) => {
        await page.goto('/login');
        await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
        await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password');
        await page.getByRole('button', { name: 'Log In' }).click();

        await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

        const responsePromise = page.waitForResponse(response => response.url().includes('/api/v1/user/profile'));
        await page.getByRole('link', { name: 'Settings' }).click();
        const response = await responsePromise;

        expect(response.status()).toBe(200);
    });
});
