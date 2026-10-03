import { randomUUID } from 'node:crypto';
import { expect, test } from './fixtures';
import { createLinkBioActor } from './link_bio_owner';
import { e2eDbQuery } from './db_utils';

test.describe('Inbox real-data behavior', () => {
  test('loads conversations from the database-backed inbox endpoint', async ({ page, baseURL, adminUser }) => {
    test.setTimeout(60000);
    const actor = await createLinkBioActor(page, baseURL, adminUser);
    const messageId = `inbox-record-${randomUUID()}`;
    const content = `Recorded inbox question ${messageId}`;
    await e2eDbQuery(`INSERT INTO omni_inbox_messages
      (id,tenant_id,source,sender_id,original_content,translated_content,target_language,status)
      VALUES($1,$2,'email','owned-test-sender',$3,$3,'en','pending')`,
      [messageId, actor.tenantId, content]);

    await page.goto('/inbox');
    // Hydration can retire its first read. Verify a complete authenticated API
    // response and the actual rendered owned record, not a transient response.
    const response = await page.request.get('/api/v1/ui/omni_inbox', {
      headers: { 'x-ohc-expected-user': actor.userId, 'x-ohc-expected-tenant': actor.tenantId },
    });

    expect(response.status(), '/api/v1/ui/omni_inbox must return the recorded data rendered by this view').toBe(200);
    expect(await response.json()).toEqual([expect.objectContaining({ id: messageId })]);
    await expect(page.getByRole('heading', { name: 'Inbox' })).toBeVisible();
    const workspace = page.getByTestId('inbox-settled');
    await expect(workspace).toBeVisible();
    // Loading and settled shells can coexist during React replacement. Verify
    // the actual recorded message instead of an ambiguous static source caption.
    await expect(workspace.locator('#messages-list').getByText(content, { exact: true })).toBeVisible();
  });

  test('does not expose simulated inbox controls or silent send-message no-ops', async ({ page }) => {
    await page.goto('/inbox');

    await expect(page.getByRole('button', { name: /simulate incoming message/i })).toHaveCount(0);
    await expect(page.getByText(/AI Replied/i)).toHaveCount(0);

    const sendButtons = page.getByRole('button', { name: /send message/i });
    const sendButtonCount = await sendButtons.count();

    for (let index = 0; index < sendButtonCount; index += 1) {
      const button = sendButtons.nth(index);
      const before = await page.locator('body').innerText();
      let responseSeen = false;

      const responsePromise = page.waitForResponse((response) => {
        const method = response.request().method();
        return ['POST', 'PUT', 'PATCH'].includes(method) && response.status() < 500;
      }, { timeout: 5000 }).then(() => {
        responseSeen = true;
      }).catch(() => undefined);

      await button.click();
      await responsePromise;
      await page.waitForTimeout(250);

      const after = await page.locator('body').innerText();
      expect(responseSeen || after !== before, 'Send message must produce a real response or visible state change').toBeTruthy();
    }
  });
});
