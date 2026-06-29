import { test, expect } from '@playwright/test';

test.describe('Anthropic Guardrails UI', () => {
  test('Owner can verify 3-stage tool gating', async ({ page }) => {
    test.setTimeout(60000);
    await page.goto('/anthropic-guardrails');
    await expect(page.getByRole('heading', { name: 'Anthropic 3-Stage Tool Gating' })).toBeVisible();

    // Stage 1 fail
    await page.getByRole('button', { name: 'Check Tool Guardrails' }).click();
    await expect(page.getByTestId('error-message')).toContainText('Anthropic Guardrail Stage 1 (Trust) tripped');

    // Trust it
    await page.locator('#trusted').check();

    // Stage 2 fail (execute_bash not in allowed tools by default)
    await page.getByRole('button', { name: 'Check Tool Guardrails' }).click();
    await expect(page.getByTestId('error-message')).toContainText('Anthropic Guardrail Stage 2 (Permission) tripped');

    // Add to allowed
    await page.getByLabel('Session Allowed Tools').fill('read_file, list_files, execute_bash');

    // Stage 3 fail (execute_bash is in high risk)
    await page.getByRole('button', { name: 'Check Tool Guardrails' }).click();
    await expect(page.getByTestId('error-message')).toContainText('Anthropic Guardrail Stage 3 (Confirmation) tripped');

    // Remove from high risk
    await page.getByLabel('High Risk Tools').fill('delete_database');

    // Pass
    await page.getByRole('button', { name: 'Check Tool Guardrails' }).click();
    await expect(page.getByTestId('success-message')).toContainText('Guardrails Passed');
  });
});
