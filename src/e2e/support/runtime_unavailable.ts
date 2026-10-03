import { expect, type Response } from '@playwright/test';

import { runtimeUnavailableMessage } from './runtime_policy';
export { runtimeUnavailableMessage } from './runtime_policy';

/** Exact deployed prerequisite, never a catch-all for authentication or transport errors. */
export async function expectRuntimeUnavailable(response: Response) {
  expect(response.status()).toBe(503);
  expect(await response.json()).toEqual({ error: runtimeUnavailableMessage });
}
