import { test, expect } from '../../../../e2e/fixtures';

test.describe('Client Hub Portal E2E Flow', () => {
  test('should navigate through all client portal tabs and perform interactive flows', async ({ page }) => {
    // 1. Navigate to client-portal
    await page.goto('/client-portal');

    // Verify header and welcome banner are visible
    await expect(page.locator('h1', { hasText: 'Client Hub Portal' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Welcome Back, Acme Corporation!' })).toBeVisible();
    await expect(page.locator('text=Today's Priorities')).toBeVisible();

    // 2. Tab: Proposals & Quotes
    const proposalsTab = page.locator('button', { hasText: 'Proposals & Quotes' });
    await proposalsTab.click();
    await expect(page.locator('h2', { hasText: 'Active Proposals & Cost Estimations' })).toBeVisible();

    // Sign outstanding proposal
    await page.locator('input[placeholder="Full Legal Name"]').fill('Jane Doe');
    await page.locator('input[type="checkbox"]').check();
    await page.locator('button', { hasText: 'Sign & Approve Proposal' }).click();

    // Verify signature success message
    await expect(page.locator('text=Successfully approved proposal QT-9021')).toBeVisible();

    // 3. Tab: Invoices & Billing
    const billingTab = page.locator('button', { hasText: 'Invoices & Billing' });
    await billingTab.click();
    await expect(page.locator('h2', { hasText: 'Invoices & Milestone Billing' })).toBeVisible();

    // Fill mock credit card details
    await page.locator('input[placeholder="4000 1234 5678 9010"]').fill('4242424242424242');
    await page.locator('input[placeholder="MM/YY"]').fill('12/28');
    await page.locator('input[placeholder="123"]').fill('456');

    // Authorize payment
    await page.locator('button', { hasText: 'Authorize Payment' }).click();

    // Wait for the simulated payment delay to complete
    await expect(page.locator('text=Invoice Fully Paid')).toBeVisible({ timeout: 5000 });
    await expect(page.locator('text=Payment of $1500.00 received successfully')).toBeVisible();

    // 4. Tab: Digital Products
    const digitalTab = page.locator('button', { hasText: 'Digital Products' });
    await digitalTab.click();
    await expect(page.locator('h2', { hasText: 'Digital Products, Online Courses & Podcasts' })).toBeVisible();

    // Tick lesson checkbox
    const lessonCheckbox = page.locator('input[type="checkbox"]').nth(3); // 4th lesson
    await lessonCheckbox.check();
    await expect(page.locator('text=80% Completed')).toBeVisible();

    // Play podcast episode
    const playEpisode = page.locator('button', { hasText: '▶️' }).first();
    await playEpisode.click();
    await expect(page.locator('text=NOW PLAYING')).toBeVisible();
    await expect(page.locator('text=Episode 14: Scaling Multi-Agent Systems Offline-First')).toBeVisible();

    // 5. Tab: Project Tracker
    const projectsTab = page.locator('button', { hasText: 'Project Tracker' });
    await projectsTab.click();
    await expect(page.locator('h2', { hasText: 'Project Tracker & Active Workflows' })).toBeVisible();
    await expect(page.locator('text=Deploy Sandbox Environment & Secure Integrations')).toBeVisible();

    // 6. Tab: Help & Live Chat
    const supportTab = page.locator('button', { hasText: 'Help & Live Chat' });
    await supportTab.click();
    await expect(page.locator('h3', { hasText: 'Create Helpdesk Ticket' })).toBeVisible();

    // Submit ticket
    await page.locator('textarea[placeholder="Detail your request..."]').fill('Urgent assistance required with database credentials.');
    await page.locator('button', { hasText: 'Submit Support Ticket' }).click();
    await expect(page.locator('text=Support ticket created successfully')).toBeVisible();

    // Chat with assistant
    await page.locator('input[placeholder*="Type a message"]').fill('I need help with my invoice');
    await page.locator('button', { hasText: '➤' }).click();

    // Verify our user message is in chat history
    await expect(page.locator('text=I need help with my invoice')).toBeVisible();

    // Verify AI responds after short latency
    await expect(page.locator('text=I see you\'re asking about billing. You can view your current invoices')).toBeVisible({ timeout: 3000 });
  });
});
