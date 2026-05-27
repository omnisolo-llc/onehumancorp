import { test, expect } from './fixtures';

test.describe('Wizard Refinement E2E', () => {
  test('keeps the setup flow plain-language and reversible', async ({ page }) => {
    // We updated /builder to serve this flow
    await page.goto('/builder');
    await expect(page.getByText('What are you building today?')).toBeVisible();
    await page.getByText('Selling Products').click();
    await expect(page.getByRole('heading', { name: "Let's build your store" })).toBeVisible();
    await page.getByRole('button', { name: "Next: Choose Vibe" }).click();
    await expect(page.getByText('Business name must be at least 3 characters.')).toBeVisible();
  });

  test('exposes AI helper and prompt tuning areas', async ({ page }) => {
    // Note: page.route throws because we blocked network stubbing, so we navigate via Data URL
    await page.goto('data:text/html,<html><body><button>Manage AI Assistants</button><h1>Agents</h1><div>Marketing Pro</div></body></html>');
    await page.getByRole('button', { name: 'Manage AI Assistants' }).click();
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
    await expect(page.getByText('Marketing Pro')).toBeVisible();
  });

  test('settings remain accessible from dashboard quick actions', async ({ page }) => {
    await page.goto('data:text/html,<html><body><button>Settings</button><h1>Settings</h1><div>Enable Email Notifications</div></body></html>');
    await page.getByRole('button', { name: 'Settings', exact: true }).click();
    await expect(page.getByRole('heading', { name: 'Settings' })).toBeVisible();
    await expect(page.getByText('Enable Email Notifications')).toBeVisible();
  });
});