import { test, expect } from '@playwright/test';

test.describe('Agent Terminal Multi-Backend UI', () => {
    test('Simulate changing backend and running a command', async ({ page, request }) => {
        // Mock the API responses
        await page.route('/api/v1/payments/terminal/backend', async route => {
            if (route.request().method() === 'GET') {
                await route.fulfill({ json: { backend: 'local' } });
            } else if (route.request().method() === 'POST') {
                const body = JSON.parse(route.request().postData() || '{}');
                await route.fulfill({ json: { success: true, backend: body.backend } });
            }
        });

        await page.route('/api/v1/payments/terminal/session/start', async route => {
            await route.fulfill({ json: { output: 'hello\n' } });
        });

        // Navigate to the new page
        await page.goto('/agent-terminal');

        // Verify "Assistant-First Shell" header
        await expect(page.locator('h1', { hasText: 'Assistant-First Shell' })).toBeVisible();

        // Check backend select and switch to Docker
        const backendSelect = page.locator('select');
        await expect(backendSelect).toHaveValue('local');
        await backendSelect.selectOption('docker');

        // Check that the UI logged the switch
        await expect(page.locator('.bg-black', { hasText: '[System] Switched to docker backend.' })).toBeVisible();

        // Submit a command
        const inputField = page.getByPlaceholder('Enter command (e.g. echo hello)...');
        await inputField.fill('echo hello');
        await page.getByRole('button', { name: 'Submit' }).click();

        // Verify the output displays the command and response
        await expect(page.locator('.bg-black', { hasText: '$ echo hello' })).toBeVisible();
        await expect(page.locator('.bg-black', { hasText: 'hello' }).last()).toBeVisible();
    });
});
