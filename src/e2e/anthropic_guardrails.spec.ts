import { test, expect } from './fixtures';

test.describe('Master Catalog B.9: Anthropic 3-Stage Tool Gating UI integration', () => {
  test('CUJ: Project Trust (Stage 1) - Untrusted project blocks mutating tools but allows read-only', async ({ page, unlimitedAdminUser, loginAs }) => {
    await loginAs(page, unlimitedAdminUser);

    await page.goto('/anthropic-guardrails');

    const toggleTrust = page.locator('input[type="checkbox"]#trusted');
    if (await toggleTrust.isChecked()) {
        await toggleTrust.uncheck();
    }
    await expect(toggleTrust).not.toBeChecked();

    const toolInput = page.locator('input[type="text"]').first();
    await toolInput.fill('execute_bash');

    const checkButton = page.getByRole('button', { name: /Check Tool Guardrails/i });
    await checkButton.click();

    await expect(page.getByTestId('error-message')).toBeVisible({ timeout: 10000 });
    await expect(page.getByTestId('error-message')).toContainText(/Stage 1/i);
  });

  test('CUJ: Permission Check (Stage 2) - Trusted project blocks unallowed tools', async ({ page, unlimitedAdminUser, loginAs }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/anthropic-guardrails');

    const toggleTrust = page.locator('input[type="checkbox"]#trusted');
    if (!(await toggleTrust.isChecked())) {
        await toggleTrust.check();
    }
    await expect(toggleTrust).toBeChecked();

    const inputs = page.locator('input[type="text"]');
    const toolInput = inputs.nth(0);
    const sessionAllowedToolsInput = inputs.nth(1);

    await toolInput.fill('execute_bash');
    await sessionAllowedToolsInput.fill('read_file, list_files');

    const checkButton = page.getByRole('button', { name: /Check Tool Guardrails/i });
    await checkButton.click();

    await expect(page.getByTestId('error-message')).toBeVisible({ timeout: 10000 });
    await expect(page.getByTestId('error-message')).toContainText(/Stage 2/i);
  });

  test('CUJ: High-Risk Explicit Confirmation (Stage 3) - User must explicitly approve', async ({ page, unlimitedAdminUser, loginAs }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/anthropic-guardrails');

    const toggleTrust = page.locator('input[type="checkbox"]#trusted');
    if (!(await toggleTrust.isChecked())) {
        await toggleTrust.check();
    }
    await expect(toggleTrust).toBeChecked();

    const inputs = page.locator('input[type="text"]');
    const toolInput = inputs.nth(0);
    const sessionAllowedToolsInput = inputs.nth(1);
    const highRiskInput = inputs.nth(2);

    await toolInput.fill('execute_bash');
    await sessionAllowedToolsInput.fill('read_file, list_files, execute_bash');
    await highRiskInput.fill('execute_bash');

    const checkButton = page.getByRole('button', { name: /Check Tool Guardrails/i });
    await checkButton.click();

    await expect(page.getByTestId('error-message')).toBeVisible({ timeout: 10000 });
    await expect(page.getByTestId('error-message')).toContainText(/Stage 3/i);
  });

  test('CUJ: Passes all checks', async ({ page, unlimitedAdminUser, loginAs }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/anthropic-guardrails');

    const toggleTrust = page.locator('input[type="checkbox"]#trusted');
    if (!(await toggleTrust.isChecked())) {
        await toggleTrust.check();
    }
    await expect(toggleTrust).toBeChecked();

    const inputs = page.locator('input[type="text"]');
    const toolInput = inputs.nth(0);
    const sessionAllowedToolsInput = inputs.nth(1);
    const highRiskInput = inputs.nth(2);

    await toolInput.fill('read_file');
    await sessionAllowedToolsInput.fill('read_file, list_files, execute_bash');
    await highRiskInput.fill('execute_bash');

    const checkButton = page.getByRole('button', { name: /Check Tool Guardrails/i });
    await checkButton.click();

    await expect(page.getByTestId('success-message')).toBeVisible({ timeout: 10000 });
    await expect(page.getByTestId('success-message')).toContainText(/Validation passed successfully/i);
  });
});
