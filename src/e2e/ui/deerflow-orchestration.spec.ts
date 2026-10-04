import { test, expect } from '@playwright/test';
import { observeButtonStates } from '../../../scripts/playwright/button-states.mjs';

test.describe('DeerFlow Sub-agent Orchestration UI', () => {
  test('should allow a user to submit a task and see the orchestrated result via real API', async ({ page }) => {
    // Navigate to the DeerFlow Orchestration page
    await page.goto('/deerflow-orchestration');

    const header = page.getByRole('heading', { name: 'DeerFlow Sub-agent Orchestration' });
    await expect(header).toBeVisible({ timeout: 10000 });

    await expect(page.getByText('Lead agent decomposes tasks, spawns parallel sub-agents, synthesizes results.')).toBeVisible();

    const executeButton = page.getByRole('button', { name: 'Execute Task via DeerFlow' });
    await expect(executeButton).toBeDisabled();

    const taskTextarea = page.getByPlaceholder('e.g. Analyze the current AI market, compare the top 3 frameworks, and synthesize a recommendation report.');
    // Keep the task extremely simple so the real backend returns quickly
    const task = 'Say exactly "DeerFlow Test Succeeded"';
    await taskTextarea.fill(task);

    await expect(executeButton).toBeEnabled();

    // A real response can settle before click() returns. Record the actual busy
    // transition before submitting, without delaying or replacing the response.
    const endpoint = new URL('/api/v1/deerflow/run', page.url()).href;
    const observation = await executeButton.evaluateHandle(observeButtonStates);
    try {
      const [request] = await Promise.all([
        page.waitForRequest(request => request.method() === 'POST'
          && request.url() === endpoint),
        executeButton.click(),
      ]);
      expect(request.postDataJSON()).toEqual({ task });
      const response = await request.response();
      expect(response, 'the submitted task must receive a real API response').not.toBeNull();
      const data = await response!.json();

      expect(await observation.evaluate(value => value.states)).toContainEqual({
        disabled: true, text: 'Orchestrating Sub-agents...',
      });

      // Preserve the real stack's outcome, including a missing agent runtime.
      // The visible text must come from that response, not merely any alert.
      if (response!.ok()) {
        expect(typeof data.result).toBe('string');
        expect(data.result.trim()).not.toBe('');
        await expect(page.locator('.whitespace-pre-wrap')).toHaveText(data.result, { timeout: 30000 });
        await expect(page.locator('.bg-red-50')).not.toBeVisible();
      } else {
        expect(typeof data.error).toBe('string');
        expect(data.error.trim()).not.toBe('');
        await expect(page.locator('.bg-red-50')).toHaveText(data.error, { timeout: 30000 });
        await expect(page.locator('.whitespace-pre-wrap')).not.toBeVisible();
      }
      await expect(executeButton).toBeEnabled();
      await expect(taskTextarea).toBeEnabled();
    } finally {
      await observation.evaluate(value => value.disconnect());
      await observation.dispose();
    }
  });
});
