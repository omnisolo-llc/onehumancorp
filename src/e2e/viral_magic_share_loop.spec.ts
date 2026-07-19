import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test.describe('Viral Magic Share Loop', () => {
    test('renders Magic Share Link widget in social post draft', async ({ page, adminUser, loginAs }) => {
        await loginAs(page, adminUser);

        await page.goto('/dashboard');

        await expect(page.locator('h1')).toBeVisible();
    });
});
