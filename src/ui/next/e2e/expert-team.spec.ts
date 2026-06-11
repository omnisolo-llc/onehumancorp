import { test, expect } from '@playwright/test';

test.describe('Expert Team CUJ', () => {
  test('User can execute task via Expert Team and see the final delivered output', async ({ page }) => {
    await page.goto('/expert-team');
    await expect(page.getByRole('heading', { name: 'Collaborative Expert Team' })).toBeVisible();
    const taskTextarea = page.getByPlaceholder('e.g. Write a comprehensive business plan');
    await taskTextarea.fill('Analyze the AI trends for small businesses');
    const executeButton = page.getByRole('button', { name: 'Execute Task via Expert Team' });
    await executeButton.click();
    await expect(executeButton).toHaveText('Orchestrating Expert Team...');
    await expect(executeButton).toBeDisabled();
    await expect(page.getByRole('heading', { name: 'Final Delivered Output' })).toBeVisible({ timeout: 15000 });
  });
});
