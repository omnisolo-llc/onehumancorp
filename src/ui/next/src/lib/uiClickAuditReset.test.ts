import { describe, expect, it, vi } from 'vitest';
import type { Page } from '@playwright/test';
import { replaceAuditDocument } from '../../../../e2e/support/ui_click_audit';

describe('audit document retirement', () => {
  it('commits a blank document on the existing page without creating another recording', async () => {
    const close = vi.fn();
    const newPage = vi.fn();
    const goto = vi.fn().mockResolvedValue(null);
    const page = { close, goto, context: () => ({ newPage }) } as unknown as Page;
    for (let index = 0; index < 3; index += 1) {
      expect(await replaceAuditDocument(page)).toBe(page);
    }
    expect(goto.mock.calls).toEqual(Array.from({ length: 3 }, () => ['about:blank', { waitUntil: 'load' }]));
    expect(close).not.toHaveBeenCalled();
    expect(newPage).not.toHaveBeenCalled();
  });

  it('does not claim retirement when the committed navigation fails', async () => {
    const page = { goto: vi.fn().mockRejectedValue(new Error('navigation interrupted')), close: vi.fn(), context: () => ({ newPage: vi.fn() }) } as unknown as Page;
    await expect(replaceAuditDocument(page)).rejects.toThrow('navigation interrupted');
  });
});
