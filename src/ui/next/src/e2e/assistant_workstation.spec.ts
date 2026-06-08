import { test, expect } from '@playwright/test';

test.describe('Assistant Workstation CUJ', () => {
  test('User can navigate to Assistant from Dashboard and see the dense workstation layout', async ({ page }) => {
    // 1. Navigate to the dashboard
    await page.goto('/dashboard');

    // 2. Find and click the new "AI Assistant Workstation" quick-action card
    const assistantLink = page.getByRole('link', { name: /Open Assistant/i });
    await expect(assistantLink).toBeVisible();
    await assistantLink.click();

    // 3. Verify we are on the assistant page
    await expect(page).toHaveURL(/.*\/assistant.*/);

    // 4. Verify the dense workstation layout elements
    // Header title
    await expect(page.getByRole('heading', { name: 'Jarvis Assistant' })).toBeVisible();

    // Task List panel
    await expect(page.getByTestId('assistant-workstation')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Task List' })).toBeVisible();

    // Task Composer panel
    await expect(page.getByRole('heading', { name: 'Task Composer' })).toBeVisible();

    // Results Panel
    await expect(page.getByRole('heading', { name: 'Results Panel' })).toBeVisible();

    // 5. Verify the Expert Center link is present
    const expertCenterLink = page.getByRole('link', { name: 'Expert Center' });
    await expect(expertCenterLink).toBeVisible();
    await expect(expertCenterLink).toHaveAttribute('href', '/agents');
  });
});
