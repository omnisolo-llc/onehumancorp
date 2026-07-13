import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('current app smoke test', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'team_chat');
});

test.describe('Team Chat E2E', () => {
  test('should load team chat page and display initial greeting from OHC', async ({ page }) => {
    await page.goto('/team/chat');
    await expect(page.locator('h1', { hasText: 'Team Chat' })).toBeVisible();
    await expect(page.locator('text=I\'m your central team interface')).toBeVisible();
  });

  test('should send a message and display user message in chat', async ({ page }) => {
    await page.goto('/team/chat');
    const input = page.getByTestId('team-chat-input');
    await input.fill('Hello team, I need a new action card.');
    await page.getByTestId('team-chat-send').click();
    await expect(page.locator('text=Hello team, I need a new action card.')).toBeVisible();
  });

  test('should receive an action card draft response from system', async ({ page }) => {
    await page.goto('/team/chat');
    const input = page.getByTestId('team-chat-input');
    await input.fill('Draft an action card for testing.');
    await page.getByTestId('team-chat-send').click();
    await expect(page.getByTestId('action-card')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text=Needs Approval')).toBeVisible();
  });

  test('should approve the pending action card', async ({ page }) => {
    await page.goto('/team/chat');
    const input = page.getByTestId('team-chat-input');
    await input.fill('Draft an action card for testing.');
    await page.getByTestId('team-chat-send').click();
    await expect(page.getByTestId('action-card')).toBeVisible({ timeout: 10000 });
    const approveBtn = page.getByTestId('approve-action-btn');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();
    await expect(page.locator('text=Approved')).toBeVisible();
  });

  test('should navigate back to the team page when clicking back button', async ({ page }) => {
    await page.goto('/team/chat');
    // Using aria-label or just clicking the button with the svg inside. It pushes to /team
    const backButton = page.getByLabel('Back to Team');
    await backButton.click();
    await expect(page).toHaveURL(/\/team$/);
  });
});
