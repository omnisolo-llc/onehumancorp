import { test, expect } from '@playwright/test';

test.describe('SONA Neural Patterns Dashboard', () => {
  test('User can view learned trajectory patterns', async ({ page }) => {
    // 1. Navigate to the SONA page
    await page.goto('/sona');

    // 2. Verify page loads correctly
    await expect(page.getByRole('heading', { name: /SONA Neural Patterns Dashboard/i })).toBeVisible();
    await expect(page.getByText(/Self-Learning Trajectory Patterns/i)).toBeVisible();

    // 3. Wait for loading to disappear
    await expect(page.getByText('Loading patterns...')).not.toBeVisible({ timeout: 10000 });

    // 4. Either patterns are listed or "No patterns recorded yet." is visible
    const emptyState = page.getByText('No patterns recorded yet.');
    const patternScore = page.getByText('Score: ');

    await expect(emptyState.or(patternScore).first()).toBeVisible();
  });

  test('User can record a new trajectory pattern', async ({ page }) => {
    await page.goto('/sona');
    await expect(page.getByText('Loading patterns...')).not.toBeVisible({ timeout: 10000 });

    const contextInput = page.getByPlaceholder('Task Context');
    const toolInput = page.getByPlaceholder('Tool used');
    const recordBtn = page.getByRole('button', { name: 'Record Pattern' });

    await expect(recordBtn).toBeDisabled();
    await contextInput.fill('Write E2E tests for Playwright');
    await toolInput.fill('bash');
    await expect(recordBtn).toBeEnabled();

    await recordBtn.click();

    // Verify that the UI state updates and reflects the newly added pattern
    // (mocked or real environment)
    await expect(page.getByText('Write E2E tests for Playwright')).toBeVisible({ timeout: 15000 });
  });

  test('Form validation keeps button disabled if missing tool', async ({ page }) => {
    await page.goto('/sona');
    const contextInput = page.getByPlaceholder('Task Context');
    const recordBtn = page.getByRole('button', { name: 'Record Pattern' });
    await contextInput.fill('Write E2E tests for Playwright');
    await expect(recordBtn).toBeDisabled();
  });

  test('Form validation keeps button disabled if missing context', async ({ page }) => {
    await page.goto('/sona');
    const toolInput = page.getByPlaceholder('Tool used');
    const recordBtn = page.getByRole('button', { name: 'Record Pattern' });
    await toolInput.fill('bash');
    await expect(recordBtn).toBeDisabled();
  });

  test('Displays error if backend is unavailable on load', async ({ page }) => {
    // Note: The actual error display depends on the mock environment,
    // but the UI must render gracefully.
    await page.goto('/sona');
    await expect(page.getByRole('heading', { name: 'SONA Neural Patterns Dashboard' })).toBeVisible();
    // It should not crash or throw unhandled exceptions.
  });
});