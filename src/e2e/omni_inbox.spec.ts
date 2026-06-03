import { test, expect } from './fixtures';

test.describe('Omni-Inbox Auto-Reply Agent', () => {
  test('simulates incoming message and handles different confidence score states', async ({ page, request }) => {
    // We send a webhook to simulate different levels of confidence
    // High confidence message
    await request.post('/api/agents/webhook', {
      data: {
        tenant_id: 'e2e-tenant',
        message: 'Are you open today?',
        source: 'instagram'
      }
    });

    // Low confidence message
    await request.post('/api/agents/webhook', {
      data: {
        tenant_id: 'e2e-tenant',
        message: 'I want to escalate this complaint immediately!',
        source: 'whatsapp'
      }
    });

    // Medium confidence message
    await request.post('/api/agents/webhook', {
      data: {
        tenant_id: 'e2e-tenant',
        message: 'Do you offer vegan options?',
        source: 'facebook'
      }
    });

    await page.goto('/inbox');

    // Wait for messages to load and verify high confidence auto-reply
    await expect(page.getByText('Are you open today?').first()).toBeVisible({ timeout: 10000 });
    const autoReplyBadge = page.getByText('✅ Auto-Replied (Score:');
    await expect(autoReplyBadge).toBeVisible();

    // Verify low confidence escalated
    await expect(page.getByText('I want to escalate this complaint immediately!').first()).toBeVisible();
    const escalateBadge = page.getByText('⚠️ Escalated (Score:');
    await expect(escalateBadge).toBeVisible();
    await expect(page.getByRole('button', { name: 'Write Manual Reply' }).first()).toBeVisible();

    // Verify medium confidence draft
    await expect(page.getByText('Do you offer vegan options?').first()).toBeVisible();
    const draftBadge = page.getByText('AI Draft (Score:');
    await expect(draftBadge).toBeVisible();
    await expect(page.getByRole('button', { name: 'Approve' }).first()).toBeVisible();
  });
});
