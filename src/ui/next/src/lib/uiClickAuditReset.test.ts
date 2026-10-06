import { describe, expect, it, vi } from 'vitest';
import { EventEmitter } from 'node:events';
import type { Page } from '@playwright/test';
import { replaceAuditDocument, resolveAuditTarget, auditDocumentSignature, hasFragmentTarget, installClickFocusProbe, installClickPopupProbe } from '../../../../e2e/support/ui_click_audit';

type FixtureHandle<T> = {
  _value: T;
  evaluate: <Result, Argument>(read: (value: T, argument: Argument) => Result, argument?: Argument | FixtureHandle<Argument>) => Promise<Result>;
  evaluateHandle: <Result, Argument>(read: (value: T, argument: Argument) => Result, argument?: Argument | FixtureHandle<Argument>) => Promise<FixtureHandle<Result>>;
  dispose: ReturnType<typeof vi.fn>;
};

function unwrap<Argument>(argument: Argument | FixtureHandle<Argument>): Argument {
  return argument && typeof argument === 'object' && '_value' in argument
    ? argument._value : argument as Argument;
}

// Each acquisition owns a fresh handle over its captured value. Execute the
// real adapter callbacks, including cross-handle document identity comparisons.
function valueHandle<T>(value: T): FixtureHandle<T> {
  return {
    _value: value,
    evaluate: async (read, argument) => read(value, unwrap(argument)),
    evaluateHandle: async (read, argument) => valueHandle(read(value, unwrap(argument))),
    dispose: vi.fn(),
  };
}

function documentPage(overrides: Record<string, unknown> = {}): Page {
  return {
    url: () => 'https://fixture.test/original',
    viewportSize: () => ({ width: 1280, height: 720 }),
    isClosed: () => false,
    evaluateHandle: vi.fn(async (read: () => unknown) => valueHandle(read())),
    evaluate: vi.fn(async (read: (argument: unknown) => unknown, argument?: unknown) => read(argument)),
    ...overrides,
  } as unknown as Page;
}

describe('audit document retirement', () => {
  it('commits a blank document on the existing page without creating another recording', async () => {
    const close = vi.fn();
    const newPage = vi.fn();
    let url = 'https://fixture.test/original';
    const goto = vi.fn(async (destination: string, options: { waitUntil: string; timeout: number }) => {
      expect(options.waitUntil).toBe('commit');
      url = destination;
      return null;
    });
    const page = documentPage({ url: () => url, close, goto, context: () => ({ newPage }) });
    for (let index = 0; index < 3; index += 1) {
      expect(await replaceAuditDocument(page)).toBe(page);
    }
    expect(goto.mock.calls).toEqual(Array.from({ length: 3 }, () => ['about:blank', { waitUntil: 'commit', timeout: expect.any(Number) }]));
    for (const [, options] of goto.mock.calls) {
      expect(options.timeout).toBeGreaterThan(0);
      expect(options.timeout).toBeLessThanOrEqual(10_000);
    }
    expect(close).not.toHaveBeenCalled();
    expect(newPage).not.toHaveBeenCalled();
  });

  it('does not claim retirement when the committed navigation fails', async () => {
    const error = new Error('navigation interrupted');
    const close = vi.fn();
    const newPage = vi.fn();
    const page = documentPage({ goto: vi.fn().mockRejectedValue(error), close, context: () => ({ newPage }) });
    await expect(replaceAuditDocument(page)).rejects.toBe(error);
    expect(close).not.toHaveBeenCalled();
    expect(newPage).not.toHaveBeenCalled();
  });
});


