import React from 'react';
import { act, cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { invalidateQueueOwner } from '@/lib/sync/queueIdentity';
import ReferralWidgetBuilderPage from './page';

const clipboardDescriptor = Object.getOwnPropertyDescriptor(navigator, 'clipboard');
let tenant = 'referral-tenant';
function clipboard(writeText?: (value: string) => Promise<void>) {
  Object.defineProperty(navigator, 'clipboard', { configurable: true, value: writeText ? { writeText } : undefined });
}
function deferred() {
  let resolve!: () => void;
  const promise = new Promise<void>(yes => { resolve = yes; });
  return { promise, resolve };
}
async function openPreview() {
  await waitFor(() => expect(screen.getByRole('button', { name: 'Get Embed Code' })).toBeEnabled());
  fireEvent.click(screen.getByRole('button', { name: 'Get Embed Code' }));
  return screen.getByRole('textbox', { name: 'Embed Code' }) as HTMLTextAreaElement;
}
beforeEach(() => {
  localStorage.clear(); act(() => invalidateQueueOwner()); tenant = 'referral-tenant';
  vi.stubGlobal('fetch', vi.fn(async url => Response.json(url === '/api/v1/auth/session-identity' ? { userId: `owner-${tenant}`, tenantId: tenant, expiresAt: Date.now() + 60000 } : { current_plan: 'Free' })));
});
afterEach(() => {
  cleanup(); act(() => invalidateQueueOwner()); vi.unstubAllGlobals();
  if (clipboardDescriptor) Object.defineProperty(navigator, 'clipboard', clipboardDescriptor);
  else Reflect.deleteProperty(navigator, 'clipboard');
});

describe('Referral widget authenticated preview copy', () => {
  it('copies exactly the displayed private preview only after a click and acknowledges fulfillment', async () => {
    const pending = deferred(); const write = vi.fn(() => pending.promise); clipboard(write);
    render(<ReferralWidgetBuilderPage />);
    const displayed = (await openPreview()).value;
    expect(write).not.toHaveBeenCalled();
    fireEvent.click(screen.getByRole('button', { name: 'Copy Code' }));
    expect(write).toHaveBeenCalledExactlyOnceWith(displayed);
    expect(screen.queryByText('Copied to clipboard.')).not.toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Copy Code' })).toBeDisabled();
    await act(async () => pending.resolve());
    expect(screen.getByRole('status')).toHaveTextContent('Copied to clipboard.');
  });
  it('shows truthful failure after a rejected clipboard write', async () => {
    clipboard(vi.fn().mockRejectedValue(new Error('permission denied')));
    render(<ReferralWidgetBuilderPage />); await openPreview();
    fireEvent.click(screen.getByRole('button', { name: 'Copy Code' }));
    expect(await screen.findByRole('alert')).toHaveTextContent('Copy failed');
    expect(screen.queryByText('Copied to clipboard.')).not.toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Copy Code' })).toBeEnabled();
  });
  it('offers manual copying when the Clipboard API is unavailable', async () => {
    clipboard(); render(<ReferralWidgetBuilderPage />); await openPreview();
    fireEvent.click(screen.getByRole('button', { name: 'Copy Code' }));
    expect(await screen.findByRole('alert')).toHaveTextContent('Select the content and copy it manually');
    expect(screen.queryByText('Copied to clipboard.')).not.toBeInTheDocument();
  });
  it('serializes repeated clicks and permits an explicit retry after fulfillment', async () => {
    const pending = deferred(); const write = vi.fn().mockReturnValueOnce(pending.promise).mockResolvedValue(undefined); clipboard(write);
    render(<ReferralWidgetBuilderPage />); await openPreview();
    const button = screen.getByRole('button', { name: 'Copy Code' });
    fireEvent.click(button); fireEvent.click(button);
    expect(write).toHaveBeenCalledTimes(1);
    await act(async () => pending.resolve());
    fireEvent.click(button);
    await screen.findByText('Copied to clipboard.');
    expect(write).toHaveBeenCalledTimes(2);
  });
  it('does not show a previous owner completion on a new private preview', async () => {
    const pending = deferred(); const write = vi.fn(() => pending.promise); clipboard(write);
    const first = render(<ReferralWidgetBuilderPage />); await openPreview();
    fireEvent.click(screen.getByRole('button', { name: 'Copy Code' }));
    expect(write).toHaveBeenCalledTimes(1);
    first.unmount(); tenant = 'another-verified-tenant'; act(() => invalidateQueueOwner());
    render(<ReferralWidgetBuilderPage />);
    const next = await openPreview();
    await act(async () => pending.resolve());
    expect(next.value).toContain('tenant=another-verified-tenant');
    expect(screen.queryByText('Copied to clipboard.')).not.toBeInTheDocument();
    expect(write).toHaveBeenCalledTimes(1);
  });
});
