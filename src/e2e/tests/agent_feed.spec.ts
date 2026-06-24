import { test, expect } from '@playwright/test';

test.describe('Unified Agent Feed CUJ', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('Owner navigates to agent feed and approves an action card', async ({ page, request }) => {
    // We need to seed some test data using the real backend so we don't mock it

    await request.post('/api/v1/agent-feed', {
        data: {
            event_source: 'customer_dm',
            context_payload: {
              department: 'Customer Success',
              title: 'Draft Reply for Maya',
              description: '"Hi Maya! Yes, we still make the vegan chocolate cake. Would you like to reorder for this weekend?"'
            }
        },
    });

    await request.post('/api/v1/agent-feed', {
        data: {
            event_source: 'inventory_alert',
            context_payload: {
              department: 'Operations',
              title: 'Inventory Alert for Priya',
              description: 'Vanilla extract is running low (2 units left). Approve to generate a purchase order.'
            }
        },
    });

    await request.post('/api/v1/agent-feed', {
        data: {
            event_source: 'sale_metrics',
            context_payload: {
              department: 'Marketing',
              title: 'Weekend Flash Sale',
              description: 'Sales are down 15% this week. Approve to schedule a 20% off Flash Sale for all summer items this weekend.'
            }
        },
    });

    await page.goto('/dashboard');

    // Navigate to Agent Feed
    await page.getByRole('link', { name: /Agent Feed/i }).click();

    // Verify we landed on the Agent Feed page
    await expect(page.getByRole('heading', { name: 'Your Feed' })).toBeVisible();

    // The component might show "Loading Agent Proposals..." initially
    const loadingMessage = page.getByText('Loading Agent Proposals...');
    if (await loadingMessage.isVisible()) {
      await expect(loadingMessage).toBeHidden({ timeout: 15000 });
    }

    // Verify we have at least 3 distinct Action Cards
    const actionCards = page.locator('div.glassmorphism.shadow-sm');
    await expect(actionCards.count()).resolves.toBeGreaterThanOrEqual(3);

    // Verify departments
    await expect(page.getByText('Customer Success').first()).toBeVisible();
    await expect(page.getByText('Operations').first()).toBeVisible();
    await expect(page.getByText('Marketing').first()).toBeVisible();

    // Tap Approve on the first card
    const firstApproveBtn = page.getByRole('button', { name: 'Approve' }).first();
    await firstApproveBtn.click();

    // Verify the card moves to a Completed state
    await expect(page.getByText('Action completed').first()).toBeVisible();
  });
});
