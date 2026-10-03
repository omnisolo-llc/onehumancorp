// @vitest-environment-options {"url":"http://127.0.0.1:44041"}
import React from 'react';
import { render, waitFor } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import EmbedBuilder from '../embed-builder/page';
import InsightWidget from '../interactive-insight-widget/page';
import ShareCards from '../share-cards/page';
import ShareToUnlock from '../share-to-unlock-generator/page';
import Countdown from '../viral-countdown-widget/page';
import GoalTracker from '../viral-goal-tracker/page';
import Wrapped from '../wrapped/page';

// These tests exercise the real page output, with explicit transport/account
// boundaries. Keep the URLs visible: the audit must classify errors correctly.
vi.mock('./AppShell', () => ({ AppShell: ({ children }: { children: React.ReactNode }) => <main>{children}</main> }));
vi.mock('./useProPlan', () => ({ useProPlan: () => ({ hasPro: false, claimTrial: vi.fn(), claimError: null }) }));

beforeEach(() => {
  localStorage.clear();
  localStorage.setItem('business_display_name', 'e2e-tenant');
  vi.stubGlobal('fetch', vi.fn<typeof fetch>(async () => Response.json({ total_sales: 123, pending_orders: 4, top_product: 'Recorded product' })));
});

describe('displayed URLs on the exact shard4 origin', () => {
  it.each([
    ['/embed-builder', EmbedBuilder],
    ['/interactive-insight-widget', InsightWidget],
    ['/share-cards', ShareCards],
    ['/share-to-unlock-generator', ShareToUnlock],
    ['/viral-countdown-widget', Countdown],
    ['/viral-goal-tracker', GoalTracker],
    ['/wrapped', Wrapped],
  ] as const)('%s preserves its generated URL without a page error', async (route, Component) => {
    const origin = 'http://127.0.0.1:44041';
    expect(window.location.origin).toBe(origin);
    render(<Component />);
    await waitFor(() => expect(document.body.textContent).toContain(origin));
    const text = document.body.textContent || '';
    // Reproduce the previous false positive using maintained page components.
    expect(text).toMatch(/404/);
    expect(text.replaceAll(origin, '')).not.toMatch(/404|not found|application error|failed to load/i);
    console.info(`${route}: ${text.slice(text.indexOf(origin), text.indexOf(origin) + 160)}`);
  });
});
