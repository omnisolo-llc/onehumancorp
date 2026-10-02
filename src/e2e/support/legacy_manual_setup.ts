import { randomUUID } from 'node:crypto';
import { expect, type Page } from '@playwright/test';
import { captureDurablePost, type DurablePostReceipt } from './durable_post_receipt';
export type SetupOwner = {
    userId: string;
    tenantId: string;
};
export type SetupReply = {
    status: number;
    body: Record<string, unknown>;
};
type SetupLaunchReply = SetupReply & Pick<DurablePostReceipt, 'requestBody' | 'expectedUser' | 'expectedTenant'>;
export function requireLocalSetup(page: Page) {
    const url = new URL(page.url());
    expect(['http:', 'https:']).toContain(url.protocol);
    expect(['localhost', '127.0.0.1', '[::1]']).toContain(url.hostname);
    return url.origin;
}
function object(value: unknown): Record<string, unknown> {
    if (!value || typeof value !== 'object' || Array.isArray(value))
        throw Error('Setup returned a non-object receipt');
    return value as Record<string, unknown>;
}
function nonempty(value: unknown): string {
    if (typeof value !== 'string' || !value)
        throw Error('Setup receipt lacks a required identity');
    return value;
}
export async function verifiedSetupOwner(page: Page): Promise<SetupOwner> {
    requireLocalSetup(page);
    const response = await page.request.get('/api/v1/auth/session-identity');
    expect(response.status()).toBe(200);
    const identity = object(await response.json());
    expect(identity.error ?? null).toBeNull();
    expect(identity.success).not.toBe(false);
    return { userId: nonempty(identity.userId), tenantId: nonempty(identity.tenantId) };
}
export function captureSetupPost(page: Page, path: string): Promise<SetupReply> {
    const origin = requireLocalSetup(page);
    return page.waitForResponse(response => new URL(response.url()).origin === origin && new URL(response.url()).pathname === `/api/v1/onboarding/${path}` && response.request().method() === 'POST')
        .then(async (response) => ({ status: response.status(), body: object(JSON.parse(await response.text())) }));
}
export async function captureSetupLaunch(page: Page) {
    requireLocalSetup(page);
    const capture = await captureDurablePost(page, '/api/v1/onboarding/launch');
    const response = capture.response.then(receipt => ({ ...receipt, body: object(JSON.parse(receipt.body)) }));
    void response.catch(() => undefined);
    return { ...capture, response };
}
export function expectPreparedSetup(reply: SetupReply, owner: SetupOwner): string {
    expect(reply.status).toBe(200);
    expect(reply.body).toMatchObject({ success: true, status: 'prepared', organization_id: owner.tenantId, user_id: owner.userId });
    expect(reply.body.error ?? null).toBeNull();
    const id = nonempty(reply.body.preparation_id);
    expect(object(reply.body.preparation)).toMatchObject({ preparation_id: id, status: 'prepared', organization_id: owner.tenantId, user_id: owner.userId });
    return id;
}
export async function expectLaunchedSetup(page: Page, reply: SetupLaunchReply, id: string, owner: SetupOwner) {
    expect(object(JSON.parse(reply.requestBody))).toEqual({ preparation_id: id });
    expect(reply.expectedUser).toBe(owner.userId);
    expect(reply.expectedTenant).toBe(owner.tenantId);
    expect(reply.status).toBe(200);
    expect(reply.body).toMatchObject({ success: true, status: 'launched', preparation_id: id, organization_id: owner.tenantId, user_id: owner.userId });
    expect(reply.body.error ?? null).toBeNull();
    const state = await page.request.get('/api/v1/onboarding/state');
    expect(state.status()).toBe(200);
    expect(object(await state.json()).preparation).toMatchObject({ preparation_id: id, status: 'launched', organization_id: owner.tenantId, user_id: owner.userId });
}
export async function enterManualSetup(page: Page, source: 'chat' | 'instant', prompt: string) {
    requireLocalSetup(page);
    await page.locator(`#${source}-manual-review`).click();
    await expect(page.locator('#manual-intake-context')).toContainText(prompt);
    await expect(page.locator('#first-offer')).toHaveValue('');
    await expect(page.locator('#first-product-price')).toHaveValue('');
    await page.getByTestId('context-local').click();
    await page.locator('#step-context .next-step-btn').click();
    await page.locator('#business-categories').selectOption('Other');
    await page.locator('#step-categories .next-step-btn').click();
    await page.locator('#business-name').fill('Owner-entered Service Studio');
    await page.locator('#step-name .next-step-btn').click();
    await page.getByTestId('team-support').click();
    await page.locator('#assistant-tone').selectOption('Professional');
    await page.locator('#step-assistant .next-step-btn').click();
    await expect(page.locator('#step-admin')).toBeVisible();
    await expect(page.locator('input[type="password"],#admin-email,#admin-name')).toHaveCount(0);
    await page.locator('#step-admin .next-step-btn').click();
    await page.locator('#first-offer').fill('Owner-entered first service');
    await page.locator('#first-product-price').fill('35.00');
    await page.locator('#step-offer .next-step-btn').click();
    await page.locator('#location-input').fill('Austin, TX');
    await page.locator('#step-location .next-step-btn').click();
    await page.locator('#target-audience').fill('Local customers');
    await page.locator('#step-target-audience .next-step-btn').click();
    await page.locator('#domain-name').fill(`owner-setup-${randomUUID()}`);
    await page.locator('#step-domain .next-step-btn').click();
    await page.locator('#template-selection').selectOption('Modern');
    await expect(page.locator('#finish-btn')).toBeEnabled();
}
export async function completeManualSetup(page: Page, source: 'chat' | 'instant', prompt: string, owner: SetupOwner) {
    await enterManualSetup(page, source, prompt);
    requireLocalSetup(page);
    // Configure passive durable observation before the action; navigation may
    // otherwise discard the renderer's actual launch response body.
    const launching = await captureSetupLaunch(page);
    try {
        const preparing = captureSetupPost(page, 'start');
        const [prepared, launched] = await Promise.all([preparing, launching.response, page.locator('#finish-btn').click()]);
        const id = expectPreparedSetup(prepared, owner);
        expect(object(prepared.body.preparation).reviewed_request).toMatchObject({ company_name: 'Owner-entered Service Studio', first_product_name: 'Owner-entered first service', first_product_price: '35.00', location: 'Austin, TX', target_audience: 'Local customers' });
        await expectLaunchedSetup(page, launched, id, owner);
    } finally { await launching.dispose(); }
}
