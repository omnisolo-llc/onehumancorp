import { currentAppSmoke } from './current_app_smoke';
import { expect, test } from './fixtures';

currentAppSmoke('dashboard_ux');

test('dashboard displays proactive ambassador action card', async ({ page }) => {
  await page.goto('/dashboard');

  // Wait for the UnifiedAgentFeed to load
  const heading = page.locator('h2', { hasText: 'Agent Proposals' });
  await expect(heading).toBeVisible({ timeout: 15000 });

  // Locate the Proactive Ambassador card
  const ambassadorLabel = page.locator('span', { hasText: 'The Ambassador' });
  await expect(ambassadorLabel).toBeVisible({ timeout: 10000 });

  // Check context
  const contextHeader = page.locator('h3', { hasText: 'Recover 3 abandoned carts?' });
  await expect(contextHeader).toBeVisible();

  const contextBody = page.locator('p', { hasText: 'You have 3 abandoned carts totaling $120. Send them a 10% discount to recover?' });
  await expect(contextBody).toBeVisible();

  // Check draft message
  const draftMessage = page.locator('div', { hasText: '"Hi! We noticed you left some items in your cart. Here is a 10% discount: RECOVER10."' });
  await expect(draftMessage).toBeVisible();

  // Check buttons
  const declineButton = page.locator('button', { hasText: 'Decline' }).first();
  await expect(declineButton).toBeVisible();

  const approveButton = page.locator('button', { hasText: 'Approve' }).first();
  await expect(approveButton).toBeVisible();

  // Click Approve
  await approveButton.click();

  // Verify the card disappears (optimistic UI update)
  await expect(contextHeader).not.toBeVisible();
});