describe('audit target reacquisition before any action', () => {
  it('retags the same identity after React removes its old index', async () => {
    const target = { waitForElementState: vi.fn(), evaluate: vi.fn().mockResolvedValue(true), getAttribute: vi.fn().mockResolvedValue('same-control'), dispose: vi.fn(), click: vi.fn() };
    const elements = vi.fn().mockResolvedValueOnce([]).mockResolvedValueOnce([target]);
    const locator = vi.fn(() => ({ elementHandles: elements }));
    const page = documentPage({ url: () => 'https://fixture.test', locator, waitForTimeout: vi.fn().mockResolvedValue(undefined) });
    const retag = vi.fn().mockResolvedValueOnce([{ key: 'same-control', index: 2, label: 'Action' }]).mockResolvedValueOnce([{ key: 'same-control', index: 7, label: 'Action' }]);
    expect(await resolveAuditTarget(page, 'same-control', retag, 1000)).toBe(target);
    expect(locator.mock.calls).toEqual([['[data-ui-audit-click-index="2"]'], ['[data-ui-audit-click-index="7"]']]);
    expect(target.click).not.toHaveBeenCalled();
  });

  it('never hands back a same-named control from a different document', async () => {
    let url = 'https://fixture.test/original';
    const target = { waitForElementState: vi.fn(), evaluate: vi.fn().mockResolvedValue(true), getAttribute: vi.fn().mockResolvedValue('same-control'), dispose: vi.fn(), click: vi.fn() };
    const page = documentPage({ url: () => url, locator: () => ({ elementHandles: async () => [target] }), waitForTimeout: vi.fn().mockResolvedValue(undefined) });
    const retag = async () => { url = 'https://fixture.test/redirected'; return [{ key: 'same-control', index: 2, label: 'Action' }]; };
    await expect(resolveAuditTarget(page, 'same-control', retag, 1000)).rejects.toThrow('document changed');
    expect(target.dispose).toHaveBeenCalled();
    expect(target.click).not.toHaveBeenCalled();
  });

  it('rejects a replacement document even when its URL and control key are unchanged', async () => {
    let documents = [document];
    const original = valueHandle(documents);
    const replacement = document.implementation.createHTMLDocument('replacement');
    const target = { waitForElementState: vi.fn(), evaluate: vi.fn().mockResolvedValue(true), getAttribute: vi.fn().mockResolvedValue('same-control'), dispose: vi.fn(), click: vi.fn() };
    const page = documentPage({ url: () => 'https://fixture.test/same',
      evaluateHandle: vi.fn(async () => valueHandle(documents)).mockResolvedValueOnce(original),
      locator: () => ({ elementHandles: async () => [target] }), waitForTimeout: vi.fn() });
    const retag = async () => { documents = [replacement]; return [{ key: 'same-control', index: 1, label: 'Action' }]; };
    await expect(resolveAuditTarget(page, 'same-control', retag, 1000)).rejects.toThrow('document changed');
    expect(original.dispose).toHaveBeenCalledTimes(1);
    expect(target.click).not.toHaveBeenCalled();
  });

  it('resolves an embedded control through its owning frame without clicking it', async () => {
    const child = document.implementation.createHTMLDocument('embedded');
    const target = { waitForElementState: vi.fn(), evaluate: vi.fn().mockResolvedValue(true), getAttribute: vi.fn().mockResolvedValue('embedded-control'), dispose: vi.fn(), click: vi.fn() };
    const handles: FixtureHandle<Document[]>[] = [];
    const embeddedLocator = vi.fn(() => ({ elementHandles: async () => [target] }));
    const frameLocator = vi.fn(() => ({ locator: embeddedLocator }));
    const page = documentPage({ evaluateHandle: async () => {
      const handle = valueHandle([document, child]); handles.push(handle); return handle;
    }, locator: () => ({ elementHandles: async () => [] }), frameLocator });
    expect(await resolveAuditTarget(page, 'embedded-control', async () => [{ key: 'embedded-control', index: 4, label: 'Action' }], 1000)).toBe(target);
    expect(frameLocator).toHaveBeenCalledWith('iframe[data-ohc-api-docs-viewer]');
    expect(embeddedLocator).toHaveBeenCalledWith('[data-ui-audit-click-index="4"]');
    expect(target.dispose).not.toHaveBeenCalled();
    expect(target.click).not.toHaveBeenCalled();
    expect(handles.length).toBeGreaterThan(1);
    for (const handle of handles) expect(handle.dispose).toHaveBeenCalledTimes(1);
  });

  it('rejects a replaced embedded document while its parent, URL and control key stay unchanged', async () => {
    let child = document.implementation.createHTMLDocument('original embedded');
    const replacement = document.implementation.createHTMLDocument('replacement embedded');
    const handles: FixtureHandle<Document[]>[] = [];
    const target = { waitForElementState: vi.fn(), evaluate: vi.fn().mockResolvedValue(true), getAttribute: vi.fn().mockResolvedValue('same-control'), dispose: vi.fn(), click: vi.fn() };
    const page = documentPage({ url: () => 'https://fixture.test/same', evaluateHandle: async () => {
      const handle = valueHandle([document, child]); handles.push(handle); return handle;
    }, locator: () => ({ elementHandles: async () => [] }),
    frameLocator: () => ({ locator: () => ({ elementHandles: async () => [target] }) }), waitForTimeout: vi.fn() });
    const retag = async () => { child = replacement; return [{ key: 'same-control', index: 1, label: 'Action' }]; };
    await expect(resolveAuditTarget(page, 'same-control', retag, 1000)).rejects.toThrow('document changed');
    expect(target.dispose).toHaveBeenCalledTimes(1);
    expect(target.click).not.toHaveBeenCalled();
    expect(handles.length).toBeGreaterThan(1);
    for (const handle of handles) expect(handle.dispose).toHaveBeenCalledTimes(1);
  });

  it('rejects a different replacement identity without clicking it', async () => {
    const target = { waitForElementState: vi.fn(), evaluate: vi.fn().mockResolvedValue(true), getAttribute: vi.fn().mockResolvedValue('different-control'), dispose: vi.fn(), click: vi.fn() };
    const page = documentPage({ url: () => 'https://fixture.test', locator: () => ({ elementHandles: async () => [target] }), waitForTimeout: (time: number) => new Promise(resolve => setTimeout(resolve, time)) });
    const retag = async () => [{ key: 'same-control', index: 2, label: 'Action' }];
    await expect(resolveAuditTarget(page, 'same-control', retag, 30)).rejects.toThrow('same-control');
    expect(target.dispose).toHaveBeenCalled();
    expect(target.click).not.toHaveBeenCalled();
  });

  it('keeps the lookup budget when a retag operation never settles', async () => {
    const page = documentPage({ url: () => 'https://fixture.test' });
    const retag = vi.fn<() => Promise<never>>(() => new Promise(() => undefined));
    await expect(resolveAuditTarget(page, 'hung-control', retag, 30)).rejects.toThrow('hung-control');
    expect(retag).toHaveBeenCalledTimes(1);
  });

  it('disposes a handle that arrives after timeout without preparing or clicking it', async () => {
    const target = { dispose: vi.fn(), waitForElementState: vi.fn(), click: vi.fn() };
    let release!: (value: typeof target[]) => void;
    const handles = new Promise<typeof target[]>(resolve => { release = resolve; });
    const page = documentPage({ url: () => 'https://fixture.test', locator: () => ({ elementHandles: () => handles }) });
    const lookup = resolveAuditTarget(page, 'late-control', async () => [{ key: 'late-control', index: 1, label: 'Action' }], 30);
    await expect(lookup).rejects.toThrow('late-control');
    release([target]);
    await vi.waitFor(() => expect(target.dispose).toHaveBeenCalledTimes(1));
    expect(target.waitForElementState).not.toHaveBeenCalled();
    expect(target.click).not.toHaveBeenCalled();
  });

  it('fails visibly if the discovered control never returns', async () => {
    const page = documentPage({ url: () => 'https://fixture.test', waitForTimeout: (time: number) => new Promise(resolve => setTimeout(resolve, time)) });
    await expect(resolveAuditTarget(page, 'missing', async () => [], 30)).rejects.toThrow('missing');
  });
});


