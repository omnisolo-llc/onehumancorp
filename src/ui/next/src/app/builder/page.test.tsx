import { render,screen,fireEvent,waitFor,cleanup } from '@testing-library/react';
import BuilderPage from './page';
import { vi,describe,it,expect,beforeEach,afterEach } from 'vitest';
import { useBuilderStore } from './store';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import { installBuilderLocks } from './testLocks';
let generatedHeadline = 'Maya Cakes';

// Mock TooltipRegistry and help components
vi.mock('../../components/TooltipRegistry', () => ({
  WithTooltip: ({ children }: { children?: import('react').ReactNode }) => <div>{children}</div>
}));
vi.mock('../../components/help', () => ({
  useWalkthrough: () => ({ startWalkthrough: vi.fn() })
}));

describe('BuilderPage V2', () => {
  beforeEach(() => {
    localStorage.clear(); notifyQueueIdentityChange(); installBuilderLocks(); generatedHeadline = 'Maya Cakes';
    global.fetch = vi.fn().mockImplementation((url) => {
      if (String(url).endsWith('/session-identity')) return Promise.resolve(Response.json({ userId: 'owner', tenantId: 'business', expiresAt: Date.now() + 60_000 }));
      if (url === '/api/v1/builder/generate') return Promise.resolve(Response.json({ pages: [{ blocks: [{ block_type: 'HeroBlock', content: { headline: generatedHeadline } }] }] }));
      if (url === "/api/v1/walkthrough/store-setup") {
         return Promise.resolve(Response.json([], { status: 200 }));
      }
      return Promise.resolve(Response.json({}, { status: 200 }));
    });
    localStorage.clear();
    useBuilderStore.setState({
      bio: "",
      businessName: "",
      businessCategory: "",
      vibe: "",
      wizardStep: 1,
      blocks: [],
      drafts: [],
      status: "onboarding",
      businessGoal: null,
      liveUrl: "",
    });
  });

  afterEach(() => {
    cleanup(); notifyQueueIdentityChange(); vi.resetAllMocks();
  });

  it('renders Screen 1 Onboarding and transitions to Idle', async () => {
    render(<BuilderPage />);

    expect(await screen.findByText('What are you building today?')).toBeTruthy();

    const productsBtn = screen.getByText('Selling Products');
    fireEvent.click(productsBtn);

    await waitFor(() => {
      expect(screen.getByText("Let's build your store")).toBeTruthy();
    }, { timeout: 1000 });
  });

  it('completes the wizard and shows the draft preparation screen', async () => {
    render(<BuilderPage />);

    // Onboarding
    fireEvent.click(await screen.findByText('Selling Products'));
    await waitFor(() => { screen.getByText('Business Name'); }, { timeout: 1000 });

    // Step 1
    fireEvent.change(screen.getByPlaceholderText('e.g. Acme Corp'), { target: { value: 'Maya Cakes' } });
    fireEvent.change(screen.getByPlaceholderText('e.g. Retail, Consulting, Tech'), { target: { value: 'Bakery' } });
    fireEvent.click(screen.getByText('Next: Choose Vibe'));

    // Step 2
    fireEvent.click(screen.getByText('Friendly'));
    fireEvent.click(screen.getByText('Next: Details'));

    // Step 3
    fireEvent.change(screen.getByPlaceholderText(/e\.g\. I run a mobile dog grooming service/i), { target: { value: 'I bake amazing custom cakes.' } });

    generatedHeadline = 'Maya Cakes';

    fireEvent.click(screen.getByText('Build Store'));

    expect(screen.getByText('Draft builder')).toBeTruthy();
    expect(screen.getByText('Preparing your storefront draft...')).toBeTruthy();

    await waitFor(() => {
      expect(screen.getByText('Pick your draft')).toBeTruthy();
    });
  });

  it('allows picking a draft and entering Mobile Editor', async () => {
     // Mock state for selection
     render(<BuilderPage />);
     // Fast forward to selection (would be better with state injection if possible, but we'll follow the flow)
     fireEvent.click(await screen.findByText('Showcasing Work'));
     await waitFor(() => { screen.getByText('Business Name'); }, { timeout: 1000 });

     fireEvent.change(screen.getByPlaceholderText('e.g. Acme Corp'), { target: { value: 'Testing' } });
     fireEvent.change(screen.getByPlaceholderText('e.g. Retail, Consulting, Tech'), { target: { value: 'Testing' } });
     fireEvent.click(screen.getByText('Next: Choose Vibe'));
     fireEvent.click(screen.getByText('Minimalist'));
     fireEvent.click(screen.getByText('Next: Details'));
     generatedHeadline = 'T';
     fireEvent.click(screen.getByText('Build Store'));

     await waitFor(() => {
       expect(screen.getByText('Pick your draft')).toBeTruthy();
     });

     fireEvent.click(screen.getByText('Draft 1'));
     fireEvent.click(screen.getByText('Customize Selected Draft'));

     expect(screen.getByText('Mobile Editor')).toBeTruthy();
  });

  it('opens Action Sheet when a block is clicked', async () => {
    // We'll skip the full flow for brevity if we can, but let's just finish it.
    render(<BuilderPage />);
    fireEvent.click(await screen.findByText('Offering Services'));
    await waitFor(() => { screen.getByText('Business Name'); }, { timeout: 1000 });

    fireEvent.change(screen.getByPlaceholderText('e.g. Acme Corp'), { target: { value: 'Testing' } });
    fireEvent.change(screen.getByPlaceholderText('e.g. Retail, Consulting, Tech'), { target: { value: 'Testing' } });
    fireEvent.click(screen.getByText('Next: Choose Vibe'));
    fireEvent.click(screen.getByText('Minimalist'));
    fireEvent.click(screen.getByText('Next: Details'));
    generatedHeadline = 'Hero Headline';
    fireEvent.click(screen.getByText('Build Store'));

    await waitFor(() => {
      fireEvent.click(screen.getByText('Customize Selected Draft'));
    });

    const heroBlock = screen.getByText('Hero Headline');
    fireEvent.click(heroBlock);

    expect(screen.getByText('Edit Hero Block')).toBeTruthy();
  });
});
