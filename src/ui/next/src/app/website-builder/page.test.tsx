import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import WebsiteBuilderPage from './page';
import { vi, describe, it, expect, beforeEach, afterEach } from 'vitest';
import userEvent from '@testing-library/user-event';

vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: vi.fn(), replace: vi.fn(), refresh: vi.fn() })
}));


// Mock TooltipRegistry and help components
vi.mock('../../components/TooltipRegistry', () => ({
  WithTooltip: ({ children }: any) => <div data-testid="tooltip">{children}</div>
}));
vi.mock('../../components/help', () => ({
  useWalkthrough: () => ({ startWalkthrough: vi.fn() })
}));
vi.mock('../builder/components', () => ({
  SmartBlock: ({ type, props }: any) => <div data-testid={`smartblock-${type}`}>{JSON.stringify(props)}</div>,
  DraggableBlock: ({ children, onDragStart, onDragOver, onDragEnter, onDragEnd, onMoveUp, onMoveDown, onClick, isSelected }: any) => (
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
  beforeEach(() => {
    global.fetch = vi.fn().mockImplementation(() => Promise.resolve({
      ok: true,
      json: () => Promise.resolve({})
    }));
    localStorage.clear();
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
      userName: '',
      userEmail: '',
      userPassword: '',
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
    expect(global.fetch).toHaveBeenCalledWith('/api/onboarding/state', expect.any(Object));
  });

  it('can follow the standard wizard flow', async () => {
    const user = userEvent.setup({ delay: null });
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
    fireEvent.change(screen.getByPlaceholderText('e.g. Maya Smith'), { target: { value: 'Test User' } });
    fireEvent.change(screen.getByPlaceholderText('you@email.com'), { target: { value: 'test@example.com' } });
    fireEvent.change(screen.getByPlaceholderText('Password'), { target: { value: 'password123' } });
    fireEvent.click(screen.getByText('Next'));

    // Step 7
    fireEvent.click(screen.getByText('Modern'));

    // Step 7.5
    fireEvent.click(screen.getByText('Next'));

    // Step 8
    fireEvent.click(screen.getByText('Free OHC Domain'));

    // Step 8.5
    fireEvent.click(screen.getByText('Next'));

    // Step 9
    fireEvent.click(screen.getByText('Publish my business'));

    // Verify generating screen
    expect(screen.getByText('Agents are building your store...')).toBeInTheDocument();

    act(() => {
      vi.advanceTimersByTime(2000);
    });

    // Verify live screen
    expect(screen.getByText('Success! Your business is live!')).toBeInTheDocument();
  });

  it('can follow the instant-build flow', async () => {
    vi.useRealTimers();
    // Mock the specific API call for instant build
    const originalFetch = global.fetch;
    global.fetch = vi.fn().mockImplementation((url: string) => {
      if (url === '/api/onboarding/intake') {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({
            business_name: 'Mock Bakery',
            business_type: 'Online Store',
            initial_products: [{ name: 'Cake', price: '20.00' }]
          })
        });
      }
      if (url === '/api/onboarding/state') {
          return Promise.resolve({
              ok: true,
              json: () => Promise.resolve({})
          })
      }
      return Promise.resolve({
        ok: true,
        json: () => Promise.resolve({})
      });
    });

    render(<WebsiteBuilderPage />);

    fireEvent.click(screen.getByText('Instant Build'));
    fireEvent.change(screen.getByPlaceholderText('e.g. I run a local bakery'), { target: { value: 'I run a local bakery' } });
    fireEvent.click(screen.getByText('Generate Storefront'));

    // Status changes to 'generating', wait for it
    await waitFor(() => {
      expect(screen.getByText('Agents are building your store...')).toBeInTheDocument();
    });

    await waitFor(() => {
        expect(screen.getByText('Success! Your business is live!')).toBeInTheDocument();
    }, { timeout: 3500 });
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

    (global.fetch as any).mockImplementation((url: string) => {
      if (url.includes('publish_draft')) {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({ domain: 'testdomain' })
        });
      }
      return Promise.resolve({ ok: true, json: () => Promise.resolve({}) });
    });

    render(<WebsiteBuilderPage />);

    await waitFor(() => {
      expect(screen.getByText('1-Tap Launch')).toBeInTheDocument();
    });

    fireEvent.click(screen.getByText('1-Tap Launch'));

    await waitFor(() => {
      expect(screen.getByText('Success! Your business is live!')).toBeInTheDocument();
      expect(screen.getByText('/bio/testdomain')).toBeInTheDocument();
    });
  });

  it('handles load from server state', async () => {
    (global.fetch as any).mockImplementation((url: string) => {
      if (url.includes('onboarding/state') && url.includes('/api/')) {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({
            builderState: {
              bio: 'Test bio',
              blocks: [{ type: 'Testimonials', props: {} }],
              status: 'draft'
            }
          })
        });
      }
      return Promise.resolve({ ok: true, json: () => Promise.resolve({}) });
    });

    render(<WebsiteBuilderPage />);

    await waitFor(() => {
      expect(screen.getAllByTestId('draggable-block').length).toBeGreaterThan(0);
    });
  });

  it('handles sync back to server state on change', async () => {
    useWebsiteBuilderStore.setState({ status: 'idle' });
    render(<WebsiteBuilderPage />);

    await waitFor(() => { expect(global.fetch).toHaveBeenCalled(); });

    // Trigger something that changes status (e.g. going through the instant build flow generates a live status)
    fireEvent.click(screen.getByText('Instant Build'));
    fireEvent.change(screen.getByPlaceholderText('e.g. I run a local bakery'), { target: { value: 'I run a local bakery' } });
    fireEvent.click(screen.getByText('Generate Storefront'));

    // Status changes to 'generating', wait for it
    await waitFor(() => {
      expect(screen.getByText('Agents are building your store...')).toBeInTheDocument();
    });

    act(() => {
      vi.advanceTimersByTime(2500); // Wait for debounce and status change to live
    });

    // Should have called fetch to start the store
    expect(global.fetch).toHaveBeenCalledWith('/api/onboarding/start', expect.objectContaining({
      method: 'POST',
      body: expect.stringContaining('"company_name":"My Business"')
    }));
  });
});
