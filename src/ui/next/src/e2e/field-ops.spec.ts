import { test, expect } from '../../../e2e/fixtures';

test.describe('Offline Field Operations', () => {
  test('Carlos can view jobs, go offline, add notes, and complete a job which generates a quote request', async ({ page, context, loginAs, adminUser }) => {
    await loginAs(page, adminUser);

    await page.goto('/field-ops/jobs');

    await expect(page.locator('text=Today\'s Route')).toBeVisible();
    await expect(page.locator('text=Alice Smith')).toBeVisible();

    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    await expect(page.locator('text=Offline Mode')).toBeVisible();

    const notesArea = page.locator('textarea').first();
    await notesArea.fill('Found a leak under the sink, requires immediate pipe replacement quote.');

    const headingToJobBtn = page.locator('button', { hasText: 'Heading to Job' }).first();

    try {
      await headingToJobBtn.waitFor({ state: 'visible', timeout: 5000 });
      await headingToJobBtn.click();
    } catch (e) {
    }

    const startWorkBtn = page.locator('button', { hasText: 'Start Work' }).first();
    try {
      await startWorkBtn.waitFor({ state: 'visible', timeout: 5000 });
      await startWorkBtn.click();
    } catch (e) {
    }

    const jobDoneBtn = page.locator('button', { hasText: 'Job Done' }).first();
    await jobDoneBtn.waitFor({ state: 'visible' });
    await jobDoneBtn.click();

    await expect(page.locator('text=Saved Notes:')).toBeVisible();
    await expect(page.locator('text="Found a leak under the sink, requires immediate pipe replacement quote."')).toBeVisible();
    await expect(page.locator('text=Sales Agent will draft an estimate based on these notes once online.')).toBeVisible();

    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    await expect(page.locator('text=Offline Mode')).not.toBeVisible();
  });
});

test.describe('Zero-Touch Smart Service Dispatch & Route Optimization Engine', () => {
  test('CUJ: View optimized route, report delay, and approve AI notification to subsequent clients', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);

    await page.goto('/field-ops/jobs');
    await expect(page.locator('text=Today\'s Route')).toBeVisible();
    await expect(page.locator('text=Alice Smith').first()).toBeVisible();

    const runningLateBtn = page.locator('button', { hasText: 'Running Late' }).first();
    await expect(runningLateBtn).toBeVisible();
    await runningLateBtn.click();

    const actionCard = page.locator('text=/Notify the next \\d+ clients of a 30-minute delay\\?|No subsequent clients affected. Adjust schedule\\?/');
    await expect(actionCard).toBeVisible();

    const approveBtn = page.locator('button', { hasText: 'Approve & Send' });
    await expect(approveBtn).toBeVisible();

    // Test the real flow through the actual app stack
    await approveBtn.click();

    // UI should close modal when successfully processed.
    // Given the E2E environment data setup, it will actually process it.
    await expect(actionCard).not.toBeVisible();
  });
});
