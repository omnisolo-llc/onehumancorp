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
      ok: true,
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
      ok: true,
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
      ok: true,
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
