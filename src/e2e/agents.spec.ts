import { expect, test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('agents');

test.describe('Invisible AI Agents - Automations UI', () => {
  test.beforeEach(async ({ page }) => {
    // 1. Arrange: Go to the agents page.
    await page.goto('/agents');
    // 2. Act: Click on Automations tab
    await page.getByRole('button', { name: 'Automations' }).click();
  });

  test('should display the Automations tab with the correct headings', async ({ page }) => {
    // Assert: Verify all 3 required features are present
    await expect(page.getByRole('heading', { name: 'Autonomous Social Media Agent' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'DM Auto-Responder' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Weekly Push Notification Insight' })).toBeVisible();
  });

  test('should enable Autonomous Social Media Agent and show confirmation', async ({ page }) => {
    // Act: Enable the social media agent
    const button = page.locator('div').filter({ hasText: /^Autonomous Social Media AgentAgent detects new product addition/ }).getByRole('button', { name: 'Enable Agent' });
    await button.click();

    // Assert: Wait for it to become enabled and show the status message
    await expect(page.getByRole('button', { name: 'Disable Agent' }).first()).toBeVisible();
    await expect(page.getByText('Agent is monitoring your catalog. Approvals will appear in your Inbox.')).toBeVisible();
  });

  test('should enable DM Auto-Responder and show confirmation', async ({ page }) => {
    // Act: Enable the DM auto responder
    const button = page.locator('div').filter({ hasText: /^DM Auto-ResponderAgent connects to IG DMs/ }).getByRole('button', { name: 'Enable Agent' });
    await button.click();

    // Assert: Wait for it to become enabled and show the status message
    await expect(page.getByText('Agent is connected to DMs. Fallback messages will route to you.')).toBeVisible();
  });

  test('should enable Weekly Push Notification Insight and show confirmation', async ({ page }) => {
    // Act: Enable the weekly push notification agent
    const button = page.locator('div').filter({ hasText: /^Weekly Push Notification InsightAgent sends a weekly push notification/ }).getByRole('button', { name: 'Enable Agent' });
    await button.click();

    // Assert: Wait for it to become enabled and show the status message
    await expect(page.getByText('Agent will notify you every Monday morning.')).toBeVisible();
  });

  test('should allow disabling an enabled agent', async ({ page }) => {
    // Act: Enable then disable the social media agent
    const button = page.locator('div').filter({ hasText: /^Autonomous Social Media AgentAgent detects new product addition/ }).getByRole('button', { name: 'Enable Agent' });
    await button.click();

    // Assert it is enabled
    await expect(page.getByText('Agent is monitoring your catalog. Approvals will appear in your Inbox.')).toBeVisible();

    // Act: Disable it
    const disableButton = page.locator('div').filter({ hasText: /^Autonomous Social Media AgentAgent detects new product addition/ }).getByRole('button', { name: 'Disable Agent' });
    await disableButton.click();

    // Assert it is no longer enabled
    await expect(page.getByText('Agent is monitoring your catalog. Approvals will appear in your Inbox.')).toBeHidden();
  });
});