it('does not count the crawler own marker changes as a user effect', () => {
  const original = document.body.innerHTML;
  try {
    document.body.innerHTML = '<button data-ui-audit-click-index="1" data-ui-audit-click-key="first">Dead control</button>';
    const before = auditDocumentSignature();
    const control = document.querySelector('button')!;
    control.setAttribute('data-ui-audit-click-index', '2');
    control.removeAttribute('data-ui-audit-click-key');
    expect(auditDocumentSignature()).toBe(before);
    control.textContent = 'Actual visible feedback';
    expect(auditDocumentSignature()).not.toBe(before);
  } finally { document.body.innerHTML = original; }
});


describe('fragment destinations', () => {
  it('recognizes an actual section, encoded ID or named anchor in this document', () => {
    document.body.innerHTML = '<section id="cost-breakdown-section">Costs</section><h2 id="雪 section">Details</h2><a name="legacy-destination"></a>';
    try {
      expect(hasFragmentTarget('#cost-breakdown-section')).toBe(true);
      expect(hasFragmentTarget('#%E9%9B%AA%20section')).toBe(true);
      expect(hasFragmentTarget('#legacy-destination')).toBe(true);
    } finally { document.body.innerHTML = ''; }
  });
  it('keeps empty, missing, malformed and unrelated names invalid', () => {
    document.body.innerHTML = '<input name="not-an-anchor"><section id="present">Present</section>';
    try {
      for (const href of ['#', '#missing', '#%E0%A4', '#not-an-anchor', '/elsewhere#present']) {
        expect(hasFragmentTarget(href)).toBe(false);
      }
    } finally { document.body.innerHTML = ''; }
  });
});

