import { test, expect } from './fixtures';
import { judgeGeneratedOutput } from './ai-judge';

test.describe('E2E Chaos Resilience', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('keeps the website publishing flow usable', async ({ page }) => {
    await page.getByRole('button', { name: 'Edit Website' }).click();
    await expect(page.getByRole('heading', { name: 'Edit Website' })).toBeVisible();

    await page.getByRole('button', { name: 'Publish Changes' }).click();
    await expect(page.getByRole('heading', { name: 'Publish Site' })).toBeVisible();

    await page.getByRole('button', { name: /Free OHC Subdomain/ }).click();
    await page.locator('#free-domain-input').fill('chaos-test');
    await page.getByRole('button', { name: 'Publish', exact: true }).click();

    await expect(page.getByText('Welcome back, Human.')).toBeVisible({ timeout: 5000 });
  });

  test('keeps dashboard and inbox interactions responsive', async ({ page }, testInfo) => {

    await expect(page.getByText("Today's Sales")).toBeVisible();
    await page.getByRole('button', { name: 'Check Messages' }).click();

    await expect(page.getByRole('heading', { name: 'Customer Inbox' })).toBeVisible();
    await page.getByRole('button', { name: /AI Draft/ }).click();
    await expect(page.locator('#reply-input')).not.toHaveValue('');
    const draft = await page.locator('#reply-input').inputValue();
    await judgeGeneratedOutput(testInfo, {
      output: draft,
      rubric: 'The reply must directly answer that vegan birthday cake options are available, sound helpful and professional, avoid making unsupported promises, and be ready to send to a customer.',
    });

    await page.getByRole('button', { name: 'Send' }).click();
    await expect(page.locator('#messages-list')).toContainText(draft);
  });

  test('keeps the agents page functional after navigation', async ({ page }) => {
    await page.getByRole('button', { name: 'My AI Assistants' }).click();

    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
    await expect(page.getByText('Marketing Pro')).toBeVisible();
    await expect(page.getByText('Status: Active').first()).toBeVisible();
    await expect(page.getByRole('button', { name: 'Hire Agent' })).toBeVisible();
  });

  test('isolates page-local state across concurrent dashboard pages', async ({ page, context }) => {
    await page.getByRole('button', { name: 'Check Messages' }).click();
    await page.locator('#reply-input').fill('Tenant one draft');

    const page2 = await context.newPage();
    await page2.goto('/');
    await page2.getByRole('button', { name: 'Check Messages' }).click();

    await expect(page2.locator('#reply-input')).toHaveValue('');
    await expect(page.locator('#reply-input')).toHaveValue('Tenant one draft');
  });

  test('keeps settings reachable from the dashboard', async ({ page }) => {
    await page.getByRole('button', { name: 'Settings' }).first().click();

    await expect(page.getByRole('heading', { name: 'Settings' })).toBeVisible();
    await expect(page.getByText('Enable Email Notifications')).toBeVisible();
    await expect(page.getByText('Timezone')).toBeVisible();
  });
});
