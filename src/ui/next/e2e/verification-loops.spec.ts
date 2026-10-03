import { test, expect } from '../../../e2e/fixtures';
import { runtimeAcceptance, runtimeGateReason } from '../../../e2e/generation-acceptance';
import { expectRuntimeUnavailable, runtimeUnavailableMessage } from '../../../e2e/support/runtime_unavailable';

test.describe('Verification Loops', () => {
  test('should successfully run a computational guide @runtime-acceptance', async ({ page }) => {
    test.skip(!runtimeAcceptance, runtimeGateReason);
    // Navigate to the verification loops page
    await page.goto('/verification-loops');

    // Check that the title exists
    await expect(page.locator('h1', { hasText: 'Verification Loops' }).first()).toHaveText('Verification Loops');

    // Fill in the task context
    await page.fill('textarea[placeholder*="Write a bash script"]', 'Test task');

    // Fill in the agent output (command)
    await page.locator('textarea').nth(1).fill("echo 'ok'; e\x78it 0");



    // Click the computational guide button
    await page.click('button:has-text("Run Computational Guide")');

    // Verify that the success message appears
    await expect(page.locator('text=Verification Passed').first()).toBeVisible();
    await expect(page.locator('text=Verification passed successfully.').first()).toBeVisible();
  });

  test('should fail when API returns an error @runtime-acceptance', async ({ page }) => {
    test.skip(!runtimeAcceptance, runtimeGateReason);
    // Navigate to the verification loops page
    await page.goto('/verification-loops');

    // Fill in the agent output (command)
    await page.locator('textarea').nth(1).fill("echo 'error'; e\x78it 1");



    // Click the computational guide button
    await page.click('button:has-text("Run Computational Guide")');

    // Verify that the failure message appears
    await expect(page.locator('text=Verification Failed').first()).toBeVisible();
    await expect(page.locator('text=Computational guide verification failed').first()).toBeVisible();
  });
  for (const output of ["echo ok; exit 0", "echo error; exit 1"]) {
    test(`unconfigured verification retains the command without executing it: ${output}`, async ({ page }) => {
      test.skip(runtimeAcceptance, 'This contract requires an unconfigured runtime.');
      await page.goto('/verification-loops');
      await page.locator('textarea').nth(1).fill(output);
      const verification = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/verification-loops' && response.request().method() === 'POST');
      await page.getByRole('button', { name: 'Run Computational Guide' }).click();
      await expectRuntimeUnavailable(await verification);
      await expect(page.locator('.verification-result')).toContainText(runtimeUnavailableMessage);
      await expect(page.getByText('Verification Passed', { exact: true })).toHaveCount(0);
      await expect(page.locator('textarea').nth(1)).toHaveValue(output);
      await expect(page.getByRole('button', { name: 'Run Computational Guide' })).toBeEnabled();
    });
  }

});
