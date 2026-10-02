import {notifyQueueIdentityChange} from '@/lib/sync/queueIdentity';
import {installBuilderLocks as installOnboardingLocks} from '../builder/testLocks';
import {initializeWebsiteDraft} from './store';
import {useOnboardingStore} from '../onboarding/store';
import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import WebsiteBuilderPage from './page';
import { vi, describe, it, expect, beforeEach, afterEach } from 'vitest';
import userEvent from '@testing-library/user-event';

const push=vi.hoisted(()=>vi.fn());
vi.mock('next/navigation', () => ({ useRouter: () => ({ push, replace: vi.fn(), refresh: vi.fn() }) }));
vi.mock('@/lib/sync/queueIdentity', async importOriginal => ({...await importOriginal<typeof import('@/lib/sync/queueIdentity')>(),readQueueOwner:vi.fn(async()=>({userId:'builder-user',tenantId:'builder-tenant'}))}));


// Mock TooltipRegistry and help components
vi.mock('../../components/TooltipRegistry', () => ({
  WithTooltip: ({ children }: import('react').PropsWithChildren) => <div data-testid="tooltip">{children}</div>
}));
vi.mock('../../components/help', () => ({
  useWalkthrough: () => ({ startWalkthrough: vi.fn() })
}));
vi.mock('../builder/components', () => ({
  SmartBlock: ({ type, props }: import('react').ComponentProps<typeof import('../builder/components').SmartBlock>) => <div data-testid={`smartblock-${type}`}>{JSON.stringify(props)}</div>,
  DraggableBlock: ({ children, onDragStart, onDragOver, onDragEnter, onDragEnd, onMoveUp, onMoveDown, onClick, isSelected }: import('react').ComponentProps<typeof import('../builder/components').DraggableBlock>) => (
    <div
      data-testid="draggable-block"
      onClick={onClick}
      onDragStart={onDragStart}
      onDragOver={onDragOver}
      onDragEnter={onDragEnter}
      onDragEnd={onDragEnd}
      data-selected={isSelected}
    >
      {children}
      {onMoveUp && <button onClick={onMoveUp}>Up</button>}
      {onMoveDown && <button onClick={onMoveDown}>Down</button>}
    </div>
  )
}));


import { useWebsiteBuilderStore } from './store';

