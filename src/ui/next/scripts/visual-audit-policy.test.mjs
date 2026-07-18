import { describe, expect, it } from 'vitest';
import {
  classifyConsoleError,
  failureReasons,
  isCoverageComplete,
  shouldFailAudit,
} from './visual-audit-policy.mjs';

const expectedCases = [
  { route: '/dashboard', viewportName: 'desktop' },
  { route: '/inbox', viewportName: 'mobile' },
];

function healthyResult(route, viewport) {
  return {
    route,
    viewport,
    attempted: true,
    completed: true,
    status: 200,
    finalPathname: route,
    shellCounts: { sidebar: 1, topbar: 1, main: 1 },
    horizontalOverflow: false,
    documentWidth: 390,
    consoleErrors: [],
    expectedServiceErrors: [],
    unexpectedConsoleErrors: [],
    hydrationErrors: [],
    pageErrors: [],
    navigationError: null,
    captureError: null,
    screenshotError: null,
    screenshotWritten: true,
  };
}

describe('visual audit policy', () => {
  it('narrowly allows a known isolated resource failure on a private LAN origin', () => {
    expect(classifyConsoleError({
      message: 'Failed to load resource: the server responded with a status of 500 (Internal Server Error)',
      locationUrl: 'http://192.168.8.35:3000/api/v1/health',
      pageUrl: 'http://192.168.8.35:3000/dashboard',
    })).toBe('expected-service');
  });

  it('does not allow isolated-service failures from a public development host', () => {
    expect(classifyConsoleError({
      message: 'Failed to load resource: the server responded with a status of 502 (Bad Gateway)',
      locationUrl: 'http://203.0.113.8:3000/api/v1/health',
      pageUrl: 'http://203.0.113.8:3000/dashboard',
    })).toBe('unexpected');
  });

  it('allows the optional help chrome to fall back while the backend is offline', () => {
    expect(classifyConsoleError({
      message: 'Failed to load resource: the server responded with a status of 502 (Bad Gateway)',
      locationUrl: 'http://192.168.8.35:3000/api/v1/help',
      pageUrl: 'http://192.168.8.35:3000/dashboard',
    })).toBe('expected-service');
    expect(classifyConsoleError({
      message: 'Failed to load tooltips Error: Failed to load tooltips, status: 502',
      locationUrl: 'webpack-internal:///(app-pages-browser)/./node_modules/next/dist/next-devtools/userspace/app/errors/intercept-console-error.js',
      pageUrl: 'http://192.168.8.35:3000/dashboard',
    })).toBe('expected-service');
  });

  it('does not blanket-allow the same status on an unknown API path', () => {
    expect(classifyConsoleError({
      message: 'Failed to load resource: the server responded with a status of 500 (Internal Server Error)',
      locationUrl: 'http://127.0.0.1:3000/api/v1/payments/charge',
      pageUrl: 'http://127.0.0.1:3000/dashboard',
    })).toBe('unexpected');
  });

  it('allows exact page-data fallbacks while the local backend is unavailable', () => {
    for (const [status, pathname] of [
      [404, '/api/v1/ledger/entries'],
      [502, '/api/v1/ui/omni_inbox'],
      [403, '/api/v1/growth/referrals/generate'],
      [404, '/api/v1/staff/shifts'],
      [401, '/api/v1/help'],
    ]) {
      expect(classifyConsoleError({
        message: `Failed to load resource: the server responded with a status of ${status} (Expected local fallback)`,
        locationUrl: `http://192.168.8.35:3000${pathname}`,
        pageUrl: 'http://192.168.8.35:3000/dashboard',
      }), pathname).toBe('expected-service');
    }
  });

  it('classifies hydration signatures independently', () => {
    expect(classifyConsoleError({
      message: 'Hydration failed because the initial UI does not match what was rendered on the server.',
      locationUrl: 'http://127.0.0.1:3000/_next/static/chunks/app.js',
      pageUrl: 'http://127.0.0.1:3000/inbox',
    })).toBe('hydration');
  });

  it('fails uncaught page errors, hydration errors, unexpected console errors, and screenshot errors', () => {
    const result = healthyResult('/dashboard', 'desktop');
    result.pageErrors.push({ message: 'synthetic uncaught exception', locationUrl: '' });
    result.hydrationErrors.push({ message: 'synthetic hydration failure', locationUrl: '' });
    result.unexpectedConsoleErrors.push({ message: 'synthetic console failure', locationUrl: '' });
    result.screenshotError = 'synthetic screenshot failure';

    expect(failureReasons(result)).toEqual(expect.arrayContaining([
      'uncaught page error',
      'hydration error',
      'unexpected console error',
      'screenshot error',
    ]));
    expect(shouldFailAudit({
      results: [result, healthyResult('/inbox', 'mobile')],
      expectedCases,
      fatalError: null,
      outputReady: true,
    })).toBe(true);
  });

  it('keeps narrowly classified isolated-service errors nonfatal', () => {
    const result = healthyResult('/dashboard', 'desktop');
    result.expectedServiceErrors.push({ message: 'known missing service', locationUrl: '/api/v1/ui/swagger-ui.css' });
    expect(failureReasons(result)).toEqual([]);
    expect(shouldFailAudit({
      results: [result, healthyResult('/inbox', 'mobile')],
      expectedCases,
      fatalError: null,
      outputReady: true,
    })).toBe(false);
  });

  it('requires every expected case and successful screenshot coverage', () => {
    const complete = [healthyResult('/dashboard', 'desktop'), healthyResult('/inbox', 'mobile')];
    expect(isCoverageComplete(complete, expectedCases)).toBe(true);
    expect(isCoverageComplete(complete.slice(0, 1), expectedCases)).toBe(false);

    complete[1].screenshotWritten = false;
    expect(isCoverageComplete(complete, expectedCases)).toBe(false);
  });

  it('fails pages that silently redirect to the login screen', () => {
    const result = healthyResult('/dashboard', 'desktop');
    result.finalPathname = '/login';
    expect(failureReasons(result)).toContain('unexpected redirect to /login');
  });

  it('allows the share-card redirect contract and no other onboarding redirect', () => {
    const shareCard = healthyResult('/share-card', 'desktop');
    shareCard.finalPathname = '/onboarding';
    expect(failureReasons(shareCard)).toEqual([]);

    const unrelated = healthyResult('/dashboard', 'desktop');
    unrelated.finalPathname = '/onboarding';
    expect(failureReasons(unrelated)).toContain('unexpected redirect to /onboarding');
  });

  it('makes policy failures produce a failing audit decision', () => {
    const results = [healthyResult('/dashboard', 'desktop'), healthyResult('/inbox', 'mobile')];
    results[0].pageErrors.push({ message: 'synthetic pageerror', locationUrl: '' });
    expect(shouldFailAudit({ results, expectedCases, fatalError: null, outputReady: true })).toBe(true);

    results[0].pageErrors = [];
    results[1].screenshotError = 'capture failed';
    results[1].screenshotWritten = false;
    expect(shouldFailAudit({ results, expectedCases, fatalError: null, outputReady: true })).toBe(true);
  });
});
