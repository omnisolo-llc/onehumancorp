import { expect, test } from '../../../../e2e/fixtures';
import { db } from '../../../../e2e/db_utils';
import { seedFeedItem } from '../../../../e2e/feed-fixtures';

test.describe('Unified Agent Feed Interactive Flow', () => {
  test.use({ viewport: { width: 375, height: 812 } });
  let itemId: string;
  let description: string;

  test.beforeEach(async ({ page }, testInfo) => {
    await page.goto('/feed');
    description = `Owner proposal ${testInfo.testId}`;
    itemId = await seedFeedItem(page, {
      event_source: 'operations',
      context_payload: { description, feature_type: testInfo.testId },
      proposed_action: { message: 'Review owner work', feature_type: testInfo.testId },
    });
  });

  test('should render properly, expand for details, and show approval transition', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard', exact: true })).toBeVisible();
    expect(await page.evaluate(() => document.body.scrollWidth)).toBeLessThanOrEqual(375);
    for (const button of await page.locator('button').all()) {
      if (await button.isVisible()) {
        const box = await button.boundingBox();
        expect(box).not.toBeNull();
        expect(box!.width).toBeGreaterThanOrEqual(44);
        expect(box!.height).toBeGreaterThanOrEqual(44);
      }
    }
    const card = page.getByTestId(`triage-card-${itemId}`);
    await expect(card).toBeVisible();
    await card.getByTestId('edit-proposal').click();
    await expect(card.getByTestId('edit-proposal-textarea')).toBeVisible();
    await card.getByTestId('feed-approve-btn').click();
    await expect(card).toHaveClass(/border-green-500/);
    await expect(card).toHaveClass(/scale-95/);
    await expect(card).toBeHidden({ timeout: 2000 });
  });

  test('should queue actions optimistically when offline', async ({ page, context, adminUser }) => {
    await page.goto('/dashboard');
    const card = page.getByTestId(`triage-card-${itemId}`);
    await expect(card).toBeVisible();
    await expect(page.getByTestId('offline-queue-readiness')).toHaveAttribute('data-state', 'ready');
    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));
    await expect(page.getByText('You are offline. Actions will sync when online.')).toBeVisible();
    await card.getByTestId('feed-approve-btn').click();
    await expect(card).toBeHidden({ timeout: 2000 });
    const synced = page.waitForResponse(response => response.url().endsWith(`/api/v1/agent-feed/${itemId}`) && response.request().method() === 'PUT');
    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));
    expect((await synced).status()).toBe(200);
    await expect(async () => {
      const rows = await db.query('SELECT lifecycle_state FROM agent_feed_items WHERE id = $1 AND tenant_id = $2', [itemId, adminUser.organizationId]);
      expect(rows).toHaveLength(1);
      expect(rows[0].lifecycle_state).toBe('APPROVED');
    }).toPass({ timeout: 15000 });
    await expect(page.getByText('You are offline. Actions will sync when online.')).toBeHidden();
  });

  test('Feed Page should load items and approve', async ({ page }) => {
    await page.goto('/feed');
    await expect(page.getByTestId('agent-feed')).toBeVisible();
    const card = page.getByTestId('agent-feed-card').filter({ hasText: description });
    await expect(card).toBeVisible();
    await card.getByRole('button', { name: 'Approve', exact: true }).click();
    await expect(card).toBeHidden();
  });

  test('Feed Page should load items and dismiss', async ({ page }) => {
    await page.goto('/feed');
    await expect(page.getByTestId('agent-feed')).toBeVisible();
    const card = page.getByTestId('agent-feed-card').filter({ hasText: description });
    await expect(card).toBeVisible();
    await card.getByRole('button', { name: 'Dismiss', exact: true }).click();
    await expect(card).toBeHidden();
  });

  test('Dashboard should have functional UnifiedAgentFeed component', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard', exact: true })).toBeVisible();
    await expect(page.getByRole('region', { name: 'Unified Agent Feed' })).toBeVisible();
    await expect(page.getByTestId(`triage-card-${itemId}`)).toContainText(description);
  });

  test('should handle inline editing of a proposal', async ({ page }) => {
    await page.goto('/dashboard');
    const card = page.getByTestId(`triage-card-${itemId}`);
    await expect(card).toBeVisible();
    await card.getByTestId('edit-proposal').click();
    const textarea = card.getByTestId('edit-proposal-textarea');
    await expect(textarea).toBeVisible();
    await textarea.fill('This is my manually edited draft text');
    await card.getByTestId('cancel-edit-proposal').click();
    await expect(textarea).toBeHidden();
    await card.getByTestId('edit-proposal').click();
    await textarea.fill('Second edited text');
    await card.getByTestId('save-proposal').click();
    await expect(textarea).toBeHidden({ timeout: 2000 });
    await expect(card).toBeHidden({ timeout: 2000 });
  });

  test('should handle inline editing of an ambassador reply', async ({ page }) => {
    await seedFeedItem(page, {
      event_source: 'Ambassador',
      context_payload: { feature_type: 'ambassador_reply', original_message: description, source: 'SMS' },
      proposed_action: { feature_type: 'ambassador_reply', generated_response: 'Original ambassador reply' },
    });
    // The full feed owns the Ambassador card's supported edit controls.
    await page.goto('/feed');
    const card = page.getByTestId('ambassador-reply-card').filter({ hasText: description });
    await expect(card).toBeVisible();
    await card.getByTestId('feed-edit-btn').click();
    const textarea = card.getByTestId('feed-edit-input');
    await expect(textarea).toBeVisible();
    await textarea.fill('Edited ambassador reply');
    await card.getByTestId('feed-save-edit-btn').click();
    await expect(textarea).toBeHidden({ timeout: 2000 });
    await expect(card).toBeHidden({ timeout: 2000 });
  });
});
