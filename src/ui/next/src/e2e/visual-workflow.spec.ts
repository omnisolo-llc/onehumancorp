import { test, expect } from './fixtures';

test.describe('Visual Workflow Builder', () => {
    test('renders correctly', async ({ page }) => {
        await page.goto('/visual-workflow');
        await expect(page.locator('h1', { hasText: 'Visual Workflow Builder' })).toBeVisible();
        await expect(page.locator('aside', { hasText: 'Nodes' })).toBeVisible();
        await expect(page.locator('button', { hasText: 'Deploy Workflow' })).toBeVisible();
    });
});
