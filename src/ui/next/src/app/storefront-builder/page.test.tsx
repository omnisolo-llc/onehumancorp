import {notifyQueueIdentityChange} from '@/lib/sync/queueIdentity';
import {installBuilderLocks as installOnboardingLocks} from '../builder/testLocks';
import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import StorefrontBuilderPage from './page';
import { TooltipProvider } from '../../components/TooltipRegistry';
import { vi, describe, it, expect, beforeEach, afterEach } from 'vitest';

// Mock TooltipRegistry and help components
vi.mock('../../components/Walkthrough', () => ({
  WalkthroughTarget: ({ children, id }: React.PropsWithChildren<{ id: string }>) => <div id={id}>{children}</div>,
  InteractiveWalkthrough: () => null
}));
vi.mock('../../components/TooltipRegistry', () => ({
  TooltipProvider: ({ children }: { children?: import('react').ReactNode }) => <>{children}</>,
  WithTooltip: ({ children }: { children?: import('react').ReactNode }) => <div>{children}</div>
}));
vi.mock('../../components/help', () => ({
  useWalkthrough: () => ({ startWalkthrough: vi.fn() })
}));

vi.mock('@/lib/sync/queueIdentity',async importOriginal=>({...await importOriginal<typeof import('@/lib/sync/queueIdentity')>(),readQueueOwner:vi.fn(async()=>({userId:'builder-user',tenantId:'builder-tenant'}))}));

