import { expect, it, vi } from 'vitest';
import type { Page } from '@playwright/test';
import { clickAuditStates, isolatedClickAuditRoutes, prepareClickAuditState } from '../../../../e2e/support/dashboard_audit_fixture';

it.each(['/onboarding', '/share-card'])('isolates each real %s control and declares the complete initial/selected setup views', route => {
  expect(isolatedClickAuditRoutes.has(route)).toBe(true);
  expect(clickAuditStates(route)).toEqual(['entry', 'intro', 'instant-draft', 'manual-draft']);
});

it.each([
  { state: 'intro', expected: ['Back'] },
  { state: 'instant-draft', expected: ['Back', 'Instant Build'] },
  { state: 'manual-draft', expected: ['Back', 'Start My Business'] },
])('prepares $state through acknowledged real UI choices before observation', async ({ state, expected }) => {
  const events: string[] = [];
  const page = {
    url: () => 'http://127.0.0.1:3000/onboarding',
    evaluate: vi.fn(async () => undefined),
    waitForResponse: vi.fn(async () => ({ status: () => 204, finished: async () => { events.push('finished'); }, request: () => ({ postDataJSON: () => ({}) }) })),
    getByRole: (_role: string, options: { name: string }) => ({ click: async () => { events.push('click:' + options.name); }, waitFor: async () => undefined }),
    locator: () => ({ waitFor: async () => undefined }),
  } as unknown as Page;
  await prepareClickAuditState(page, '/onboarding', state);
  expect(events.filter(event => event.startsWith('click:'))).toEqual(expected.map(label => 'click:' + label));
  expect(events).toEqual(expected.flatMap(label => ['click:' + label, 'finished']));
});

it('never accepts a failed onboarding state save as a prepared click baseline', async () => {
  const page = {
    url: () => 'http://127.0.0.1:3000/onboarding',
    evaluate: vi.fn(async () => undefined),
    waitForResponse: vi.fn(async () => ({ status: () => 500, finished: vi.fn() })),
    getByRole: () => ({ click: vi.fn(), waitFor: vi.fn() }),
    locator: () => ({ waitFor: vi.fn() }),
  } as unknown as Page;
  await expect(prepareClickAuditState(page, '/onboarding', 'intro')).rejects.toThrow(/state.*HTTP 500/i);
});

it('does not start observation while acknowledged preparation still has admitted writes pending', async () => {
  let release!: () => void;
  let complete = false;
  const page = {
    url: () => 'http://127.0.0.1:3000/onboarding', evaluate: vi.fn(async () => undefined),
    waitForResponse: vi.fn(async () => ({ status: () => 204, finished: async () => null })),
    getByRole: () => ({ click: vi.fn(), waitFor: vi.fn() }),
    locator: (selector: string) => ({ waitFor: () => selector === '#setup-screen[aria-busy="false"]'
      ? new Promise<void>(resolve => { release = resolve; }) : Promise.resolve() }),
  } as unknown as Page;
  const preparing = prepareClickAuditState(page, '/onboarding', 'intro').then(() => { complete = true; });
  await new Promise(resolve => setTimeout(resolve, 0));
  const returnedWhilePending = complete;
  release?.(); await preparing;
  expect(returnedWhilePending).toBe(false);
  expect(complete).toBe(true);
});

it('rejects a failed transport completion even when preparation headers contained204', async () => {
  const page = {
    url: () => 'http://127.0.0.1:3000/onboarding', evaluate: vi.fn(async () => undefined),
    waitForResponse: vi.fn(async () => ({ status: () => 204, finished: async () => new Error('Preparation response interrupted') })),
    getByRole: () => ({ click: vi.fn(), waitFor: vi.fn() }), locator: () => ({ waitFor: vi.fn() }),
  } as unknown as Page;
  await expect(prepareClickAuditState(page, '/onboarding', 'intro')).rejects.toThrow('Preparation response interrupted');
});
