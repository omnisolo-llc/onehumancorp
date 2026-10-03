import { describe, expect, it, vi } from 'vitest';
import type { Page } from '@playwright/test';
import { replaceAuditDocument, resolveAuditTarget, auditDocumentSignature, hasFragmentTarget, installClickFocusProbe } from '../../../../e2e/support/ui_click_audit';

function documentPage(overrides: Record<string, unknown>): Page {
  return {
    evaluateHandle: vi.fn().mockResolvedValue({ dispose: vi.fn() }),
    evaluate: vi.fn().mockResolvedValue(true),
    ...overrides,
  } as unknown as Page;
}

describe('audit document retirement', () => {
  it('commits a blank document on the existing page without creating another recording', async () => {
    const close = vi.fn();
    const newPage = vi.fn();
    const goto = vi.fn().mockResolvedValue(null);
    const page = documentPage({ close, goto, context: () => ({ newPage }) });
    for (let index = 0; index < 3; index += 1) {
      expect(await replaceAuditDocument(page)).toBe(page);
    }
    expect(goto.mock.calls).toEqual(Array.from({ length: 3 }, () => ['about:blank', { waitUntil: 'load' }]));
    expect(close).not.toHaveBeenCalled();
    expect(newPage).not.toHaveBeenCalled();
  });

  it('does not claim retirement when the committed navigation fails', async () => {
    const page = documentPage({ goto: vi.fn().mockRejectedValue(new Error('navigation interrupted')), close: vi.fn(), context: () => ({ newPage: vi.fn() }) });
    await expect(replaceAuditDocument(page)).rejects.toThrow('navigation interrupted');
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
    const original = { dispose: vi.fn() };
    let sameDocument = true;
    const target = { waitForElementState: vi.fn(), evaluate: vi.fn().mockResolvedValue(true), getAttribute: vi.fn().mockResolvedValue('same-control'), dispose: vi.fn(), click: vi.fn() };
    const page = documentPage({ url: () => 'https://fixture.test/same', evaluateHandle: async () => original,
      evaluate: async () => sameDocument, locator: () => ({ elementHandles: async () => [target] }), waitForTimeout: vi.fn() });
    const retag = async () => { sameDocument = false; return [{ key: 'same-control', index: 1, label: 'Action' }]; };
    await expect(resolveAuditTarget(page, 'same-control', retag, 1000)).rejects.toThrow('document changed');
    expect(original.dispose).toHaveBeenCalledTimes(1);
    expect(target.click).not.toHaveBeenCalled();
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
    await expect(resolveAuditTarget(page, 'hung-control', () => new Promise(() => undefined), 30)).rejects.toThrow('hung-control');
  });

  it('disposes a handle that arrives after timeout without preparing or clicking it', async () => {
    const target = { dispose: vi.fn(), waitForElementState: vi.fn(), click: vi.fn() };
    let release!: (value: typeof target[]) => void;
    const handles = new Promise<typeof target[]>(resolve => { release = resolve; });
    const page = documentPage({ url: () => 'https://fixture.test', locator: () => ({ elementHandles: () => handles }) });
    const lookup = resolveAuditTarget(page, 'late-control', async () => [{ key: 'late-control', index: 1, label: 'Action' }], 30);
    await expect(lookup).rejects.toThrow('late-control');
    release([target]);
    await Promise.resolve();
    await Promise.resolve();
    expect(target.dispose).toHaveBeenCalledTimes(1);
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
  const disposeHandle = vi.fn();
  const disposeProbe = vi.fn();
  const target = { evaluate: async (callback: (element: Element, argument?: unknown) => unknown, argument?: unknown) => callback(button, argument),
    evaluateHandle: async (callback: typeof installClickFocusProbe) => {
      const probe = callback(button);
      const originalDispose = probe.dispose;
      probe.dispose = () => { disposeProbe(); originalDispose(); };
      return { evaluate: async <T,>(read: (value: typeof probe) => T) => read(probe), dispose: disposeHandle };
    },
    hover: vi.fn(), focus: vi.fn(), click: async () => button.click() };
  const page = { url: () => window.location.href, evaluate: async (callback: (argument?: unknown) => unknown, argument?: unknown) => callback(argument),
    waitForTimeout: (ms: number) => new Promise(resolve => setTimeout(resolve, ms)), on: vi.fn(), off: vi.fn() };
  try {
    const effect = await observeClickEffects(page as unknown as Page, target as never);
    expect(hasMeaningfulClickEffect(effect)).toBe(false);
    expect(effect.focusSeen).toBe(false);
    expect(disposeProbe).toHaveBeenCalledTimes(1);
    expect(disposeHandle).toHaveBeenCalledTimes(1);
    expect(inputEffect).not.toHaveBeenCalled();
    expect(control.value).toBe('');
    expect(document.querySelector('output')).toHaveTextContent('');
  } finally { timers.forEach(clearTimeout); bounds.mockRestore(); document.body.innerHTML = ''; }
});

it('keeps distinct owned record controls identifiable when their DOM order changes', async () => {
  const { tagClickTargets } = await import('../../../../e2e/support/ui_click_audit');
  const original = document.body.innerHTML;
  const fixture = (namespace: string, records: string[]) => records.map(id => `<section data-testid="triage-card-${namespace}-${id}"><button>Dismiss</button></section>`).join('');
  const page = { locator: () => ({ evaluateAll: async (read: (elements: Element[], namespace?: string) => unknown, namespace?: string) => read(Array.from(document.querySelectorAll('button')), namespace) }) } as unknown as Page;
  try {
    document.body.innerHTML = fixture('audit-case-one', ['first', 'second']);
    const initial = await tagClickTargets(page, 'audit-case-one');
    document.body.innerHTML = fixture('audit-case-two', ['second', 'first']);
    const restored = await tagClickTargets(page, 'audit-case-two');
    expect(restored[0].key).toBe(initial[1].key);
    expect(restored[1].key).toBe(initial[0].key);
    expect(new Set(restored.map(target => target.key)).size).toBe(2);
  } finally { document.body.innerHTML = original; }
});

it('maps case-specific UUID ancestry back to the same canonical record identity', async () => {
  const { tagClickTargets } = await import('../../../../e2e/support/ui_click_audit');
  const original = document.body.innerHTML;
  const page = { locator: () => ({ evaluateAll: async (read: (elements: Element[], identity: unknown) => unknown, identity: unknown) => read(Array.from(document.querySelectorAll('button')), identity) }) } as unknown as Page;
  try {
    document.body.innerHTML = '<section data-testid="quote-first-generated-uuid"><button>Approve</button></section>';
    const first = await tagClickTargets(page, 'audit-case-one', { 'first-generated-uuid': 'canonical-quote-uuid' });
    document.body.innerHTML = '<section data-testid="quote-second-generated-uuid"><button>Approve</button></section>';
    const second = await tagClickTargets(page, 'audit-case-two', { 'second-generated-uuid': 'canonical-quote-uuid' });
    expect(second[0].key).toBe(first[0].key);
  } finally { document.body.innerHTML = original; }
});
