import { randomUUID } from 'node:crypto';
import { test, expect } from './fixtures';
import { createGrowthOwner } from './growth_owner';
import { e2eDbQuery } from './db_utils';

test('SMB owner can approve their persisted CustomerSuccessAgent draft on mobile', async ({ browser, baseURL }) => {
  if (!baseURL) throw new Error('The actual isolated application origin is required');
  const context = await browser.newContext({
    baseURL,
    viewport: { width: 375, height: 812 },
    userAgent: 'Mozilla/5.0 (iPhone; CPU iPhone OS 13_2_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.0.3 Mobile/15E148 Safari/604.1',
  });
  try {
    const page = await context.newPage();
    const owner = await createGrowthOwner(page, baseURL);
    const triageItemId = randomUUID();
    const draft = 'Hi Maya! A custom 8-inch cake starts at $65. Let me know what flavor you want.';
    // This is an explicitly test-owned persisted input, not provider-generation
    // evidence. Seed before this new tenant's first feed read so direct SQL does
    // not rely on an event invalidating a different test's cached feed.
    const rows = await e2eDbQuery(
      `INSERT INTO agent_feed (tenant_id,id,source,priority,description,payload,state,title)
       VALUES ($1,$2,'CustomerSuccessAgent','High',$3,$4::jsonb,'PENDING_APPROVAL','Instagram Inquiry')
       RETURNING id,tenant_id,state`,
      [owner.tenantId,triageItemId,'Customer inquired about custom cake pricing on Instagram.',JSON.stringify({draft})],
    );
    expect(rows).toEqual([{id:triageItemId,tenant_id:owner.tenantId,state:'PENDING_APPROVAL'}]);
    const identity = await page.request.get('/api/v1/auth/session-identity');
    expect(identity.status()).toBe(200);
    expect(await identity.json()).toMatchObject({userId:owner.userId,tenantId:owner.tenantId});
    const recorded = await page.request.get('/api/v1/agent-feed');
    expect(recorded.status()).toBe(200);
    expect((await recorded.json()).items).toEqual(expect.arrayContaining([expect.objectContaining({id:triageItemId,tenant_id:owner.tenantId})]));

    await page.goto('/dashboard');
    const card = page.getByTestId(`triage-card-${triageItemId}`);
    await expect(card).toBeVisible();
    await expect(card.getByText('Needs Attention: Pending Draft')).toBeVisible();
    await expect(card.getByText('Customer inquired about custom cake pricing on Instagram.')).toBeVisible();
    await expect(card.getByTestId(`triage-draft-${triageItemId}`)).toContainText(draft);
    const approve = card.getByTestId(`triage-approve-${triageItemId}`);
    await expect(approve).toBeVisible();
    const box = await approve.boundingBox();
    expect(box?.width).toBeGreaterThanOrEqual(44);
    expect(box?.height).toBeGreaterThanOrEqual(44);
    await expect(approve.locator('.btn-text')).toHaveText('Approve & Send');
    const decision = page.waitForResponse(response => new URL(response.url()).pathname === `/api/v1/agent-feed/${triageItemId}` && response.request().method() === 'PUT');
    await approve.click();
    expect((await decision).status()).toBe(200);
    await expect(card).not.toBeVisible({timeout:10000});
    // A committed approval is not proof that a provider delivered the message.
    await expect.poll(async () => e2eDbQuery('SELECT state FROM agent_feed WHERE id=$1 AND tenant_id=$2',[triageItemId,owner.tenantId])).toEqual([{state:'APPROVED'}]);
  } finally { await context.close(); }
});
