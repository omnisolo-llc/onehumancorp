import { test, expect } from './fixtures';
import { runtimeAcceptance, runtimeGateReason } from './generation-acceptance';
import { expectRuntimeUnavailable, runtimeUnavailableMessage } from './support/runtime_unavailable';

test.describe('Scalable multi-agent deployment', () => {
  test('user can adjust scale and deploy a fleet of agents @runtime-acceptance', async ({ page, unlimitedAdminUser, loginAs }) => {
    test.skip(!runtimeAcceptance, runtimeGateReason);
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/scaling');

    // Wait for the UI to load
    await expect(page.locator('#scaling-screen')).toBeVisible();

    // Verify initial state
    await expect(page.locator('text=3 agents')).toBeVisible();

    // Increase scale
    await page.getByRole('button', { name: 'Increase Scale (+1)' }).click();
    await expect(page.locator('text=4 agents')).toBeVisible();

    // Max scale
    await page.getByRole('button', { name: 'Max Scale (1000)' }).click();
    await expect(page.getByText('1000 agents', { exact: true })).toBeVisible();
    await expect(page.locator('text=Cloud Distributed')).toBeVisible();

    // Decrease back down a bit for the test to run quickly
    await page.getByRole('button', { name: 'Decrease Scale' }).click();
    // Now it should be 999
    await expect(page.locator('text=999 agents')).toBeVisible();

    // Set text
    await page.fill('input[placeholder="e.g. Analyze dataset"]', 'Test Scalable Task');

    // Run deployment
    await page.locator('#deploy-agents-btn').click();

    // Wait for results
    await expect(page.locator('h3:has-text("Results")')).toBeVisible({ timeout: 15000 });
    await expect(page.locator('h3:has-text("Results (999 outputs)")')).toBeVisible();

    // Verify some outputs are shown
    await expect(page.locator('text=Agent 1:').first()).toBeVisible();
    await expect(page.locator('text=and 979 more results not shown.')).toBeVisible();
  });
  test('scale controls retain the requested fleet when the runtime is unavailable', async ({ page, unlimitedAdminUser, loginAs }) => {
    test.skip(runtimeAcceptance, 'This contract requires an unconfigured runtime.');
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/scaling');
    await page.getByRole('button', { name: 'Increase Scale (+1)' }).click();
    await expect(page.getByText('4 agents', { exact: true })).toBeVisible();
    await page.getByRole('button', { name: 'Max Scale (1000)' }).click();
    await expect(page.getByText('1000 agents', { exact: true })).toBeVisible();
    await page.getByRole('button', { name: 'Decrease Scale', exact: true }).click();
    await expect(page.getByText('999 agents', { exact: true })).toBeVisible();
    await page.getByPlaceholder('e.g. Analyze dataset').fill('Retained task');
    const execution = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/scaling' && response.request().method() === 'POST');
    await page.locator('#deploy-agents-btn').click();
    const response = await execution;
    expect(response.request().postDataJSON()).toEqual({ count: 999, message: 'Retained task' });
    await expectRuntimeUnavailable(response);
    await expect(page.getByRole('alert')).toHaveText(runtimeUnavailableMessage);
    await expect(page.getByRole('heading', { name: /Results/ })).toHaveCount(0);
    await expect(page.getByText('Agent 1:', { exact: true })).toHaveCount(0);
    await expect(page.getByPlaceholder('e.g. Analyze dataset')).toHaveValue('Retained task');
    await expect(page.locator('#deploy-agents-btn')).toBeEnabled();
  });

});
