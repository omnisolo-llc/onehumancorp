import { test, expect } from '../../../e2e/fixtures';
import { runtimeAcceptance, runtimeGateReason } from '../../../e2e/generation-acceptance';
import { expectRuntimeUnavailable, runtimeUnavailableMessage } from '../../../e2e/support/runtime_unavailable';

test.describe('Expert Team', () => {
  test('unconfigured expert team retains the task without inventing a delivered report', async ({ page }) => {
    test.skip(runtimeAcceptance, 'This contract requires an unconfigured runtime.');
    // Navigate to the expert team page
    await page.goto('/expert-team');

    // Check that the title exists
    await expect(page.locator('h1', { hasText: 'Collaborative Expert Team' }).first()).toHaveText('Collaborative Expert Team');

    // Fill in the task context
    await page.fill('textarea[placeholder*="Write a comprehensive business plan"]', 'Write a comprehensive business plan for a new AI startup. Chart: Required. Analysis: Deep. Chapter 1, Chapter 2, Chapter 3, Chapter 4, Chapter 5, Chapter 6, Chapter 7, Chapter 8');

    // Click the execute button
    const execution = page.waitForResponse(response => response.url().endsWith('/api/v1/expert-team') && response.request().method() === 'POST');
    await page.click('button:has-text("Execute Task via Expert Team")');

    // Verify that the loading state appears
    await expect(page.locator('button:has-text("Orchestrating Expert Team...")')).toBeVisible();

    await expectRuntimeUnavailable(await execution);
    await expect(page.locator('.expert-error-content')).toContainText(runtimeUnavailableMessage);
    await expect(page.locator('textarea')).toHaveValue(/Write a comprehensive business plan/);
    await expect(page.getByRole('heading', { name: 'Final Delivered Output' })).toHaveCount(0);
    await expect(page.getByRole('button', { name: 'Execute Task via Expert Team' })).toBeEnabled();
  });

  test('short tasks still report the exact missing runtime', async ({ page }) => {
    test.skip(runtimeAcceptance, 'This contract requires an unconfigured runtime.');
    await page.goto('/expert-team');

    // Fill in the task context that will fail a quality gate (e.g., missing chapters or short)
    await page.fill('textarea[placeholder*="Write a comprehensive business plan"]', 'Short task');

    const execution = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/expert-team' && response.request().method() === 'POST');
    await page.click('button:has-text("Execute Task via Expert Team")');
    await expectRuntimeUnavailable(await execution);
    await expect(page.locator('.expert-error-content')).toContainText(runtimeUnavailableMessage);

    // Wait for the error message
    await expect(page.locator('h3:has-text("Quality Gate or Execution Error:")')).toBeVisible({ timeout: 15000 });
  });
  test('authorized expert runtime produces a genuinely delivered report @runtime-acceptance', async ({ page }) => {
    test.skip(!runtimeAcceptance, runtimeGateReason);
    await page.goto('/expert-team');
    await page.locator('textarea').fill('Write a business plan for a neighborhood bakery. Include distinct expert analysis, an executive summary, market assumptions, operations, risks, and a clearly labeled budget estimate.');
    const execution = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/expert-team' && response.request().method() === 'POST');
    await page.getByRole('button', { name: 'Execute Task via Expert Team' }).click();
    const response = await execution;
    expect(response.status()).toBe(200);
    const result = await response.json();
    expect(typeof result.result).toBe('string');
    expect(result.result.trim().length).toBeGreaterThan(0);
    expect(result.error).toBeUndefined();
    await expect(page.getByRole('heading', { name: 'Final Delivered Output' })).toBeVisible();
    await expect(page.locator('pre')).toHaveText(result.result);
    await expect(page.locator('.expert-error-content')).toHaveCount(0);
  });

});