describe('StorefrontBuilderPage', () => {
  beforeEach(() => {
    global.fetch = vi.fn().mockResolvedValue({
        json: () => Promise.resolve({ data: {} })
    });
    localStorage.clear();notifyQueueIdentityChange();installOnboardingLocks();
  });

  afterEach(() => {
    vi.resetAllMocks();
  });

  it('renders initial setup state', async () => {
    render(<TooltipProvider><StorefrontBuilderPage /></TooltipProvider>);
    expect(await screen.findByText('Welcome to OmniSolo OneHumanCorp Smart Builder')).toBeTruthy();
    expect(screen.getByText('Build My Storefront')).toBeTruthy();
  });

  it('handles empty input generation by keeping button disabled', async () => {
    render(<TooltipProvider><StorefrontBuilderPage /></TooltipProvider>);
    const button = await screen.findByText('Build My Storefront');
    expect(button.className).toContain('cursor-not-allowed');
  });

  it('enables button with valid input and calls generate', async () => {
    render(<TooltipProvider><StorefrontBuilderPage /></TooltipProvider>);

    const textarea = await screen.findByPlaceholderText(/e.g. I run a mobile dog grooming service/i);
    fireEvent.change(textarea, { target: { value: 'Valid long business bio' } });

    const button = await screen.findByText('Build My Storefront');
    expect(button.className).not.toContain('cursor-not-allowed');

    vi.mocked(global.fetch, { partial: true }).mockResolvedValueOnce({
      ok: true, status: 200,
      json: async () => ({
        pages: [{
          blocks: [
            { block_type: 'HeroBlock', content: { headline: 'Test Hero' } }
          ]
        }]
      })
    });

    fireEvent.click(button);

    expect(screen.getByText('Agents are building your store...')).toBeTruthy();

    await waitFor(() => {
      expect(screen.getByText('Preview Mode')).toBeTruthy();
      expect(screen.getByText('⚡ Powered by OmniSolo')).toBeTruthy();
    });
  });

  it('handles publish workflow correctly', async () => {
    render(<TooltipProvider><StorefrontBuilderPage /></TooltipProvider>);

    // Setup state manually or go through flow
    const textarea = await screen.findByPlaceholderText(/e.g. I run a mobile dog grooming service/i);
    fireEvent.change(textarea, { target: { value: 'Valid long business bio' } });

    vi.mocked(global.fetch, { partial: true }).mockResolvedValueOnce({
      ok: true, status: 200,
      json: async () => ({
        pages: [{
          blocks: [
            { block_type: 'HeroBlock', content: { headline: 'Test Hero' } }
          ]
        }]
      })
    });

    fireEvent.click(screen.getByText('Build My Storefront'));

    await waitFor(() => {
      expect(screen.getByText('Save site draft')).toBeTruthy();
    });

    vi.mocked(global.fetch, { partial: true }).mockResolvedValueOnce(Response.json({ id:'33333333-3333-4333-8333-333333333333',domain: 'test' }));

    fireEvent.click(screen.getByText('Save site draft'));

    await waitFor(() => {
      expect(screen.getByText(/Site saved.*publishing has not been verified/)).toBeTruthy();
      expect(screen.queryByText("You're Live!")).toBeNull();
    });
  });

  it('handles chat with agent workflow', async () => {
    render(<TooltipProvider><StorefrontBuilderPage /></TooltipProvider>);

    const textarea = await screen.findByPlaceholderText(/e.g. I run a mobile dog grooming service/i);
    fireEvent.change(textarea, { target: { value: 'Valid long business bio' } });

    vi.mocked(global.fetch, { partial: true }).mockResolvedValueOnce({
      ok: true, status: 200,
      json: async () => ({
        pages: [{
          blocks: [
            { block_type: 'HeroBlock', content: { headline: 'Test Hero' } }
          ]
        }]
      })
    });

    fireEvent.click(screen.getByText('Build My Storefront'));

    await waitFor(() => {
      expect(screen.getByText('Agent')).toBeTruthy();
    });

    fireEvent.click(screen.getByText('Agent'));

    await act(async () => {
        await new Promise(resolve => setTimeout(resolve, 0));
    });
    expect(screen.queryAllByText(/Marketing Agent/i).length).toBeGreaterThan(0);
    const chatTextarea = screen.getByPlaceholderText(/e.g. Add a new product.../i);
    fireEvent.change(chatTextarea, { target: { value: "Add a new product" } });

    // It works! We just want to check if chat screen is open
  });
  it('keeps a failed block save visibly unsaved and preserves the previous owner-local copy',async()=>{
    render(<TooltipProvider><StorefrontBuilderPage/></TooltipProvider>);
    const input=await screen.findByPlaceholderText(/mobile dog grooming service/i);fireEvent.change(input,{target:{value:'Owner layout description'}});
    vi.mocked(fetch).mockResolvedValueOnce(Response.json({pages:[{blocks:[{block_type:'HeroBlock',content:{headline:'Editable headline'}}]}]}));
    fireEvent.click(screen.getByText('Build My Storefront'));fireEvent.click(await screen.findByText('Editable headline'));
    const edit=await screen.findByDisplayValue('Editable headline');fireEvent.change(edit,{target:{value:'Newer unsaved headline'}});
    const key='omnisolo_onboarding_owned_v1:'+encodeURIComponent(JSON.stringify(['builder-user','builder-tenant']))+':storefront-builder-draft';const saved=localStorage.getItem(key);const original=localStorage.setItem.bind(localStorage);
    const fail=vi.spyOn(localStorage,'setItem').mockImplementation((name,value)=>{if(name===key)throw new Error('quota');original(name,value);});
    try{
      fireEvent.click(screen.getByText('Save Changes'));expect(localStorage.getItem(key)).toBe(saved);
      expect(await screen.findByText(/could not save your latest builder edits/i)).toBeVisible();expect(screen.queryByText('Changes saved on this device.')).toBeNull();
    }finally{fail.mockRestore();}
  });

});

async function startStorefront() {
  render(<TooltipProvider><StorefrontBuilderPage /></TooltipProvider>);
  fireEvent.change(await screen.findByPlaceholderText(/mobile dog grooming service/i), { target: { value: 'Owner supplied cake business' } });
  fireEvent.click(screen.getByRole('button', { name: 'Build My Storefront' }));
}

