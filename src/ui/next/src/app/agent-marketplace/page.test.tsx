import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { act, cleanup, render, screen, fireEvent, waitFor } from '@testing-library/react';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import { marketplaceBackend } from './marketplace.test-support';
import Page from './page';
let backend: ReturnType<typeof marketplaceBackend>;
beforeEach(() => { backend = marketplaceBackend(); });
afterEach(() => { cleanup(); notifyQueueIdentityChange(); vi.unstubAllGlobals(); });
it('renders the marketplace header and search', async () => {
 render(<Page/>); expect(screen.getByRole('heading',{name:'Agent Marketplace'})).toBeVisible();
 expect(screen.getByPlaceholderText('Search for agents...')).toBeVisible(); await screen.findByText('Senior Rust Developer');
});
it('fetches actual definitions and displays their source and content', async () => {
 render(<Page/>); await screen.findByText('Senior Rust Developer');
 expect(screen.getAllByText('Drafts for owner review')).toHaveLength(2); expect(screen.getAllByText(/First-party definition/)).toHaveLength(2);
 expect(backend.fetch.mock.calls.some(([url]) => url.startsWith('/api/v1/agents/definitions?'))).toBe(true);
});
it('displays no results only after a verified empty response', async () => {
 backend.state.definitions=[]; render(<Page/>); await screen.findByText('No agents found.'); expect(screen.queryByRole('alert')).toBeNull();
});
it('updates the actual search query when typing', async () => {
 render(<Page/>); await screen.findByText('Technical Writer'); fireEvent.change(screen.getByPlaceholderText('Search for agents...'),{target:{value:'Rust'}});
 await waitFor(() => expect(backend.fetch.mock.calls.some(([url,init]) => url==='/api/v1/agents/definitions?q=Rust' && init?.signal instanceof AbortSignal)).toBe(true));
 await screen.findByText('Senior Rust Developer'); expect(screen.queryByText('Technical Writer')).toBeNull();
});
it('shows service failure without replacing it with empty results', async () => {
 backend.fetch.mockImplementation(async(url,init)=>url.includes('/definitions')?new Response('',{status:503}):backend.route(url,init));
 render(<Page/>); await screen.findByRole('button',{name:'Retry marketplace'}); expect(screen.getByRole('alert')).toHaveTextContent('Failed to fetch agents'); expect(screen.queryByText('No agents found.')).toBeNull();
});
it('retries the current query after a service failure', async () => {
 let fail=true; backend.fetch.mockImplementation(async(url,init)=>url.includes('/definitions')&&fail?new Response('',{status:503}):backend.route(url,init));
 render(<Page/>); await screen.findByRole('button',{name:'Retry marketplace'}); fireEvent.change(screen.getByPlaceholderText('Search for agents...'),{target:{value:'Writer'}});
 await waitFor(()=>expect(backend.fetch.mock.calls.some(([url])=>url.endsWith('q=Writer'))).toBe(true)); fail=false;
 await act(async()=>fireEvent.click(screen.getByRole('button',{name:'Retry marketplace'}))); await screen.findByText('Technical Writer'); expect(screen.queryByText('Senior Rust Developer')).toBeNull();
});
