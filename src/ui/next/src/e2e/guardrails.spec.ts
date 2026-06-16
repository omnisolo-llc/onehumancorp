import { test, expect } from '@playwright/test';

test.describe('Guardrails & Safety', () => {
  test('Anthropic 3-Stage Tool Gating - Tool allowed', async ({ page }) => {
    await page.goto('/guardrails');

    // Trust project
    await page.locator('input[type="checkbox"]').check();

    // Fill in tools
    await page.locator('input[placeholder="Comma separated tools, e.g. read_file, execute_bash"]').fill('read_file, execute_bash');
    await page.locator('input[placeholder="Comma separated tools, e.g. delete_database"]').fill('delete_database');
    await page.locator('input[placeholder="e.g. execute_bash"]').fill('execute_bash');

    // Run
    await page.getByRole('button', { name: 'Test Guardrails' }).click();

    // Verify Success
    await expect(page.getByTestId('success-message')).toBeVisible();
    await expect(page.getByTestId('success-message')).toContainText('Tool Allowed');
    await expect(page.getByTestId('success-message')).toContainText('passed all 3 guardrail stages successfully');
  });

  test('Anthropic 3-Stage Tool Gating - Stage 1 Trust Blocked', async ({ page }) => {
    await page.goto('/guardrails');

    // Untrusted (default is unchecked)

    // Fill in tools
    await page.locator('input[placeholder="Comma separated tools, e.g. read_file, execute_bash"]').fill('read_file, execute_bash');
    await page.locator('input[placeholder="Comma separated tools, e.g. delete_database"]').fill('delete_database');
    await page.locator('input[placeholder="e.g. execute_bash"]').fill('execute_bash');

    // Run
    await page.getByRole('button', { name: 'Test Guardrails' }).click();

    // Verify Blocked
    await expect(page.getByTestId('success-message')).toBeVisible();
    await expect(page.getByTestId('success-message')).toContainText('Guardrail Tripped');
    await expect(page.getByTestId('success-message')).toContainText('Stage 1 (Trust) tripped');
  });

  test('Anthropic 3-Stage Tool Gating - Stage 2 Permission Blocked', async ({ page }) => {
    await page.goto('/guardrails');

    // Trust project
    await page.locator('input[type="checkbox"]').check();

    // Fill in tools (allow list doesn't have the tool we want to run)
    await page.locator('input[placeholder="Comma separated tools, e.g. read_file, execute_bash"]').fill('read_file');
    await page.locator('input[placeholder="Comma separated tools, e.g. delete_database"]').fill('delete_database');
    await page.locator('input[placeholder="e.g. execute_bash"]').fill('execute_bash');

    // Run
    await page.getByRole('button', { name: 'Test Guardrails' }).click();

    // Verify Blocked
    await expect(page.getByTestId('success-message')).toBeVisible();
    await expect(page.getByTestId('success-message')).toContainText('Guardrail Tripped');
    await expect(page.getByTestId('success-message')).toContainText('Stage 2 (Permission) tripped');
  });

  test('Anthropic 3-Stage Tool Gating - Stage 3 Confirmation Blocked', async ({ page }) => {
    await page.goto('/guardrails');

    // Trust project
    await page.locator('input[type="checkbox"]').check();

    // Fill in tools
    await page.locator('input[placeholder="Comma separated tools, e.g. read_file, execute_bash"]').fill('read_file, execute_bash, delete_database');
    await page.locator('input[placeholder="Comma separated tools, e.g. delete_database"]').fill('delete_database');
    await page.locator('input[placeholder="e.g. execute_bash"]').fill('delete_database');

    // Run
    await page.getByRole('button', { name: 'Test Guardrails' }).click();

    // Verify Blocked
    await expect(page.getByTestId('success-message')).toBeVisible();
    await expect(page.getByTestId('success-message')).toContainText('Guardrail Tripped');
    await expect(page.getByTestId('success-message')).toContainText('Stage 3 (Confirmation) tripped');
  });
});
