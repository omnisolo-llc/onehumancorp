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
  it('narrowly allows a known isolated Swagger resource failure', () => {
    expect(classifyConsoleError({
      message: 'Failed to load resource: the server responded with a status of 500 (Internal Server Error)',
      locationUrl: 'http://127.0.0.1:3000/api/ui/swagger-ui.css',
      pageUrl: 'http://127.0.0.1:3000/dashboard',
    })).toBe('expected-service');
  });

  it('does not blanket-allow the same status on an unknown API path', () => {
    expect(classifyConsoleError({
      message: 'Failed to load resource: the server responded with a status of 500 (Internal Server Error)',
      locationUrl: 'http://127.0.0.1:3000/api/payments/charge',
      pageUrl: 'http://127.0.0.1:3000/dashboard',
    })).toBe('unexpected');
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
    result.expectedServiceErrors.push({ message: 'known missing service', locationUrl: '/api/ui/swagger-ui.css' });
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
