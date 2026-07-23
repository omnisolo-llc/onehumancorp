import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';
import { Client } from 'pg';

test.describe('CustomerSuccessAgent Auto-Reply Flow', () => {
  let db: Client;
  let triageItemId: string;

  test.beforeAll(async () => {
    db = new Client({ connectionString: process.env.DATABASE_URL || 'postgres://ohc:ohc@localhost:5432/ohc' });
    await db.connect();
  });

  test.afterAll(async () => {
    if (db) {
      if (triageItemId) {
        await db.query(`DELETE FROM agent_feed WHERE id = $1`, [triageItemId]);
      }
      await db.end();
    }
  });

  test('SMB Owner can approve a CustomerSuccessAgent draft on mobile', async ({ browser }) => {
    // 1. Simulate mobile viewport
    const context = await browser.newContext({
      viewport: { width: 375, height: 812 },
      userAgent: 'Mozilla/5.0 (iPhone; CPU iPhone OS 13_2_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.0.3 Mobile/15E148 Safari/604.1',
    });

    const page = await context.newPage();

    // Authenticate (reusing fixture logic conceptually, but manually here for mobile context)
    await page.goto('/login');
    await page.fill('input[name="email"]', 'test@example.com');
    await page.fill('input[name="password"]', 'password123');
    await page.click('button[type="submit"]');
    await page.waitForURL('/dashboard');

    // 2. Inject a pending agent draft into the backend
    const res = await db.query(`
      INSERT INTO agent_feed (tenant_id, id, source, priority, description, payload, state, title)
      VALUES (
        'e2e-tenant',
        gen_random_uuid(),
        'CustomerSuccessAgent',
        'High',
        'Customer inquired about custom cake pricing on Instagram.',
        '{"draft": "Hi Maya! A custom 8-inch cake starts at $65. Let me know what flavor you want."}',
        'PENDING_APPROVAL',
        'Instagram Inquiry'
      )
      RETURNING id;
    `);
    triageItemId = res.rows[0].id;

    // 3. Navigate to Dashboard (Command Center) and verify the card
    await page.goto('/dashboard');
    await page.reload(); // Ensure fresh data

    const card = page.locator(`[data-testid="triage-card-${triageItemId}"]`);
    await expect(card).toBeVisible();
    await expect(card.locator('text=Needs Attention: Pending Draft')).toBeVisible();
    await expect(card.locator('text=Customer inquired about custom cake pricing on Instagram.')).toBeVisible();

    // Verify the draft text is visible
    const draftContainer = card.locator(`[data-testid="triage-draft-${triageItemId}"]`);
    await expect(draftContainer).toBeVisible();
    await expect(draftContainer).toContainText('Hi Maya! A custom 8-inch cake starts at $65. Let me know what flavor you want.');

    // 4. Tap "Approve & Send" and verify loading state
    const approveBtn = card.locator(`[data-testid="triage-approve-${triageItemId}"]`);
    await expect(approveBtn).toBeVisible();

    // Verify touch target size (at least 44x44px)
    const btnBox = await approveBtn.boundingBox();
    expect(btnBox?.width).toBeGreaterThanOrEqual(44);
    expect(btnBox?.height).toBeGreaterThanOrEqual(44);

    // Verify initial text
    await expect(approveBtn.locator('.btn-text')).toHaveText('Approve & Send');

    await approveBtn.click();

    // 5. Verify the card disappears (handled by handleTriageAction removing the element)
    await expect(card).not.toBeVisible();

    // 6. Verify backend state updated to APPROVED
    const finalState = await db.query(`SELECT state FROM agent_feed WHERE id = $1`, [triageItemId]);
    expect(finalState.rows[0].state).toBe('APPROVED');
  });
});
