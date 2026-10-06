import { EventEmitter } from 'node:events';
import type { Page } from '@playwright/test';
import { afterEach, expect, it, vi } from 'vitest';
import { observeClickEffects, hasMeaningfulClickEffect } from '../../../../e2e/support/ui_click_audit';

function handle<T>(value: T) {
  return {
    evaluate: async <Result, Argument>(read: (value: T, argument: Argument) => Result, argument: Argument) => read(value, argument),
    evaluateHandle: async <Result, Argument>(read: (value: T, argument: Argument) => Result, argument: Argument) => handle(read(value, argument)),
    dispose: vi.fn(),
  };
}

afterEach(() => {
  document.body.innerHTML = '';
  vi.restoreAllMocks();
});

for (const embedded of [false, true]) for (const changes of [false, true]) it(`keeps ${embedded ? 'frame-owned' : 'parent-owned'} ${changes ? 'working' : 'inert'} click preparation from scrolling an already hovered control`, async () => {
  const frame = document.createElement('iframe');
  if (embedded) document.body.append(frame);
  const owner = embedded ? frame.contentDocument! : document;
  owner.body.innerHTML = '<button aria-expanded="false">Read operation</button>';
  const button = owner.querySelector('button')!;
  const focus = button.focus.bind(button);
  const preparedFocus = vi.spyOn(button, 'focus').mockImplementation(options => {
    // JSDOM has no layout. Assert the native focus contract that avoids the
    // hover -> implicit focus scroll -> sticky-header interception seen in CI.
    expect(options).toEqual({ preventScroll: true });
    focus(options);
  });
  if (changes) button.addEventListener('click', () => button.setAttribute('aria-expanded', 'true'));
  const click = vi.fn(async (options: { timeout: number; scroll?: 'auto' | 'none' }) => {
    // Preserve the hovered position. Re-scrolling this tall frame at click
    // time can place its operation under the sticky parent application header.
    if (options.scroll !== 'none') throw new Error('Parent header intercepts the re-scrolled operation');
    button.click();
  });
  const target = { ...handle(button), hover: vi.fn(), focus: async () => button.focus(), click };
  const context = new EventEmitter();
  const page = Object.assign(new EventEmitter(), {
    url: () => 'http://localhost/api-docs', context: () => context,
    evaluateHandle: async () => handle(embedded ? [document, owner] : [document]),
    waitForTimeout: async () => {},
  });

  const effect = await observeClickEffects(page as unknown as Page, target as never);
  expect(preparedFocus).toHaveBeenCalledTimes(1);
  expect(owner.activeElement).toBe(button);
  expect(target.hover).toHaveBeenCalledWith({ timeout: 5000 });
  expect(click).toHaveBeenCalledExactlyOnceWith({ timeout: 5000, scroll: 'none' });
  expect(button).toHaveAttribute('aria-expanded', String(changes));
  expect(effect.changed).toBe(changes);
  expect(effect.focusSeen).toBe(false);
  expect(hasMeaningfulClickEffect(effect)).toBe(changes);
});
