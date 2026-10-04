import type { Browser, BrowserContext, BrowserContextOptions } from '@playwright/test';

export function withOwnedBrowserContexts<Result>(
  browser: Browser,
  options: BrowserContextOptions,
  use: (contexts: [BrowserContext, BrowserContext]) => Promise<Result>,
): Promise<Result>;
