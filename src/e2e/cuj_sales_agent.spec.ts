import { test, expect } from './fixtures';

test.describe('CUJ: AI Agent Department Architecture - Sales Agent Custom Cake', () => {

  test('UI: Salesperson agent generates quote draft and appears in pending approvals', async ({ page }) => {
    let postCalled = false;

    await page.route('**/api/agents/approvals', async (route, request) => {
      if (request.method() === 'GET') {
        await route.fulfill({ json: { pending_approvals: [
          { id: 'mock-sales-cuj-1', department: 'Sales', description: 'The Salesperson drafted a quote for a custom cake and drafted a reply. | Payload: {"draft_reply":"We can certainly make a custom cake for you. Here is a quote for $150.00.","generated_quote":"$150.00","original_message":"I need a custom cake"}', status: 'Pending', action_risk: 'High' }
        ] } });
      } else {
        await route.fallback();
      }
    });

    await page.route('**/api/agents/approvals/mock-sales-cuj-1', async route => {
      postCalled = true;
      await route.fulfill({ json: { success: true } });
    });

    await page.goto('/team');

    // 1. Check The Salesperson card
    const salespersonCard = page.locator('button', { hasText: 'The Salesperson' });
    await expect(salespersonCard).toContainText('1 item awaiting approval');
    await salespersonCard.click();

    // 2. View details of the approval request
    await expect(page.locator('h1')).toContainText('The Salesperson');
    await expect(page.getByText('The Salesperson drafted a quote for a custom cake and drafted a reply.')).toBeVisible();

    // 3. Reject/Edit logic
    const rejectBtn = page.getByRole('button', { name: 'Reject / Edit' });
    await expect(rejectBtn).toBeVisible();

    // 4. Approve action
    await page.getByRole('button', { name: 'Approve' }).click();

    // 5. Success UI verification
    await expect(page.getByText('All Caught Up!')).toBeVisible();
    expect(postCalled).toBe(true);
  });

  test('UI: Business Advisory agent presents summary briefing', async ({ page }) => {
    let postCalled = false;

    await page.route('**/api/agents/approvals', async (route, request) => {
      if (request.method() === 'GET') {
        await route.fulfill({ json: { pending_approvals: [
          { id: 'mock-advisory-cuj-1', department: 'BusinessAdvisory', description: 'Good morning Maya. You have a new cake inquiry. I\'ve drafted a reply and quote for your review. | Payload: {"department":"sales","description":"The Salesperson drafted a quote for a custom cake and drafted a reply.","risk":"HIGH"}', status: 'Pending', action_risk: 'High' }
        ] } });
      } else {
        await route.fallback();
      }
    });

    await page.route('**/api/agents/approvals/mock-advisory-cuj-1', async route => {
      postCalled = true;
      await route.fulfill({ json: { success: true } });
    });

    await page.goto('/team');

    const advisorCard = page.locator('button', { hasText: 'The Advisor' });
    await expect(advisorCard).toContainText('1 item awaiting approval');
    await advisorCard.click();

    await expect(page.locator('h1')).toContainText('The Advisor');
    await expect(page.getByText('Good morning Maya. You have a new cake inquiry.')).toBeVisible();

    await page.getByRole('button', { name: 'Approve' }).click();
    await expect(page.getByText('All Caught Up!')).toBeVisible();
    expect(postCalled).toBe(true);
  });

  test('UI: Missing data shows All Caught Up in Salesperson', async ({ page }) => {
    await page.route('**/api/agents/approvals', async route => {
      await route.fulfill({ json: { pending_approvals: [] } });
    });

    await page.goto('/team');
    await page.locator('button', { hasText: 'The Salesperson' }).click();
    await expect(page.getByText('All Caught Up!')).toBeVisible();
    await expect(page.getByText('There are no pending actions requiring your review.')).toBeVisible();
  });

  test('UI: Action Risk is properly highlighted on cards for Sales Drafts', async ({ page }) => {
    await page.route('**/api/agents/approvals', async (route, request) => {
      if (request.method() === 'GET') {
        await route.fulfill({ json: { pending_approvals: [
          { id: 'mock-risk-high', department: 'Sales', description: 'High Risk Action', status: 'Pending', action_risk: 'High' }
        ] } });
      } else {
        await route.fallback();
      }
    });

    await page.goto('/team');
    await page.locator('button', { hasText: 'The Salesperson' }).click();

    const highRiskBadge = page.locator('span', { hasText: 'High Risk' });
    await expect(highRiskBadge).toBeVisible();
    await expect(highRiskBadge).toHaveClass(/bg-\[#FF3B30\]\/10/);
    await expect(highRiskBadge).toHaveClass(/text-\[#FF3B30\]/);
  });

  test('UI: Mobile viewport behavior ensures cards and touches are accessible', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });

    await page.route('**/api/agents/approvals', async route => {
      await route.fulfill({ json: { pending_approvals: [] } });
    });

    await page.goto('/team');
    const header = page.locator('h1', { hasText: 'Your Team' });
    await expect(header).toBeVisible();

    const salesBtn = page.locator('button', { hasText: 'The Salesperson' });
    const box = await salesBtn.boundingBox();
    // Ensuring touch targets are large enough
    expect(box?.height).toBeGreaterThanOrEqual(44);
  });

});
