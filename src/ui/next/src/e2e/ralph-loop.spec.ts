import { test, expect } from '../../../../e2e/fixtures';
import { runtimeAcceptance } from '../../../../e2e/generation-acceptance';

const runtimeUnavailable = 'Agent runtime is not configured; no work was dispatched';

test.describe('The Ralph Loop UI E2E', () => {
  test('Owner submits a Ralph Loop task and sees the actual runtime response', async ({ page }) => {
    await page.goto('/ralph-loop');
    await expect(page.getByRole('heading', { name: /The Ralph Loop/ })).toBeVisible();

    const task = 'Implement an end-to-end feature spanning multiple sessions';
    const taskInput = page.getByLabel(/Long-Running Task Description/i);
    await taskInput.fill(task);
    const executeButton = page.getByRole('button', { name: /Start Ralph Loop/i });
    await expect(executeButton).toBeEnabled();

    // Observe the real response before clicking. Loading may finish before the
    // next Playwright command; its intermediate state is owned by component tests.
    const responsePromise = page.waitForResponse(response =>
      new URL(response.url()).pathname === '/api/v1/ralph-loop'
      && response.request().method() === 'POST');
    await executeButton.click();
    const response = await responsePromise;

    expect(response.request().postDataJSON()).toEqual({
      task,
      progress_file: '.ralph_progress.json',
    });
    const body = await response.json();
    if (runtimeAcceptance) {
      expect(response.status()).toBe(200);
      expect(body.error).toBeUndefined();
      expect(body.result).toBeTruthy();
      await expect(page.getByTestId('success-message')).toBeVisible();
      await expect(page.getByTestId('success-message').locator('pre'))
        .toHaveText(JSON.stringify(body.result, null, 2));
      await expect(page.getByTestId('error-message')).toHaveCount(0);
    } else {
      // The native E2E fixture owns an unconfigured workspace runtime.
      expect(response.status()).toBe(503);
      expect(body).toEqual({ error: runtimeUnavailable });
      await expect(page.getByTestId('error-message')).toBeVisible();
      await expect(page.getByTestId('error-message')).toContainText(runtimeUnavailable);
      await expect(page.getByTestId('success-message')).toHaveCount(0);
    }
    await expect(taskInput).toHaveValue(task);
    await expect(executeButton).toBeEnabled();
  });
});
