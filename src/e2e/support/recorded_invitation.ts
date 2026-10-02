import { expect, type Page, type Locator } from '@playwright/test';

export function requireLoopbackUrl(value: string) {
  const url = new URL(value);
  expect(['http:', 'https:']).toContain(url.protocol);
  expect(['localhost', '127.0.0.1', '[::1]']).toContain(url.hostname);
}


export async function createRecordedInvitation(page: Page, controls?: { button: Locator; input: Locator; invitee: string }): Promise<string> {
  requireLoopbackUrl(page.url());
  const button = controls?.button ?? page.locator('#dashboard-invite-btn');
  const input = controls?.input ?? page.locator('#dashboard-invite-link');
  const invitee = controls?.invitee ?? 'pending';
  await expect(button).toBeEnabled();
  requireLoopbackUrl(page.url());
  const origin = new URL(page.url()).origin;
  const responsePromise = page.waitForResponse(response =>
    new URL(response.url()).origin === origin
    && new URL(response.url()).pathname === '/api/v1/growth/cloud-bridge/invite'
    && response.request().method() === 'POST');
  await button.click();
  const response = await responsePromise;
  expect(response.status()).toBe(200);
  expect(response.request().postDataJSON()).toEqual({ invitee_id: invitee });
  const receipt: unknown = await response.json();
  if (!receipt || typeof receipt !== 'object' || Array.isArray(receipt)
    || !('invite_link' in receipt) || typeof receipt.invite_link !== 'string') {
    throw new Error('Invitation response lacks a confirmed link');
  }
  expect('success' in receipt ? receipt.success : undefined).not.toBe(false);
  expect(('error' in receipt ? receipt.error : null) ?? null).toBeNull();
  expect(receipt.invite_link).toMatch(/^https:\/\/(cloud\.)?omnisolo\.co\/invite\/[^/?#]+$/);
  expect(receipt.invite_link).not.toMatch(/\/(fallback|default)$/);
  await expect(input).toHaveValue(receipt.invite_link);
  return receipt.invite_link;
}
