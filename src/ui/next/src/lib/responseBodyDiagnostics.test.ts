import { afterEach, expect, it, vi } from 'vitest';
import type { Response } from '@playwright/test';
import { finishResponseBodyDiagnostics, observeResponseBody } from '../../../../e2e/support/response_body_diagnostics';

function response(text: () => Promise<string>, finished: () => Promise<Error | null> = async () => null) {
  return { text, finished, request: () => ({ timing: () => ({ responseEnd: -1 }) }) } as Pick<Response, 'text' | 'finished' | 'request'>;
}

afterEach(() => vi.useRealTimers());

it.each(['body', 'finished'] as const)('bounds a never-finishing supplemental %s without losing completed receipts', async phase => {
  vi.useFakeTimers();
  const captures = [
    observeResponseBody(response(
      () => phase === 'body' ? new Promise(() => {}) : Promise.resolve('body read'),
      () => phase === 'finished' ? new Promise(() => {}) : Promise.resolve(null),
    ), Date.now()),
    observeResponseBody(response(async () => 'actual acknowledged response'), Date.now()),
  ];
  let finished = false;
  const result = finishResponseBodyDiagnostics(captures).then(value => { finished = true; return value; });
  await vi.advanceTimersByTimeAsync(999);
  expect(finished).toBe(false);
  await vi.advanceTimersByTimeAsync(1);
  expect(finished).toBe(true);
  expect(await result).toMatchObject([
    { captureState: 'incomplete', captureError: expect.stringContaining('diagnostic deadline'), timing: { responseEnd: -1 } },
    { captureState: 'complete', body: 'actual acknowledged response', finishedError: null },
  ]);
  expect(vi.getTimerCount()).toBe(0);
});

it('preserves normal completion and clears the diagnostic timer early', async () => {
  vi.useFakeTimers();
  const result = await finishResponseBodyDiagnostics([
    observeResponseBody(response(async () => 'actual body'), Date.now()),
  ]);
  expect(result[0]).toMatchObject({ captureState: 'complete', body: 'actual body', finishedError: null });
  expect(vi.getTimerCount()).toBe(0);
});

it('records body errors rather than rejecting or erasing the diagnostic receipt', async () => {
  vi.useFakeTimers();
  const result = await finishResponseBodyDiagnostics([
    observeResponseBody(response(async () => { throw new Error('request canceled'); }), Date.now()),
  ]);
  expect(result[0]).toMatchObject({ captureState: 'error', captureError: 'Error: request canceled' });
  expect(vi.getTimerCount()).toBe(0);
});

it.each(['resolve', 'reject'] as const)('does not mutate returned evidence when an incomplete body later %ss', async outcome => {
  vi.useFakeTimers();
  let resolve!: (body: string) => void;
  let reject!: (error: Error) => void;
  const body = new Promise<string>((yes, no) => { resolve = yes; reject = no; });
  const pending = finishResponseBodyDiagnostics([observeResponseBody(response(() => body), Date.now())]);
  let settled = false;
  pending.then(() => { settled = true; });
  await vi.advanceTimersByTimeAsync(1000);
  expect(settled).toBe(true);
  const result = await pending;
  const snapshot = JSON.stringify(result);
  if (outcome === 'resolve') resolve('late body');
  else reject(new Error('context closed'));
  await vi.runAllTimersAsync();
  expect(JSON.stringify(result)).toBe(snapshot);
  expect(result[0].captureState).toBe('incomplete');
  expect(vi.getTimerCount()).toBe(0);
});

it('snapshots Playwright live timing objects before returning incomplete evidence', async () => {
  vi.useFakeTimers();
  const timing = { responseEnd: -1 };
  const source = response(() => new Promise(() => {}));
  source.request = () => ({ timing: () => timing }) as ReturnType<Response['request']>;
  const pending = finishResponseBodyDiagnostics([observeResponseBody(source, Date.now())]);
  await vi.advanceTimersByTimeAsync(1000);
  const result = await pending;
  timing.responseEnd = 4000;
  expect(result[0].timing.responseEnd).toBe(-1);
});
