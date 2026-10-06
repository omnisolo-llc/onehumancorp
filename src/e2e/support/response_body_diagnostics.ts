import type { Response } from '@playwright/test';

type BodyReceipt = {
  captureState: 'complete' | 'error' | 'incomplete';
  body?: string;
  finishedError?: Error | null;
  captureError?: string;
  completedAtMs: number;
  timing: ReturnType<ReturnType<Response['request']>['timing']>;
};
type BodyCapture = { result: Promise<BodyReceipt>; incomplete: () => BodyReceipt };

export function observeResponseBody(response: Pick<Response, 'text' | 'finished' | 'request'>, startedAt: number): BodyCapture {
  const stamp = () => ({ completedAtMs: Date.now() - startedAt, timing: { ...response.request().timing() } });
  return {
    result: (async () => {
      try {
        const body = await response.text();
        const finishedError = await response.finished();
        return { captureState: 'complete' as const, body, finishedError, ...stamp() };
      } catch (error) {
        return { captureState: 'error' as const, captureError: String(error), ...stamp() };
      }
    })(),
    incomplete: () => ({ captureState: 'incomplete', captureError: 'Supplemental response body capture did not finish before its diagnostic deadline.', ...stamp() }),
  };
}

export async function finishResponseBodyDiagnostics(captures: BodyCapture[], timeoutMs = 1000): Promise<BodyReceipt[]> {
  const receipts: (BodyReceipt | undefined)[] = new Array(captures.length);
  let accepting = true;
  const completed = Promise.all(captures.map(async (capture, index) => {
    const receipt = await capture.result;
    if (accepting) receipts[index] = receipt;
  }));
  let timer: ReturnType<typeof setTimeout> | undefined;
  try {
    await Promise.race([
      completed,
      new Promise<void>(resolve => { timer = setTimeout(resolve, timeoutMs); }),
    ]);
  } finally {
    accepting = false;
    clearTimeout(timer);
  }
  // Incomplete reads may only settle when Playwright tears down the context.
  // Keep their rejection handlers, but never mutate an already attached result.
  return captures.map((capture, index) => receipts[index] ?? capture.incomplete());
}
