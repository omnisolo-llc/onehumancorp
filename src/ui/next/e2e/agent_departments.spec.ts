import { test, expect } from '@playwright/test';

/**
 * Persona: Maya — The Home Baker
 * Concept: Custom cakes from her kitchen.
 * Plan: Use AI agents to handle DMs and process orders.
 *
 * CUJ:
 * 1. Login to OHC
 * 2. Navigate to Team page
 * 3. Open Customer Success department
 * 4. Toggle "Review all messages"
 * 5. Navigate to Team Chat
 * 6. Ask "I need a quote for 50 vegan cupcakes"
 * 7. Sales agent generates a draft action
 * 8. Approve the draft action in chat
 */

test.describe('AI Agent Department Management', () => {
  test(' Maya manages her bakery team', async ({ page }) => {
    // 1. Login
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('maya@homebaker.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();
    await expect(page).not.toHaveURL(/.*\/login/);

    // 2. Navigate to Team page
    await page.goto('/team');
    await expect(page.getByText('Your Team')).toBeVisible();

    // 3. Open Customer Success
    await page.click('text=The Ambassador');
    await expect(page.getByText('Approval Inbox')).toBeVisible();

    // 4. Toggle Review Mode
    const toggle = page.locator('button').filter({ hasText: '' }).last();
    await toggle.click();

    // 5. Navigate to Team Chat
    await page.goto('/team');
    await page.click('aria-label=Team Chat');
    await expect(page.url()).toContain('/team/chat');

    // 6. Ask for quote
    const input = page.getByTestId('team-chat-input');
    await input.fill('I need a quote for 50 vegan cupcakes');
    await page.click('data-testid=team-chat-send');

    // 7. Sales agent generates draft
    await expect(page.getByTestId('action-card')).toBeVisible({ timeout: 15000 });
    await expect(page.getByText('The Salesperson')).toBeVisible();

    // 8. Approve draft
    await page.click('data-testid=approve-action-btn');
    await expect(page.getByText('Approved')).toBeVisible();
  });
});