it('shows the provider prerequisite after an unavailable generation without losing the owner description', async () => {
  localStorage.clear(); notifyQueueIdentityChange(); installOnboardingLocks();
  vi.stubGlobal('fetch', vi.fn(async () => Response.json({ code: 'generation_unavailable', error: 'Configure the builder operator tenant and an authorized text-generation provider before generating a draft' }, { status: 503 })));
  await startStorefront();
  expect(await screen.findByRole('alert')).toHaveTextContent(/authorized text-generation provider/i);
  expect(screen.getByPlaceholderText(/mobile dog grooming service/i)).toHaveValue('Owner supplied cake business');
  expect(screen.queryByText('Preview Mode')).toBeNull();
  expect(screen.getByRole('button', { name: 'Build My Storefront' })).toBeEnabled();
});

it.each([
  { success: false, pages: [{ blocks: [{ block_type: 'HeroBlock', content: { headline: 'Unconfirmed' } }] }] },
  { error: 'rejected', pages: [{ blocks: [{ block_type: 'HeroBlock', content: { headline: 'Unconfirmed' } }] }] },
  { pages: [{ blocks: [] }] },
  { pages: [{ blocks: [{ block_type: 'HeroBlock', content: { headline: { nested: 'Unconfirmed' } } }] }] },
  { pages: [{ blocks: [{ block_type: 'HeroBlock', content: { headline: [{ name: 'Unconfirmed' }] } }] }] },
  { pages: [{ blocks: [{ block_type: 'HeroBlock', content: { headline: 'Title', subtitle: [{ name: 'Unconfirmed' }] } }] }] },
  { pages: [{ blocks: [{ block_type: 'TextBlock', content: { text: [{ name: 'Unconfirmed' }] } }] }] },
  { pages: [{ blocks: [{ block_type: 'ProductGridBlock', content: {} }] }] },
])('rejects an unconfirmed generated layout before replacing the local draft %#', async payload => {
  localStorage.clear(); notifyQueueIdentityChange(); installOnboardingLocks();
  vi.stubGlobal('fetch', vi.fn(async () => Response.json(payload)));
  await startStorefront();
  expect(await screen.findByRole('alert')).toHaveTextContent(/draft.*could not be confirmed|unsupported content/i);
  expect(screen.queryByText('Preview Mode')).toBeNull();
  expect(Object.values(localStorage).join(' ')).not.toContain('Unconfirmed');
});

it('renders the generated Hero subtitle and TextBlock instead of dropping actual provider content', async () => {
  localStorage.clear(); notifyQueueIdentityChange(); installOnboardingLocks();
  vi.stubGlobal('fetch', vi.fn(async () => Response.json({ pages: [{ blocks: [
    { block_type: 'HeroBlock', content: { headline: 'Owner cake business', subtitle: 'Custom bakes from the supplied brief' } },
    { block_type: 'TextBlock', content: { text: 'Orders must be discussed with the owner' } },
  ] }] })));
  await startStorefront();
  expect(await screen.findByText('Preview Mode')).toBeVisible();
  expect(screen.getByText('Custom bakes from the supplied brief')).toBeVisible();
  expect(screen.getByText('Orders must be discussed with the owner')).toBeVisible();
});

it('keeps the existing draft and edit request when the generation provider is unavailable', async () => {
  localStorage.clear(); notifyQueueIdentityChange(); installOnboardingLocks();
  const request = vi.fn(async () => Response.json({ pages: [{ blocks: [{ block_type: 'HeroBlock', content: { headline: 'Existing reviewed headline' } }] }] }));
  vi.stubGlobal('fetch', request);
  await startStorefront();
  fireEvent.click(await screen.findByText('Agent'));
  fireEvent.change(screen.getByPlaceholderText(/Add a new product/i), { target: { value: 'Change the headline to vegan cakes' } });
  request.mockImplementation(async () => Response.json({ code: 'generation_outcome_unknown' }, { status: 502 }));
  fireEvent.keyDown(screen.getByPlaceholderText(/Add a new product/i), { key: 'Enter' });
  expect(await screen.findByRole('alert')).toHaveTextContent(/No automatic retry/i);
  expect(screen.getByPlaceholderText(/Add a new product/i)).toHaveValue('Change the headline to vegan cakes');
  fireEvent.click(screen.getByRole('button', { name: 'Close Marketing Agent' }));
  expect(await screen.findByText('Existing reviewed headline')).toBeVisible();
});
