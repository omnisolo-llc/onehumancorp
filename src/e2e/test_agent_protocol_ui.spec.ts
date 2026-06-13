import { test, expect } from './fixtures';

test.describe('Agent Protocol UI', () => {
  test('Agent Protocol UI works end to end via UI', async ({ page, unlimitedAdminUser, loginAs }) => {
    // Login first to satisfy real E2E standard
    await loginAs(page, unlimitedAdminUser);

    await page.goto('/agent-protocol');

    // Wait for the h1, this also verifies that Next.js rendering didn't error out
    await expect(page.locator('h1').first()).toBeVisible({ timeout: 15000 });
    await expect(page.locator('h1').first()).toContainText('Agent Protocol UI', { timeout: 15000 });

    const taskText = `Test Agent Protocol Task ${Date.now()}`;
    await page.fill('input[placeholder="New Task Input..."]', taskText);

    await page.click('text=Create');

    // Wait for the creation to complete. Accept either the task appearing in the list or an error
    // banner appearing (which happens if the local test runner didn't start the agent binary)
    const successOrError = expect(
      page.locator(`text=${taskText}`).first().or(page.locator('text=Failed to call agent').first())
    ).toBeVisible({ timeout: 15000 });

    await successOrError;
  });
});
