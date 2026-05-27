import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import BuilderPage from './page';
import { vi, describe, it, expect, beforeEach, afterEach } from 'vitest';
import { useBuilderStore } from './store';

// Mock TooltipRegistry and help components
vi.mock('../../components/TooltipRegistry', () => ({
  WithTooltip: ({ children }: any) => <div>{children}</div>
}));
vi.mock('../../components/help', () => ({
  useWalkthrough: () => ({ startWalkthrough: vi.fn() })
}));

describe('BuilderPage V2', () => {
  beforeEach(() => {
    global.fetch = vi.fn().mockResolvedValue({ json: () => Promise.resolve({ data: {} }) });
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
    vi.resetAllMocks();
  });

  it('renders Screen 1 Onboarding and transitions to Idle', async () => {
    render(<BuilderPage />);

    expect(screen.getByText('What are you building today?')).toBeTruthy();

    const productsBtn = screen.getByText('Selling Products');
    fireEvent.click(productsBtn);

    await waitFor(() => {
      expect(screen.getByText("Let's build your store")).toBeTruthy();
    }, { timeout: 1000 });
  });

  it('completes the wizard and shows AI Architect generating screen', async () => {
    render(<BuilderPage />);

    // Onboarding
    fireEvent.click(screen.getByText('Selling Products'));
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

    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        pages: [{
          blocks: [
            { block_type: 'HeroBlock', content: { headline: 'Maya Cakes' } }
          ]
        }]
      })
    });

    fireEvent.click(screen.getByText('Build Store'));

    expect(screen.getByText('AI Architect')).toBeTruthy();
    expect(screen.getByText('Designing your custom storefront...')).toBeTruthy();

    await waitFor(() => {
      expect(screen.getByText('Pick your draft')).toBeTruthy();
    });
  });

  it('allows picking a draft and entering Mobile Editor', async () => {
     // Mock state for selection
     render(<BuilderPage />);
     // Fast forward to selection (would be better with state injection if possible, but we'll follow the flow)
     fireEvent.click(screen.getByText('Showcasing Work'));
     await waitFor(() => { screen.getByText('Business Name'); }, { timeout: 1000 });

     fireEvent.change(screen.getByPlaceholderText('e.g. Acme Corp'), { target: { value: 'Testing' } });
     fireEvent.change(screen.getByPlaceholderText('e.g. Retail, Consulting, Tech'), { target: { value: 'Testing' } });
     fireEvent.click(screen.getByText('Next: Choose Vibe'));
     fireEvent.click(screen.getByText('Minimalist'));
     fireEvent.click(screen.getByText('Next: Details'));
     (global.fetch as any).mockResolvedValueOnce({
       ok: true,
       json: async () => ({ pages: [{ blocks: [{ block_type: 'HeroBlock', content: { headline: 'T' } }] }] })
     });
     fireEvent.click(screen.getByText('Build Store'));

     await waitFor(() => {
       expect(screen.getByText('Pick your draft')).toBeTruthy();
     });

     fireEvent.click(screen.getByText('Draft 2'));
     fireEvent.click(screen.getByText('Customize Selected Draft'));

     expect(screen.getByText('Mobile Editor')).toBeTruthy();
  });

  it('opens Action Sheet when a block is clicked', async () => {
    // We'll skip the full flow for brevity if we can, but let's just finish it.
    render(<BuilderPage />);
    fireEvent.click(screen.getByText('Offering Services'));
    await waitFor(() => { screen.getByText('Business Name'); }, { timeout: 1000 });

    fireEvent.change(screen.getByPlaceholderText('e.g. Acme Corp'), { target: { value: 'Testing' } });
    fireEvent.change(screen.getByPlaceholderText('e.g. Retail, Consulting, Tech'), { target: { value: 'Testing' } });
    fireEvent.click(screen.getByText('Next: Choose Vibe'));
    fireEvent.click(screen.getByText('Minimalist'));
    fireEvent.click(screen.getByText('Next: Details'));
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ pages: [{ blocks: [{ block_type: 'HeroBlock', content: { headline: 'Hero Headline' } }] }] })
    });
    fireEvent.click(screen.getByText('Build Store'));

    await waitFor(() => {
      fireEvent.click(screen.getByText('Customize Selected Draft'));
    });

    const heroBlock = screen.getByText('Hero Headline');
    fireEvent.click(heroBlock);

    expect(screen.getByText('Edit Hero Block')).toBeTruthy();
  });
});
