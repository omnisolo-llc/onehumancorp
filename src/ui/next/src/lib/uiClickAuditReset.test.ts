import { describe, expect, it, vi } from 'vitest';
import type { Page } from '@playwright/test';
import { replaceAuditDocument, resolveAuditTarget, auditDocumentSignature } from '../../../../e2e/support/ui_click_audit';

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
