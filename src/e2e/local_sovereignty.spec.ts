import { type Page } from '@playwright/test';
import { test, expect } from './fixtures';

type TelemetryPolicy = {
  product_telemetry_enabled: boolean;
  effective_enabled: boolean;
  operator_enforced: boolean;
  can_change: boolean;
  change_block_reason: string | null;
};
const telemetryUrl = '/api/v1/settings/telemetry';
const toggleName = 'Enable Product Telemetry (Standalone Mode)';

async function readPolicy(page: Page): Promise<TelemetryPolicy> {
  const response = await page.request.get(telemetryUrl);
  expect(response.status()).toBe(200);
  const policy = await response.json() as TelemetryPolicy;
  for (const key of ['product_telemetry_enabled', 'effective_enabled', 'operator_enforced', 'can_change'] as const) {
    expect(typeof policy[key]).toBe('boolean');
  }
  expect(policy.can_change).toBe(false);
  expect(policy.change_block_reason).toBe('hosted_global_control_unavailable');
  return policy;
}

async function expectDisplayedPolicy(page: Page, policy: TelemetryPolicy) {
  const checkbox = page.getByRole('checkbox', { name: toggleName, exact: true });
  await expect(page.getByText(`Effective telemetry: ${policy.effective_enabled ? 'On' : 'Off'}`, { exact: true })).toBeVisible();
  await expect(checkbox).toBeDisabled();
  await expect(checkbox).toBeChecked({ checked: policy.effective_enabled });
  await expect(page.getByText(policy.operator_enforced
    ? 'Telemetry is enabled by server configuration and cannot be changed here.'
    : 'Telemetry is controlled by the server operator in hosted mode.', { exact: true })).toBeVisible();
}

test.describe('Hosted telemetry authority boundary', () => {
  test.beforeEach(async ({ page, loginAs, adminUser }) => {
    // A tenant administrator is not the instance operator.
    await loginAs(page, adminUser);
    await page.goto('/settings');
    await expect(page.getByRole('heading', { name: 'Workspace Settings', exact: true })).toBeVisible();
  });

  test('shows the privacy controls and their standalone-only description', async ({ page }) => {
    await expect(page.getByText('Local Sovereignty & Data Sharing', { exact: true })).toBeVisible();
    await expect(page.getByText('Control your privacy and telemetry in Standalone Mode.', { exact: true })).toBeVisible();
    await expect(page.getByText('Shares anonymous usage data to help us improve OmniSolo. Explicit opt-in required for Standalone Mode.', { exact: true })).toBeVisible();
  });

  test('displays actual effective policy without offering a hosted write', async ({ page }) => {
    const policy = await readPolicy(page);
    await expectDisplayedPolicy(page, policy);
    await expect(page.getByText('Telemetry preference saved.', { exact: true })).toHaveCount(0);
  });

  for (const enabled of [true, false]) {
    test(`rejects hosted tenant telemetry ${enabled ? 'opt-in' : 'opt-out'} without changing state`, async ({ page }) => {
      const before = await readPolicy(page);
      // A browser mutation supplies the same-origin metadata required by CSRF
      // middleware, so this reaches the hosted operator policy under test.
      const response = await page.evaluate(async ({ url, enabled }) => {
        const response = await fetch(url, {
          method: 'POST', credentials: 'same-origin',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ product_telemetry_enabled: enabled }),
        });
        return { status: response.status, body: await response.json() };
      }, { url: telemetryUrl, enabled });
      expect(response.status).toBe(403);
      expect(response.body).toMatchObject({
        success: false, error: 'hosted_global_control_unavailable', ...before,
      });
      expect(await readPolicy(page)).toEqual(before);
      await expectDisplayedPolicy(page, before);
    });
  }

  test('rejects a mutation without trusted browser origin before changing policy', async ({ page }) => {
    const before = await readPolicy(page);
    const response = await page.request.post(telemetryUrl, {
      data: { product_telemetry_enabled: !before.product_telemetry_enabled },
    });
    expect(response.status()).toBe(403);
    expect(await response.json()).toEqual({ error: 'forbidden' });
    expect(await readPolicy(page)).toEqual(before);
    await expectDisplayedPolicy(page, before);
  });

  test('reloads the authoritative read-only setting without issuing a write', async ({ page }) => {
    const before = await readPolicy(page);
    const writes: string[] = [];
    page.on('request', request => {
      if (new URL(request.url()).pathname === telemetryUrl && request.method() !== 'GET') writes.push(request.method());
    });
    await page.reload();
    await expect(page.getByRole('heading', { name: 'Workspace Settings', exact: true })).toBeVisible();
    expect(await readPolicy(page)).toEqual(before);
    await expectDisplayedPolicy(page, before);
    expect(writes).toEqual([]);
  });
});
