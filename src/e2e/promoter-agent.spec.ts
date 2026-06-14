import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Promoter Agent Action Card', () => {
  test('should display social post draft in agent feed and allow approval', async ({ page }) => {
    // The e2e-seed.sql file already creates an agent_feed_items record with:
    // id='e2e-feed-social', tenant_id='e2e-tenant', event_source='marketing',
    // context_payload='{"feature_type": "social_post_draft", "tiktok": "Check out our new product!", "instagram": "New arrival! Link in bio.", "facebook": "We just added a new product to our store."}'

    // adminPage fixture logs in as test@example.com (e2e-tenant admin)
    // and returns the page. We just need to use it.
    await adminPage(page, async () => {
      // Navigate to the Dashboard (which contains the Agent Feed)
      await page.goto('/dashboard');

      // Wait for the Unified Agent Feed section to be visible
      const feedSection = page.locator('section[aria-label="Unified Agent Feed"], h2:has-text("Unified Agent Feed")').first();
      await feedSection.waitFor({ state: 'visible', timeout: 10000 });

      // Look for the action card that was seeded
      // The e2e-feed-social card might be hidden under a specific tab or scroll area.
      // Wait for at least one card with text related to the seeded social post.
      // It has instagram text: "New arrival! Link in bio."
      const socialCard = page.locator('div[data-testid="triage-card-e2e-feed-social"], .triage-item:has-text("New arrival! Link in bio."), .triage-item:has-text("Check out our new product!")').first();

      // Sometimes it is inside the tauri-style UI, sometimes Next.js UI
      // Next.js UI uses different classes/IDs for the card.
      const nextCard = page.locator('.glassmorphism:has-text("New arrival! Link in bio."), .glassmorphism:has-text("Schedule a post?")').first();

      // Since e2e runs against Tauri, we check the Tauri structure.
      // Actually e2e runs against Next.js legacy UI too? Let's check both possibilities.

      const card = socialCard.or(nextCard);
      await expect(card).toBeVisible({ timeout: 15000 });

      // Locate the Approve & Schedule button
      const approveButton = card.locator('button:has-text("Approve & Schedule"), button[data-testid="approve-social-post"], button[data-testid="approve-btn"]').first();
      await expect(approveButton).toBeVisible();

      // Click approve
      await approveButton.click();

      // Check that the card disappears or status updates.
      // E.g., the UI might show "Approving..." or just remove the card.
      await expect(card).not.toBeVisible({ timeout: 10000 });
    });
  });
});
