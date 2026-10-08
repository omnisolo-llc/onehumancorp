import { test, expect } from '../../../../e2e/fixtures';
import { runtimeAcceptance, runtimeGateReason } from '../../../../e2e/generation-acceptance';
import { expectRuntimeUnavailable, runtimeUnavailableMessage } from '../../../../e2e/support/runtime_unavailable';

test.describe('LangGraph State Machine', () => {
  test('should execute a task via LangGraph mechanics and trigger tool node routing from sidebar @runtime-acceptance', async ({ page }) => {
    test.skip(!runtimeAcceptance, runtimeGateReason);
    test.setTimeout(180000);
    // Navigate to the dashboard (home page)
    await page.goto('/dashboard');

    // Click the LangGraph link in the sidebar navigation
    await page.click('a[href="/langgraph"]');

    // Check that the title exists
    await expect(page.getByRole('heading', { name: 'LangGraph State Machine', exact: true })).toBeVisible();

    // Instruct the agent to use the Bash tool, proving that it routes to tool_node and back
    await page.fill('textarea[placeholder*="Write a quick poem about a cake"]', 'Use the Bash tool to execute "echo hello". Then confirm you have done so.');

    // Click the execute button
    await page.click('button:has-text("Run LangGraph")');

    // Verify that the success message appears
    await expect(page.locator('h2:has-text("LangGraph Output")')).toBeVisible({ timeout: 120000 });

    // Verify the text content indicates that the tool was actually used and completed
    const resultText = await page.locator('pre').textContent();
    expect(resultText?.toLowerCase()).toContain('hello');
  });
  test('unconfigured runtime retains the LangGraph prompt without claiming tool execution', async ({ page }) => {
    test.skip(runtimeAcceptance, 'This contract requires an unconfigured runtime.');
    await page.goto('/dashboard');
    await page.locator('a[href="/langgraph"]').click();
    const prompt = 'Use the Bash tool to execute "echo hello". Then confirm you have done so.';
    await page.locator('#message').fill(prompt);
    const execution = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/agents/langgraph' && response.request().method() === 'POST');
    await page.getByRole('button', { name: 'Run LangGraph' }).click();
    await expectRuntimeUnavailable(await execution);
    await expect(page.getByTestId('error-message')).toContainText(runtimeUnavailableMessage);
    await expect(page.getByTestId('success-message')).toHaveCount(0);
    await expect(page.locator('#message')).toHaveValue(prompt);
    await expect(page.getByRole('button', { name: 'Run LangGraph' })).toBeEnabled({ timeout: 15000 });
  });

});
