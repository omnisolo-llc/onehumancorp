import React from 'react';
import { act, cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import ReferralWidgetBuilderPage from './page';

const clipboardDescriptor = Object.getOwnPropertyDescriptor(navigator, 'clipboard');
function clipboard(writeText?: (value: string) => Promise<void>) {
  Object.defineProperty(navigator, 'clipboard', { configurable: true, value: writeText ? { writeText } : undefined });
}
function deferred() {
  let resolve!: () => void;
  let reject!: (error: Error) => void;
  const promise = new Promise<void>((yes, no) => { resolve = yes; reject = no; });
  return { promise, resolve, reject };
}
beforeEach(() => { vi.stubGlobal('fetch', vi.fn(async url => Response.json(url === '/api/v1/auth/session-identity' ? { userId: 'referral-owner', tenantId: 'referral-tenant', expiresAt: Date.now() + 60000 } : { current_plan: 'Free' }))); localStorage.clear(); localStorage.setItem('business_display_name', 'reviewed-store'); });
afterEach(() => {
  cleanup(); vi.unstubAllGlobals();
  if (clipboardDescriptor) Object.defineProperty(navigator, 'clipboard', clipboardDescriptor);
  else Reflect.deleteProperty(navigator, 'clipboard');
});

describe('Referral widget copy action', () => {
  it('copies exactly the displayed link only after a click and acknowledges fulfillment', async () => {
    const pending=deferred(); const write=vi.fn(() => pending.promise); clipboard(write);
    render(<ReferralWidgetBuilderPage />);
    const displayed=(screen.getByRole('textbox',{name:'Referral link preview'}) as HTMLInputElement).value;
    expect(write).not.toHaveBeenCalled();
    fireEvent.click(screen.getByRole('button',{name:'Copy Link'}));
    expect(write).toHaveBeenCalledExactlyOnceWith(displayed);
    expect(screen.queryByText('Link copied')).not.toBeInTheDocument();
    expect(screen.getByRole('button',{name:'Copying…'})).toBeDisabled();
    await act(async()=>pending.resolve());
    expect(screen.getByRole('status')).toHaveTextContent('Link copied');
  });
  it('shows truthful failure after a rejected clipboard write', async () => {
    clipboard(vi.fn().mockRejectedValue(new Error('permission denied')));
    render(<ReferralWidgetBuilderPage />);
    fireEvent.click(screen.getByRole('button',{name:'Copy Link'}));
    expect(await screen.findByRole('alert')).toHaveTextContent('Could not copy the link');
    expect(screen.queryByText('Link copied')).not.toBeInTheDocument();
    expect(screen.getByRole('button',{name:'Copy Link'})).toBeEnabled();
  });
  it('offers manual copying when the Clipboard API is unavailable', async () => {
    clipboard(); render(<ReferralWidgetBuilderPage />);
    fireEvent.click(screen.getByRole('button',{name:'Copy Link'}));
    expect(await screen.findByRole('alert')).toHaveTextContent('Select the link and copy it manually');
    expect(screen.queryByText('Link copied')).not.toBeInTheDocument();
  });
  it('serializes repeated clicks and allows an explicit retry after fulfillment', async () => {
    const pending=deferred(); const write=vi.fn().mockReturnValueOnce(pending.promise).mockResolvedValue(undefined); clipboard(write);
    render(<ReferralWidgetBuilderPage />);
    const button=screen.getByRole('button',{name:'Copy Link'});
    fireEvent.click(button); fireEvent.click(button);
    expect(write).toHaveBeenCalledTimes(1);
    await act(async()=>pending.resolve());
    fireEvent.click(screen.getByRole('button',{name:'Copy Link'}));
    await screen.findByText('Link copied');
    expect(write).toHaveBeenCalledTimes(2);
  });
  it('does not report an older page completion as success for a newly displayed link', async () => {
    const pending=deferred(); const write=vi.fn(()=>pending.promise); clipboard(write);
    const first=render(<ReferralWidgetBuilderPage />);
    fireEvent.click(screen.getByRole('button',{name:'Copy Link'}));
    expect(write).toHaveBeenCalledTimes(1);
    first.unmount(); localStorage.setItem('business_display_name','another-store');
    render(<ReferralWidgetBuilderPage />);
    await act(async()=>pending.resolve());
    expect(screen.getByRole('textbox',{name:'Referral link preview'})).toHaveValue('https://cloud.omnisolo.co/setup.html?ref=another-store&promo=ref123');
    expect(screen.queryByText('Link copied')).not.toBeInTheDocument();
    expect(write).toHaveBeenCalledTimes(1);
  });
});
