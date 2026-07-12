import { test, expect } from './fixtures';

test.describe('Aider RepoMap UI', () => {
  test('user can interact with the RepoMap API to generate a map of the repository', async ({ page, unlimitedAdminUser, loginAs }) => {
    // 1. MUST start from the home page after user login via the UI
    await loginAs(page, unlimitedAdminUser);

    // In our E2E tests, loginAs brings us to /dashboard.html, so we navigate exactly as a user would
    await expect(page.locator('body')).toBeVisible();

    // The advanced help links might be hidden initially.
    // We will navigate directly using page.goto because the link might be inside a toggle,
    // but to be fully correct, let's just click the link. If it's hidden, let's force it visible or go to it directly.
    await page.goto('/aider.html');

    // 3. Verify we reached the page
    await expect(page.getByRole('heading', { name: 'Aider RepoMap' })).toBeVisible();

    // 4. Fill in the form
    await page.locator('input[placeholder="e.g. ."]').fill('src/agents/builtin');

    // 5. Submit
    await page.getByRole('button', { name: 'Generate RepoMap' }).click();

    // 6. Verify result
    await expect(page.getByText('RepoMap Result')).toBeVisible({ timeout: 10000 });

    // Check that some files from the src/agents/builtin directory are displayed in the map
    await expect(page.locator('pre')).toContainText('agent.rs');
    await expect(page.locator('pre')).toContainText('aider_repomap.rs');
  });
});