describe('WebsiteBuilderPage', () => {
  beforeEach(async () => {
    global.fetch = vi.fn().mockImplementation(() => Promise.resolve(Response.json({}, { status: 200 })));
    localStorage.clear(); notifyQueueIdentityChange(); installOnboardingLocks(); push.mockClear(); await initializeWebsiteDraft();
    vi.useFakeTimers({ shouldAdvanceTime: true });
    // Reset zustand store state
    useWebsiteBuilderStore.setState({
      wizardStep: 0,
      businessName: '',
      businessType: '',
      hasPhysicalProducts: false,
      hasDigitalProducts: false,
      productName: '',
      productPrice: '',
      paymentMethod: '',
      template: '',
      bio: '',
      domainChoice: 'subdomain',
      aiAgents: [],
      aiAutoRespond: false,
      blocks: [],
      status: "idle",
      liveUrl: ""
    });
  });


  afterEach(() => {
    vi.resetAllMocks();
    vi.useRealTimers();
  });

  it('renders initial setup screen', async () => {
    await act(async () => { render(<WebsiteBuilderPage />); });
    expect(screen.getByText('Your business, live in minutes.')).toBeInTheDocument();

    // Check local storage init fetching
    expect(global.fetch).toHaveBeenCalledWith('/api/v1/onboarding/state',expect.objectContaining({headers:expect.any(Headers)}));
  });

  it('can follow the standard wizard flow', async () => {
    vi.useRealTimers();
    global.fetch = vi.fn().mockImplementation((url: string) => {
      if (url === '/api/v1/onboarding/start') {
        return Promise.resolve(Response.json({ organization_id: 'test-org-id' }, { status: 200 }));
      }
      if (url === '/api/v1/onboarding/state') {
          return Promise.resolve(Response.json({}, { status: 200 }))
      }
      return Promise.resolve(Response.json({}, { status: 200 }));
    });

    userEvent.setup({ delay: null });
    await act(async () => { render(<WebsiteBuilderPage />); });

    // Step 0
    fireEvent.click(screen.getByText('Start My Business'));

    // Step 1
    fireEvent.click(screen.getByText('Online Store'));

    // Step 2
    fireEvent.change(screen.getByPlaceholderText('What is your business called?'), { target: { value: 'My Shop' } });
    fireEvent.click(screen.getByText('Next'));

    // Step 3
    fireEvent.click(screen.getByLabelText('Physical Products'));
    fireEvent.click(screen.getByText('Next'));

    // Step 4
    fireEvent.change(screen.getByPlaceholderText('What is the name of this product?'), { target: { value: 'T-Shirt' } });
    fireEvent.change(screen.getByPlaceholderText('0.00'), { target: { value: '25.00' } });
    fireEvent.click(screen.getByText('Next'));

    // Step 5
    fireEvent.click(screen.getByText('Online'));

    // Step 6
    expect(screen.queryByPlaceholderText('Password')).not.toBeInTheDocument();
    fireEvent.click(screen.getByText('Next'));

    // Step 7
    fireEvent.click(screen.getByText('Modern'));

    // Step 7.5
    fireEvent.click(screen.getByText('Next'));

    // Step 8
    fireEvent.click(screen.getByText('Free OmniSolo Domain'));

    // Step 8.5
    fireEvent.click(screen.getByText('Next'));

    // Step 9
    fireEvent.click(screen.getByText('Review workspace setup'));

    await waitFor(() => expect(push).toHaveBeenCalledWith('/onboarding'));
    expect(vi.mocked(fetch).mock.calls.some(([url])=>url==='/api/v1/onboarding/start')).toBe(false);
    const transferred=useOnboardingStore.getState();
    expect(transferred).toEqual(expect.objectContaining({businessName:'My Shop',businessDescription:'',categories:['physical'],websiteTemplate:'Modern',domainChoice:'subdomain',aiAutoRespond:false,firstProductName:'T-Shirt',firstProductPrice:'25.00',step:3}));
    expect(useWebsiteBuilderStore.getState().paymentMethod).toBe('Online');
    expect(transferred).not.toHaveProperty('admin_password');expect(transferred).not.toHaveProperty('admin_email');
  });

  it('can follow the instant-build flow into canonical review with its description preserved', async () => {
    vi.useRealTimers();render(<WebsiteBuilderPage/>);
    fireEvent.click(await screen.findByText('Instant Build'));
    fireEvent.change(screen.getByPlaceholderText('e.g. I run a local bakery'),{target:{value:'I run a local bakery'}});
    fireEvent.click(screen.getByText('Review setup options'));
    await waitFor(()=>expect(push).toHaveBeenCalledWith('/onboarding'));
    expect(useOnboardingStore.getState()).toEqual(expect.objectContaining({bio:'I run a local bakery',step:-1}));
    expect(vi.mocked(fetch).mock.calls.some(([url])=>url==='/api/v1/onboarding/intake'||url==='/api/v1/onboarding/start')).toBe(false);
    expect(screen.queryByText('Success! Your business is live!')).toBeNull();
  });

  it('never reports a rejected publication as live', async () => {
    vi.useRealTimers();useWebsiteBuilderStore.setState({businessName:'Rejected Shop',blocks:[{type:'Hero',props:{headline:'Local draft'}}],status:'draft'});
    vi.mocked(fetch).mockImplementation(async url=>String(url).endsWith('/publish_draft')?Response.json({error:'rejected'},{status:500}):Response.json({}));
    render(<WebsiteBuilderPage/>);fireEvent.click(await screen.findByText('Save site draft'));
    expect(await screen.findByText(/site save was not acknowledged/i)).toBeVisible();
    expect(screen.queryByText('Success! Your business is live!')).toBeNull();expect(screen.queryByText(/Site saved \(/)).toBeNull();
  });

  it('loads blocks from local storage and handles drag/drop/reorder', async () => {
    const initialBlocks = [
      { type: 'Hero', props: { title: '1' } },
      { type: 'Catalog', props: { title: '2' } },
      { type: 'Booking', props: { title: '3' } }
    ];
    useWebsiteBuilderStore.setState({ blocks: initialBlocks, status: 'draft' });

    render(<WebsiteBuilderPage />);

    await waitFor(() => {
      // 3 + 1 PoweredBy (the powered by component isn't wrapped in draggable-block anymore based on actual implementation)
      // Wait for it to not be empty
      expect(screen.getAllByTestId('draggable-block').length).toBe(3);
      expect(screen.getByText('⚡ Powered by OmniSolo')).toBeInTheDocument();
    });

    const blocks = screen.getAllByTestId('draggable-block');

    // Test Move Down
    const downBtn = blocks[0].querySelector('button');
    expect(downBtn?.textContent).toBe('Down');
    fireEvent.click(downBtn!);

    // Test selection (simulated by click, but our mock doesn't truly pass down state changes the same way, we just want to ensure it doesn't crash)
    fireEvent.click(blocks[1]);
    fireEvent.click(blocks[1]); // deselect

    // Drag and drop is hard to fully simulate, but we can trigger the events
    const dataTransfer = {
      setData: vi.fn(),
      effectAllowed: '',
      dropEffect: ''
    };

    fireEvent.dragStart(blocks[0], { dataTransfer });
    fireEvent.dragEnter(blocks[1]);
    fireEvent.dragOver(blocks[1], { dataTransfer });
    fireEvent.dragEnd(blocks[0]);
  });

  it('handles launch from draft mode', async () => {
    useWebsiteBuilderStore.setState({ status: 'draft', blocks: [{ type: 'Hero', props: {} }] });

    (vi.mocked(global.fetch)).mockImplementation((url: string) => {
      if (url.includes('publish_draft')) {
        return Promise.resolve(Response.json({ id:'33333333-3333-4333-8333-333333333333',domain: 'testdomain' }, { status: 200 }));
      }
      return Promise.resolve(Response.json({}, { status: 200 }));
    });

    render(<WebsiteBuilderPage />);

    await waitFor(() => {
      expect(screen.getByText('Save site draft')).toBeInTheDocument();
    });

    fireEvent.click(screen.getByText('Save site draft'));

    await waitFor(() => {
      expect(screen.getByText(/Site saved.*33333333/)).toBeInTheDocument();
      expect(screen.getByText(/publishing has not been verified/)).toBeInTheDocument();
      expect(screen.queryByText('/bio/testdomain')).toBeNull();
    });
  });

  it('loads accepted setup fields from real server state while layouts remain local',async()=>{
    vi.mocked(fetch).mockImplementation(async url=>String(url).endsWith('/state')?Response.json({wizardState:{businessName:'Server Shop',bio:'Test bio',firstProductName:'Service',firstProductPrice:'20.00'}}):Response.json({}));
    render(<WebsiteBuilderPage/>);await screen.findByText('Your business, live in minutes.');
    act(()=>useWebsiteBuilderStore.setState({wizardStep:2}));
    expect(screen.getByDisplayValue('Server Shop')).toBeVisible();expect(screen.getByDisplayValue('Test bio')).toBeVisible();
    expect(useWebsiteBuilderStore.getState()).toEqual(expect.objectContaining({productName:'Service',productPrice:'20.00',blocks:[]}));
  });

  it('syncs accepted setup fields through the owner-bound gate after an edit',async()=>{
    vi.useRealTimers();vi.mocked(fetch).mockImplementation(async(_url,options)=>options?.method==='POST'?new Response(null,{status:204}):Response.json({}));
    render(<WebsiteBuilderPage/>);await screen.findByText('Your business, live in minutes.');act(()=>useWebsiteBuilderStore.setState({wizardStep:2}));
    fireEvent.change(screen.getByPlaceholderText('What is your business called?'),{target:{value:'Bakery From Edit'}});
    await waitFor(()=>expect(vi.mocked(fetch).mock.calls.some(([url,options])=>url==='/api/v1/onboarding/state'&&options?.method==='POST')).toBe(true),{timeout:2000});
    const call=vi.mocked(fetch).mock.calls.find(([url,options])=>url==='/api/v1/onboarding/state'&&options?.method==='POST')!;
    expect(JSON.parse(String(call[1]?.body))).toEqual(expect.objectContaining({wizardState:expect.objectContaining({businessName:'Bakery From Edit'})}));
    expect(new Headers(call[1]?.headers).get('x-ohc-expected-user')).toBe('builder-user');expect(new Headers(call[1]?.headers).get('x-ohc-expected-tenant')).toBe('builder-tenant');
    expect(await screen.findByText(/Setup details saved/)).toBeVisible();
  });
});
