import { test, expect } from '../../../../e2e/fixtures';

test.describe('Milestone Celebration Growth Loop', () => {
    test('displays milestone modal and correct links after login', async ({ page }) => {
        await page.goto('/milestones');

        await expect(page.getByRole('heading', { name: /Success Milestones/i })).toBeVisible();
        await expect(page.getByText('10th Order Milestone')).toBeVisible();
        await page.getByRole('button', { name: /Back to Dashboard/i }).click();
        await expect(page).toHaveURL(/\/dashboard$/);
    });
});
