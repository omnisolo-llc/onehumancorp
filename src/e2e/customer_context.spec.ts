import { test, expect } from './fixtures';
import { randomUUID } from 'node:crypto';
import { seedFeedItem } from './feed-fixtures';

test.describe('Omnichannel Unified Customer Memory Graph UI', () => {
  // Use loginAs fixture which handles authentication
  test('displays customer context in the Ambassador Reply Card', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    const inquiry = `Vegan options inquiry ${randomUUID()}`;
    await seedFeedItem(page, {
      event_source: 'Ambassador',
      context_payload: { feature_type: 'ambassador_reply', source: 'Instagram', past_orders: 'Returning Customer (2 past orders).', context_used: 'Customer prefers vegan options.', original_message: inquiry },
      proposed_action: { feature_type: 'ambassador_reply', action_type: 'DraftForReview', generated_response: 'Yes! We have a full vegan pastry selection.' },
    }, adminUser.organizationId);

    // Navigate to dashboard
    await page.goto('/dashboard');

    // Wait for the unified agent feed to load
    await expect(page.locator('#unified-agent-feed-section')).toBeVisible();

    // Verify the ambassador reply card is rendered (seeded in e2e-seed.sql)
    const card = page.getByTestId('ambassador-reply-card').filter({ hasText: inquiry });
    await expect(card).toBeVisible({ timeout: 15000 });

    // Check if the context section is rendered
    await expect(card.locator('text=Customer Context')).toBeVisible();

    // Check past orders is rendered properly
    await expect(card.locator('text=Returning Customer (2 past orders).')).toBeVisible();

    // Check context is rendered properly
    await expect(card.locator('text=Customer prefers vegan options.')).toBeVisible();
  });
});
