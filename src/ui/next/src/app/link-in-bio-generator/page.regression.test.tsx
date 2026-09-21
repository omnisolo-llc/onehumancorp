import { act, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import LinkInBioGeneratorPage from './page';

vi.mock('../components/PoweredByOmniSolo', () => ({ PoweredByOmniSolo: () => null }));

const savedConfig = {
  store_name: 'Test Store', bio: 'Our store', theme: 'light',
  links: [{ id: 'existing-link', title: 'Visit our store', url: 'https://store.example.test/shop' }],
};

beforeEach(() => {
  vi.clearAllMocks();
  localStorage.clear();
  global.fetch = vi.fn(async (_url, init) => init?.method === 'POST'
    ? new Response(null, { status: 200 })
    : Response.json(savedConfig));
  Object.defineProperty(navigator, 'clipboard', {
    configurable: true,
    value: { writeText: vi.fn().mockResolvedValue(undefined) },
  });
});

async function openEditor() {
  render(<LinkInBioGeneratorPage />);
  await screen.findByDisplayValue('Test Store');
}

function publishedBodies() {
  return vi.mocked(fetch).mock.calls.filter(([, init]) => init?.method === 'POST')
    .map(([, init]) => JSON.parse(String(init!.body)));
}

describe('real link-in-bio editor contracts', () => {
  it('publishes unique stable IDs for new links while retaining persisted IDs', async () => {
    await openEditor();
    fireEvent.click(screen.getByRole('button', { name: '+ Add Link' }));
    fireEvent.change(screen.getByRole('textbox', { name: 'Link 2 URL' }), {
      target: { value: 'https://store.example.test/booking' },
    });
    fireEvent.click(screen.getByRole('button', { name: 'Save & Publish' }));
    await screen.findByRole('button', { name: 'Saved! ✅' });
    const [first] = publishedBodies();
    expect(first.links[0].id).toBe('existing-link');
    expect(first.links[1].id).toEqual(expect.any(String));
    expect(first.links[1].id.length).toBeGreaterThan(0);
    expect(new Set(first.links.map((link: { id: string }) => link.id)).size).toBe(2);

    fireEvent.change(screen.getByRole('textbox', { name: 'Link 2 Title' }), {
      target: { value: 'Book a visit' },
    });
    fireEvent.click(screen.getByRole('button', { name: 'Save & Publish' }));
    await waitFor(() => expect(publishedBodies()).toHaveLength(2));
    expect(publishedBodies()[1].links[1].id).toBe(first.links[1].id);
  });

  it.each(['', 'https://', '#', '//evil.example', 'javascript:alert(1)',
    'data:text/html,test', 'https://user:password@store.example.test'])('does not turn an invalid destination %j into navigation or publish it', async (url) => {
    await openEditor();
    fireEvent.change(screen.getByRole('textbox', { name: 'Link 1 URL' }), { target: { value: url } });
    expect(screen.queryByRole('link', { name: 'Visit our store' })).not.toBeInTheDocument();
    expect(screen.getByText('Visit our store')).toBeVisible();

    fireEvent.click(screen.getByRole('button', { name: 'Save & Publish' }));
    expect(await screen.findByRole('alert')).toHaveTextContent(/link 1.*valid/i);
    expect(publishedBodies()).toHaveLength(0);
    expect(screen.getByRole('textbox', { name: 'Link 1 URL' })).toHaveValue(url);
  });

  it.each(['https://store.example.test/shop?q=a%26b', '/booking?service=consultation'])('keeps a valid destination %s working in the preview and published payload', async (url) => {
    await openEditor();
    fireEvent.change(screen.getByRole('textbox', { name: 'Link 1 URL' }), { target: { value: url } });
    expect(screen.getByRole('link', { name: 'Visit our store' })).toHaveAttribute('href', url);
    fireEvent.click(screen.getByRole('button', { name: 'Save & Publish' }));
    await screen.findByRole('button', { name: 'Saved! ✅' });
    expect(publishedBodies()[0].links[0].url).toBe(url);
  });

  it('shows a publish error without discarding edits or claiming success', async () => {
    await openEditor();
    fireEvent.change(screen.getByRole('textbox', { name: 'Bio / Description' }), {
      target: { value: 'Keep this unsaved text' },
    });
    vi.mocked(fetch).mockResolvedValueOnce(Response.json({ error: 'Service unavailable' }, { status: 503 }));
    fireEvent.click(screen.getByRole('button', { name: 'Save & Publish' }));
    expect(await screen.findByRole('alert')).toHaveTextContent(/publish/i);
    expect(screen.getByRole('textbox', { name: 'Bio / Description' })).toHaveValue('Keep this unsaved text');
    expect(screen.queryByRole('button', { name: 'Saved! ✅' })).not.toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Save & Publish' })).toBeEnabled();
  });

  it('does not report clipboard success before the write completes', async () => {
    await openEditor();
    let finish!: () => void;
    vi.mocked(navigator.clipboard.writeText).mockReturnValueOnce(new Promise<void>(resolve => { finish = resolve; }));
    fireEvent.click(screen.getByRole('button', { name: 'Copy Link' }));
    expect(screen.queryByRole('button', { name: 'Copied URL!' })).not.toBeInTheDocument();
    await act(async () => { finish(); });
    expect(await screen.findByRole('button', { name: 'Copied URL!' })).toBeVisible();
  });

  it('reports an unavailable clipboard instead of falsely claiming a copy', async () => {
    await openEditor();
    Object.defineProperty(navigator, 'clipboard', { configurable: true, value: undefined });
    fireEvent.click(screen.getByRole('button', { name: 'Copy Link' }));
    expect(await screen.findByRole('alert')).toHaveTextContent(/copy/i);
    expect(screen.queryByRole('button', { name: 'Copied URL!' })).not.toBeInTheDocument();
  });

  it('encodes the tenant as one segment when loading and copying a public URL', async () => {
    localStorage.setItem('business_display_name', 'store / & team');
    await openEditor();
    expect(fetch).toHaveBeenCalledWith('/api/v1/growth/link-in-bio/store%20%2F%20%26%20team');
    fireEvent.click(screen.getByRole('button', { name: 'Copy Link' }));
    await waitFor(() => expect(navigator.clipboard.writeText).toHaveBeenCalledWith(
      'https://cloud.omnisolo.co/bio/store%20%2F%20%26%20team',
    ));
  });
});
