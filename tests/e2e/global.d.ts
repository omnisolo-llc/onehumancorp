/**
 * global.d.ts — Minimal stubs for TypeScript compile checks that run in a
 * Bazel sandbox without node_modules (no @types/node, no @playwright/test).
 *
 * When running via `npm test` (full Playwright run), the real @playwright/test
 * package and @types/node take precedence and this file is effectively a no-op.
 */

// ── Node.js process global ────────────────────────────────────────────────────
declare namespace NodeJS {
  interface ProcessEnv {
    [key: string]: string | undefined;
  }
}
declare const process: { readonly env: NodeJS.ProcessEnv };

// ── @playwright/test stubs ────────────────────────────────────────────────────
declare module '@playwright/test' {
  // Minimal Page interface — only the methods used in ohc-cuj.spec.ts.
  export interface Page {
    goto(url: string): Promise<void>;
    waitForLoadState(state?: string): Promise<void>;
    waitForSelector(selector: string, options?: { timeout?: number }): Promise<unknown>;
    waitForTimeout(ms: number): Promise<void>;
    waitForRequest(urlOrPredicate: string | ((req: Request) => boolean), options?: { timeout?: number }): Promise<Request>;
    locator(selector: string): Locator;
    getByText(text: string | RegExp): Locator;
    on(event: string, handler: (arg: unknown) => void): void;
    keyboard: { press(key: string): Promise<void> };
  }

  // Minimal Locator interface.
  export interface Locator {
    filter(options: { hasText?: string | RegExp }): Locator;
    first(): Locator;
    nth(index: number): Locator;
    or(other: Locator): Locator;
    click(options?: unknown): Promise<void>;
    fill(value: string): Promise<void>;
    press(key: string): Promise<void>;
    check(): Promise<void>;
    uncheck(): Promise<void>;
    selectOption(value: unknown): Promise<void>;
    getAttribute(name: string): Promise<string | null>;
    inputValue(): Promise<string>;
    textContent(): Promise<string | null>;
    allTextContents(): Promise<string[]>;
    isVisible(options?: { timeout?: number }): Promise<boolean>;
    isDisabled(): Promise<boolean>;
    isChecked(): Promise<boolean>;
    count(): Promise<number>;
  }

  // Minimal Request stub.
  export interface Request {
    url(): string;
  }

  // expect — returns a chainable assertion object.
  export function expect(value: unknown): {
    toBeVisible(options?: { timeout?: number }): Promise<void>;
    not: {
      toBeVisible(options?: { timeout?: number }): Promise<void>;
      toContainText(text: string | RegExp): Promise<void>;
    };
    toHaveValue(value: string): Promise<void>;
    toHaveAttribute(name: string, value: string): Promise<void>;
    toBeEnabled(): Promise<void>;
    toBeChecked(): Promise<void>;
    toContainText(text: string | RegExp): Promise<void>;
    toBe(expected: unknown): void;
    toBeGreaterThanOrEqual(n: number): void;
    toBeGreaterThan(n: number): void;
    toMatch(pattern: RegExp): void;
    not: unknown;
  };

  // test — the Playwright test function.
  export const test: {
    (name: string, fn: (args: { page: Page }) => Promise<void>): void;
    describe(name: string, fn: () => void): void;
    beforeEach(fn: (args: { page: Page }) => Promise<void>): void;
    afterEach(fn: (args: { page: Page }) => Promise<void>): void;
  };
}
