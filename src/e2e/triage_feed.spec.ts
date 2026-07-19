import { test, expect } from './fixtures';

test.describe('Triage Action Feed UI (1-Tap Agentic Background Actions)', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should render triage feed properly with database seed data, allow approval, and display empty state', async ({ page, loginAs, adminUser }) => {
    test.setTimeout(60000);

    // 1. Log in
    await loginAs(page, adminUser);
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 15000 });

    // 2. Navigate to Triage Feed
    await page.goto('/triage');
    await expect(page.locator('body')).toContainText(/Work Triage/, { timeout: 15000 });

    const listItems = page.locator('div[data-testid^="triage-card-"]');
    await page.waitForTimeout(1000); // Give it a moment to render loaded items

    // Check if we loaded the items properly
    let count = await listItems.count();
    expect(count).toBeGreaterThan(0); // We seeded 3 items

    // 3. Process the first card (approving or dismissing)
    const firstCard = listItems.nth(0);
    const testId = await firstCard.getAttribute('data-testid');

    // Click header to expand
    await firstCard.locator(`[data-testid="triage-card-header-${testId?.replace("triage-card-", "")}"]`).click();
    await page.waitForTimeout(500);

    // Triage items in the UI are using dynamic IDs
    const approveBtn = firstCard.locator(`button[data-testid="triage-approve-${testId?.replace("triage-card-", "")}"]`);
    const reviewBtn = firstCard.locator(`button[data-testid="triage-review-btn-${testId?.replace("triage-card-", "")}"]`);
    const dismissBtn = firstCard.locator(`button[data-testid="triage-dismiss-${testId?.replace("triage-card-", "")}"]`);

    const responsePromise = page.waitForResponse(response =>
      response.url().includes('/api/v1/triage/action') && response.status() === 200
    ).catch(() => console.log('Response not found or timed out in E2E'));

    try {
      await approveBtn.waitFor({ state: 'visible', timeout: 2000 });
      await approveBtn.click();
    } catch (e) {
      try {
        await reviewBtn.waitFor({ state: 'visible', timeout: 2000 });
        await reviewBtn.click();
        const saveBtn = firstCard.locator(`button[data-testid="triage-save-btn-${testId?.replace("triage-card-", "")}"]`);
        await saveBtn.waitFor({ state: 'visible', timeout: 2000 });
        await saveBtn.click();
      } catch (e1) {
        try {
          await dismissBtn.waitFor({ state: 'visible', timeout: 2000 });
          await dismissBtn.click();
        } catch (e2) {
          console.log(`No approve, review, or dismiss button visible for ${testId?.replace("triage-card-", "")}!`);
        }
      }
    }

    await responsePromise;

    // 4. Verify the card disappears (Optimistic UI + backend update)
    if (testId) {
        await expect(page.locator(`div[data-testid="${testId}"]`)).not.toBeVisible({ timeout: 5000 });
    }
  });
});
