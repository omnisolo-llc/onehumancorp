
import React from 'react';
import { act, cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import ApiDocsPage from './page';
import { TooltipProvider } from '../../components/TooltipRegistry';

describe('ApiDocsPage', () => {
  beforeEach(() => {
    global.fetch = vi.fn(async () => Response.json({ paths: { '/api/v1/help': {}, '/api/v1/tooltips': {} } }, { status: 200 }));
  });

  afterEach(() => {
    cleanup();
    vi.restoreAllMocks();
  });

  it('renders the warning and sends the authenticated spec only to its own titled frame', async () => {
    render(<TooltipProvider><ApiDocsPage /></TooltipProvider>);
    expect(screen.getByText('Advanced:')).toBeInTheDocument();
    const frame = await screen.findByTitle('Interactive API documentation') as HTMLIFrameElement;
    expect(frame).toHaveAttribute('src', '/api-docs/viewer');
    const send = vi.spyOn(frame.contentWindow!, 'postMessage');
    await act(async () => { fireEvent.load(frame); });
    expect(send).toHaveBeenCalledWith({ type: 'ohc-api-docs:init', spec: { paths: { '/api/v1/help': {}, '/api/v1/tooltips': {} } }, dark: false }, window.location.origin);
    expect(frame).toHaveAttribute('aria-busy', 'true');
    act(() => window.dispatchEvent(new MessageEvent('message', { origin: window.location.origin, source: frame.contentWindow, data: { type: 'ohc-api-docs:ready' } })));
    expect(frame).toHaveAttribute('aria-busy', 'false');
  });

  it('aborts a pending spec request when navigation unmounts the page', async () => {
    let finish!: (response: Response) => void;
    const fetcher = vi.fn<typeof fetch>((url) => url === '/api/v1/api-docs-spec' ? new Promise<Response>(resolve => { finish = resolve; }) : Promise.resolve(Response.json({})));
    global.fetch = fetcher;
    const page = render(<TooltipProvider><ApiDocsPage /></TooltipProvider>);
    await waitFor(() => expect(fetcher.mock.calls.filter(call => call[0] === '/api/v1/api-docs-spec')).toHaveLength(1));
    const signal = fetcher.mock.calls.find(call => call[0] === '/api/v1/api-docs-spec')?.[1]?.signal;
    expect(signal).toBeInstanceOf(AbortSignal);
    page.unmount();
    expect(signal?.aborted).toBe(true);
    await act(async () => finish(Response.json({ paths: {} })));
    expect(screen.queryByTitle('Interactive API documentation')).toBeNull();
  });

  it('rejects foreign and retired frame messages after navigation and remount', async () => {
    const first = render(<TooltipProvider><ApiDocsPage /></TooltipProvider>);
    const oldFrame = await screen.findByTitle('Interactive API documentation') as HTMLIFrameElement;
    const retiredWindow = oldFrame.contentWindow;
    first.unmount();
    render(<TooltipProvider><ApiDocsPage /></TooltipProvider>);
    const frame = await screen.findByTitle('Interactive API documentation') as HTMLIFrameElement;
    for (const message of [
      { source: retiredWindow, origin: window.location.origin },
      { source: frame.contentWindow, origin: 'https://foreign.invalid' },
      { source: window, origin: window.location.origin },
    ]) act(() => window.dispatchEvent(new MessageEvent('message', { ...message, data: { type: 'ohc-api-docs:resize', height: 1200 } })));
    expect(frame.style.height).toBe('600px');
    act(() => window.dispatchEvent(new MessageEvent('message', { source: frame.contentWindow, origin: window.location.origin, data: { type: 'ohc-api-docs:resize', height: 1200 } })));
    expect(frame.style.height).toBe('1200px');
  });

  it('displays an error message when fetch fails', async () => {
    global.fetch = vi.fn().mockResolvedValue(Response.json({}, { status: 500 }));

    render(
      <TooltipProvider>
        <ApiDocsPage />
      </TooltipProvider>
    );

    await waitFor(() => {
      expect(screen.getByText('Failed to load API documentation')).toBeInTheDocument();
      expect(screen.getByText('Failed to load API Documentation.')).toBeInTheDocument();
    });
  });

  it('displays an error message when fetch throws an exception', async () => {
    global.fetch = vi.fn().mockRejectedValue(new Error('Network error'));

    render(
      <TooltipProvider>
        <ApiDocsPage />
      </TooltipProvider>
    );

    await waitFor(() => {
      expect(screen.getByText('Failed to load API documentation')).toBeInTheDocument();
      expect(screen.getByText('Failed to load API Documentation.')).toBeInTheDocument();
    });
  });
});
