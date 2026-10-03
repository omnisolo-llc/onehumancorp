import { test, expect } from './fixtures';
import { runtimeAcceptance, runtimeGateReason } from './generation-acceptance';

test.describe('Aider RepoMap UI', () => {
  test('unconfigured runtime explains that no repository work was dispatched', async ({ page, unlimitedAdminUser, loginAs }) => {
    test.skip(runtimeAcceptance, 'This contract requires an unconfigured repository runtime.');
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/aider.html');
    await page.getByPlaceholder('e.g. .').fill('src/agents/builtin');
    const response = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/rpc' && response.request().method() === 'POST');
    await page.getByRole('button', { name: 'Generate RepoMap' }).click();
    const result = await response;
    expect(result.status()).toBe(503);
    expect(await result.json()).toEqual({ error: 'Agent runtime is not configured; no work was dispatched' });
    await expect(page.getByRole('alert')).toHaveText('Error: Agent runtime is not configured; no work was dispatched');
    await expect(page.getByText('RepoMap Result')).toBeHidden();
    await expect(page.locator('#result-content')).toBeEmpty();
    await expect(page.getByRole('button', { name: 'Generate RepoMap' })).toBeEnabled();
  });

  test('the authorized runtime returns the actual repository map @runtime-acceptance', async ({ page, unlimitedAdminUser, loginAs }) => {
    test.skip(!runtimeAcceptance, runtimeGateReason);
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/aider.html');
    await page.getByPlaceholder('e.g. .').fill('src/agents/builtin');
    await page.getByRole('button', { name: 'Generate RepoMap' }).click();
    await expect(page.getByText('RepoMap Result')).toBeVisible({ timeout: 30_000 });
    await expect(page.locator('#result-content')).toContainText('agent.rs');
    await expect(page.locator('#result-content')).toContainText('aider_repomap.rs');
  });
});
