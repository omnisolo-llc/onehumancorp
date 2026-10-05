import React from 'react';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import vm from 'node:vm';
import ts from 'typescript';
import { act, cleanup, fireEvent, queryAllByRole, queryAllByTestId, render } from '@testing-library/react';
import { afterEach, describe, expect, it, vi } from 'vitest';
import type { AgentFeedItem } from '../../lib/agent-feed-types';
import FeedPage from './page';

vi.mock('../components/AppShell', () => ({
  AppShell: ({ children }: { children: React.ReactNode }) => <main>{children}</main>,
}));
vi.mock('../../hooks/useAuthenticatedPolling', () => ({ useAuthenticatedPolling: vi.fn() }));

const compiledSpec = ts.transpileModule(
  readFileSync(join(__dirname, '../../e2e/unified_agent_feed_interaction.spec.ts'), 'utf8'),
  { compilerOptions: { module: ts.ModuleKind.CommonJS } },
).outputText;

type Action = 'approve' | 'dismiss';
type ButtonContract = 'name' | 'testid';
type Role = Parameters<typeof queryAllByRole>[1];
type RoleOptions = Parameters<typeof queryAllByRole>[2];
type Locator = {
  query: () => HTMLElement[];
  label: string;
  filter: (options: { hasText: string }) => Locator;
  getByTestId: (id: string) => Locator;
  getByRole: (role: Role, options: RoleOptions) => Locator;
  and: (other: Locator) => Locator;
  click: () => Promise<void>;
};
type Page = { goto: (url: string) => Promise<void>; getByTestId: (id: string) => Locator };
type SpecCase = (fixtures: { page: Page }) => Promise<void>;
type BeforeCase = (fixtures: { page: Page }, info: { testId: string }) => Promise<void>;

// Execute the maintained journey against the real /feed renderer. Only the
// browser/transport boundary is adapted; this does not replace real-stack E2E.
async function runJourney(action: Action, missingContract?: ButtonContract) {
  let beforeCase: BeforeCase;
  const cases = new Map<string, SpecCase>();
  const test = Object.assign(
    (title: string, callback: SpecCase) => { cases.set(title, callback); },
    {
      describe: (_title: string, callback: () => void) => callback(),
      use: () => {},
      beforeEach: (callback: BeforeCase) => { beforeCase = callback; },
    },
  );
  const items: AgentFeedItem[] = [{
    id: 'other-item', event_source: 'operations',
    context_payload: { description: 'Another pending proposal' },
    proposed_action: { message: 'Review owner work' },
    lifecycle_state: 'PENDING_APPROVAL', created_at: '2026-10-04T12:00:00Z',
  }];
  const writes: { id: string; state: string }[] = [];
  vi.stubGlobal('fetch', vi.fn(async (url: string, options?: RequestInit) => {
    if (url === '/api/v1/agent-feed') return Response.json({ items });
    const match = url.match(/^\/api\/v1\/agent-feed\/([^/]+)\/state$/);
    if (match && options?.method === 'PUT') {
      const payload = JSON.parse(String(options.body));
      writes.push({ id: match[1], state: payload.state });
      return Response.json({});
    }
    throw new Error(`Unexpected feed request: ${url}`);
  }));

  const locator = (query: () => HTMLElement[], label: string): Locator => ({
    query, label,
    filter: ({ hasText }) => locator(() => query().filter(node => node.textContent?.includes(hasText)), `${label} containing ${hasText}`),
    getByTestId: id => locator(() => query().flatMap(node => queryAllByTestId(node, id)), `${label} testid ${id}`),
    getByRole: (role, options) => locator(() => query().flatMap(node => queryAllByRole(node, role, options)), `${label} role ${role} ${options?.name}`),
    and: other => locator(() => query().filter(node => other.query().includes(node)), `${label} AND ${other.label}`),
    click: async () => {
      const nodes = query();
      expect(nodes, label).toHaveLength(1);
      await act(async () => { fireEvent.click(nodes[0]); });
    },
  });
  const page: Page = {
    goto: async url => {
      expect(url).toBe('/feed');
      cleanup();
      await act(async () => { render(<FeedPage />); });
      if (items.length === 2 && missingContract) {
        const card = queryAllByTestId(document.body, 'agent-feed-card')
          .find(node => node.textContent?.includes(`Owner proposal case-${action}`))!;
        const [button] = queryAllByTestId(card, `feed-${action}-btn`);
        if (missingContract === 'name') button.setAttribute('aria-label', 'Unexpected action');
        else button.removeAttribute('data-testid');
      }
    },
    getByTestId: id => locator(() => queryAllByTestId(document.body, id), `page testid ${id}`),
  };
  const assertions: string[] = [];
  const dependencies: Record<string, unknown> = {
    '../../../../e2e/fixtures': {
      test,
      expect: (target: Locator) => ({
        toBeVisible: async () => {
          const nodes = target.query();
          expect(nodes, target.label).toHaveLength(1);
          expect(nodes[0]).toBeVisible();
          assertions.push('visible');
        },
        toBeHidden: async () => {
          expect(target.query(), target.label).toHaveLength(0);
          assertions.push('hidden');
        },
      }),
    },
    '../../../../e2e/db_utils': { db: {} },
    '../../../../e2e/feed-fixtures': {
      seedFeedItem: async (_page: Page, payload: Pick<AgentFeedItem, 'event_source' | 'context_payload' | 'proposed_action'>) => {
        items.push({ ...payload, id: 'owned-item', lifecycle_state: 'PENDING_APPROVAL', created_at: '2026-10-04T12:00:00Z' });
        return 'owned-item';
      },
    },
  };
  vm.runInNewContext(compiledSpec, {
    exports: {},
    require: (name: string) => {
      expect(name in dependencies, `Unexpected spec dependency: ${name}`).toBe(true);
      return dependencies[name];
    },
  });
  await beforeCase({ page }, { testId: `case-${action}` });
  await cases.get(`Feed Page should load items and ${action}`)!({ page });
  expect(writes).toEqual([{ id: 'owned-item', state: action === 'approve' ? 'APPROVED' : 'DISMISSED' }]);
  expect(queryAllByTestId(document.body, 'agent-feed-card')).toHaveLength(1);
  expect(document.body).toHaveTextContent('Another pending proposal');
  expect(assertions).toEqual(['visible', 'visible', 'hidden']);
}

afterEach(() => { cleanup(); vi.unstubAllGlobals(); });

describe.each(['approve', 'dismiss'] as const)('/feed %s journey selectors', action => {
  it('selects and resolves its own rendered proposal while preserving another pending item', async () => {
    await runJourney(action);
  });
  it.each(['name', 'testid'] as const)('still requires the button %s contract', async contract => {
    await expect(runJourney(action, contract)).rejects.toThrow(/role button .* AND .*testid feed-/);
  });
});