it('does not let delayed field-edit effects certify an inert submit click', async () => {
  const { observeClickEffects, hasMeaningfulClickEffect } = await import('../../../../e2e/support/ui_click_audit');
  document.body.innerHTML = '<form><input type="email"><button>Dead submit</button><output></output></form>';
  const control = document.querySelector('input')!;
  const button = document.querySelector('button')!;
  const inputEffect = vi.fn();
  const timers: ReturnType<typeof setTimeout>[] = [];
  document.querySelector('form')!.addEventListener('submit', event => event.preventDefault());
  control.addEventListener('input', () => {
    timers.push(setTimeout(() => { inputEffect(); document.querySelector('output')!.textContent = 'Autosaved input'; }, 100));
  });
  const bounds = vi.spyOn(HTMLElement.prototype, 'getBoundingClientRect').mockReturnValue({ width: 100, height: 40 } as DOMRect);
  const disposals = new Map<unknown, { probe: ReturnType<typeof vi.fn>; handle: ReturnType<typeof vi.fn> }>();
  const originalOpen = window.open;
  const context = new EventEmitter();
  const pageEvents = new EventEmitter();
  const target = { evaluate: async (callback: (element: Element, argument?: unknown) => unknown, argument?: unknown) => callback(button, argument),
    evaluateHandle: async <T extends { dispose(): void },>(callback: (element: HTMLElement) => T) => {
      const probe = callback(button);
      const originalDispose = probe.dispose;
      const disposeProbe = vi.fn(() => originalDispose());
      const disposeHandle = vi.fn();
      probe.dispose = disposeProbe;
      disposals.set(callback, { probe: disposeProbe, handle: disposeHandle });
      return { evaluate: async <U,>(read: (value: T) => U) => read(probe), dispose: disposeHandle };
    },
    hover: vi.fn(), focus: vi.fn(), click: async () => button.click() };
  const page = documentPage({ url: () => window.location.href, evaluate: async (callback: (argument?: unknown) => unknown, argument?: unknown) => callback(argument),
    waitForTimeout: (ms: number) => new Promise(resolve => setTimeout(resolve, ms)),
    context: () => context, on: pageEvents.on.bind(pageEvents), off: pageEvents.off.bind(pageEvents) });
  try {
    const effect = await observeClickEffects(page as unknown as Page, target as never);
    expect(hasMeaningfulClickEffect(effect)).toBe(false);
    expect(effect.focusSeen).toBe(false);
    expect(disposals.get(installClickFocusProbe)?.probe).toHaveBeenCalledTimes(1);
    expect(disposals.get(installClickFocusProbe)?.handle).toHaveBeenCalledTimes(1);
    expect(disposals.get(installClickPopupProbe)?.probe).toHaveBeenCalledTimes(1);
    expect(disposals.get(installClickPopupProbe)?.handle).toHaveBeenCalledTimes(1);
    expect(window.open).toBe(originalOpen);
    expect(context.listenerCount('request')).toBe(0);
    expect(pageEvents.eventNames()).toEqual([]);
    expect(inputEffect).not.toHaveBeenCalled();
    expect(control.value).toBe('');
    expect(document.querySelector('output')).toHaveTextContent('');
  } finally { timers.forEach(clearTimeout); bounds.mockRestore(); document.body.innerHTML = ''; }
});

it('keeps distinct owned record controls identifiable when their DOM order changes', async () => {
  const { tagClickTargets } = await import('../../../../e2e/support/ui_click_audit');
  const original = document.body.innerHTML;
  const fixture = (namespace: string, records: string[]) => records.map(id => `<section data-testid="triage-card-${namespace}-${id}"><button>Dismiss</button></section>`).join('');
  const page = documentPage();
  const bounds = vi.spyOn(HTMLElement.prototype, 'getBoundingClientRect').mockReturnValue({ width: 100, height: 40 } as DOMRect);
  try {
    document.body.innerHTML = fixture('audit-case-one', ['first', 'second']);
    const initial = await tagClickTargets(page, 'audit-case-one');
    document.body.innerHTML = fixture('audit-case-two', ['second', 'first']);
    const restored = await tagClickTargets(page, 'audit-case-two');
    expect(initial).toHaveLength(2);
    expect(restored).toHaveLength(2);
    expect(restored[0].key).toBe(initial[1].key);
    expect(restored[1].key).toBe(initial[0].key);
    expect(new Set(restored.map(target => target.key)).size).toBe(2);
  } finally { bounds.mockRestore(); document.body.innerHTML = original; }
});

it('maps case-specific UUID ancestry back to the same canonical record identity', async () => {
  const { tagClickTargets } = await import('../../../../e2e/support/ui_click_audit');
  const original = document.body.innerHTML;
  const page = documentPage();
  const bounds = vi.spyOn(HTMLElement.prototype, 'getBoundingClientRect').mockReturnValue({ width: 100, height: 40 } as DOMRect);
  try {
    document.body.innerHTML = '<section data-testid="quote-first-generated-uuid"><button>Approve</button></section>';
    const first = await tagClickTargets(page, 'audit-case-one', { 'first-generated-uuid': 'canonical-quote-uuid' });
    document.body.innerHTML = '<section data-testid="quote-second-generated-uuid"><button>Approve</button></section>';
    const second = await tagClickTargets(page, 'audit-case-two', { 'second-generated-uuid': 'canonical-quote-uuid' });
    expect(first).toHaveLength(1);
    expect(second).toHaveLength(1);
    expect(second[0].key).toBe(first[0].key);
  } finally { bounds.mockRestore(); document.body.innerHTML = original; }
});
