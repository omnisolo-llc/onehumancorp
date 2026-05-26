import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import BuilderPage from './page';
import { vi, describe, it, expect, beforeEach, afterEach } from 'vitest';

vi.mock('../../components/TooltipRegistry', () => ({
  WithTooltip: ({ children }: any) => <div>{children}</div>
}));
vi.mock('../../components/help', () => ({
  useWalkthrough: () => ({ startWalkthrough: vi.fn() })
}));

describe('BuilderPage V2', () => {
  beforeEach(() => {
    global.fetch = vi.fn();
    localStorage.clear();
  });

  afterEach(() => {
    vi.resetAllMocks();
  });

  it('renders Screen 1 Onboarding and transitions to Idle', async () => {
    render(<BuilderPage />);
    expect(screen.getByText('What are you building today?')).toBeInTheDocument();
    fireEvent.click(screen.getByText('Selling Products'));
    await waitFor(() => {
      expect(screen.getByText("Let's build your store")).toBeInTheDocument();
    }, { timeout: 1000 });
  });

  it('completes the wizard and shows AI Architect generating screen', async () => {
    render(<BuilderPage />);
    fireEvent.click(screen.getByText('Selling Products'));
    await waitFor(() => expect(screen.getByText("Let's build your store")).toBeInTheDocument());
    fireEvent.change(screen.getByPlaceholderText('e.g. Acme Corp'), { target: { value: 'Maya Cakes' } });
    fireEvent.change(screen.getByPlaceholderText('e.g. Retail, Consulting, Tech'), { target: { value: 'Bakery' } });
    fireEvent.click(screen.getByText('Next: Choose Vibe'));
    fireEvent.click(screen.getByText('Friendly'));
    fireEvent.click(screen.getByText('Next: Details'));
    fireEvent.change(screen.getByPlaceholderText(/e\.g\. I run a mobile dog grooming service/i), { target: { value: 'I bake amazing custom cakes.' } });

    (global.fetch as any).mockImplementation((url: string) => {
      if (url.includes('generate')) {
        return Promise.resolve({
          ok: true,
          json: async () => ({ pages: [{ blocks: [{ block_type: 'HeroBlock', content: { headline: 'Maya Cakes' } }] }] })
        });
      }
      return Promise.resolve({ ok: true, json: async () => ({}) });
    });

    fireEvent.click(screen.getByText('Build Store'));
    expect(screen.getByText('AI Architect')).toBeInTheDocument();
    expect(screen.getByText('Designing your custom storefront...')).toBeInTheDocument();
    await waitFor(() => {
      expect(screen.getByText('Pick your draft')).toBeInTheDocument();
    });
  });

  it('allows picking a draft and entering Mobile Editor', async () => {
     render(<BuilderPage />);
     fireEvent.click(screen.getByText('Selling Products'));
     await waitFor(() => expect(screen.getByText("Let's build your store")).toBeInTheDocument());
     fireEvent.change(screen.getByPlaceholderText('e.g. Acme Corp'), { target: { value: 'Maya Cakes' } });
     fireEvent.change(screen.getByPlaceholderText('e.g. Retail, Consulting, Tech'), { target: { value: 'Bakery' } });
     fireEvent.click(screen.getByText('Next: Choose Vibe'));
     fireEvent.click(screen.getByText('Friendly'));
     fireEvent.click(screen.getByText('Next: Details'));
     fireEvent.change(screen.getByPlaceholderText(/e\.g\. I run a mobile dog grooming service/i), { target: { value: 'I bake amazing custom cakes.' } });

     (global.fetch as any).mockImplementation((url: string) => {
       if (url.includes('generate')) {
         return Promise.resolve({
           ok: true,
           json: async () => ({ pages: [{ blocks: [{ block_type: 'HeroBlock', content: { headline: 'T' } }] }] })
         });
       }
       return Promise.resolve({ ok: true, json: async () => ({}) });
     });

     fireEvent.click(screen.getByText('Build Store'));
     await waitFor(() => {
       expect(screen.getByText('Pick your draft')).toBeInTheDocument();
     });
     fireEvent.click(screen.getByText('Draft 2'));
     fireEvent.click(screen.getByText('Customize Selected Draft'));
     expect(screen.getByText('Mobile Editor')).toBeInTheDocument();
  });

  it('opens Action Sheet when a block is clicked', async () => {
    render(<BuilderPage />);
    fireEvent.click(screen.getByText('Selling Products'));
    await waitFor(() => expect(screen.getByText("Let's build your store")).toBeInTheDocument());
    fireEvent.change(screen.getByPlaceholderText('e.g. Acme Corp'), { target: { value: 'Maya Cakes' } });
    fireEvent.change(screen.getByPlaceholderText('e.g. Retail, Consulting, Tech'), { target: { value: 'Bakery' } });
    fireEvent.click(screen.getByText('Next: Choose Vibe'));
    fireEvent.click(screen.getByText('Friendly'));
    fireEvent.click(screen.getByText('Next: Details'));
    fireEvent.change(screen.getByPlaceholderText(/e\.g\. I run a mobile dog grooming service/i), { target: { value: 'I bake amazing custom cakes.' } });

    (global.fetch as any).mockImplementation((url: string) => {
      if (url.includes('generate')) {
        return Promise.resolve({
          ok: true,
          json: async () => ({ pages: [{ blocks: [{ block_type: 'HeroBlock', content: { headline: 'Hero Headline' } }] }] })
        });
      }
      return Promise.resolve({ ok: true, json: async () => ({}) });
    });

    fireEvent.click(screen.getByText('Build Store'));
    await waitFor(() => {
      expect(screen.getByText('Pick your draft')).toBeInTheDocument();
    });
    fireEvent.click(screen.getByText('Draft 1'));
    fireEvent.click(screen.getByText('Customize Selected Draft'));
    await waitFor(() => {
      expect(screen.getByText('Mobile Editor')).toBeInTheDocument();
    });
    const heroBlock = screen.getByText('Hero Headline');
    fireEvent.click(heroBlock);
    expect(screen.getByText('Edit Hero Block')).toBeInTheDocument();
  });

    it('handles Auto SEO click failure gracefully', async () => {
    render(<BuilderPage />);
    fireEvent.click(screen.getByText('Selling Products'));
    await waitFor(() => expect(screen.getByText("Let's build your store")).toBeInTheDocument());
    fireEvent.change(screen.getByPlaceholderText('e.g. Acme Corp'), { target: { value: 'Maya Cakes' } });
    fireEvent.change(screen.getByPlaceholderText('e.g. Retail, Consulting, Tech'), { target: { value: 'Bakery' } });
    fireEvent.click(screen.getByText('Next: Choose Vibe'));
    fireEvent.click(screen.getByText('Friendly'));
    fireEvent.click(screen.getByText('Next: Details'));
    fireEvent.change(screen.getByPlaceholderText(/e\.g\. I run a mobile dog grooming service/i), { target: { value: 'I bake amazing custom cakes.' } });

    (global.fetch as any).mockImplementation((url: string) => {
      if (url.includes('generate')) {
        return Promise.resolve({
          ok: true,
          json: async () => ({ pages: [{ blocks: [{ block_type: 'HeroBlock', content: { headline: 'T' } }] }] })
        });
      }
      if (url.includes('publish_draft')) {
        return Promise.resolve({
          ok: true,
          json: async () => ({ domain: 'test' })
        });
      }
      if (url.includes('geo_score')) {
        return Promise.resolve({
          ok: true,
          json: async () => ({ generative_score: 85, recommendations: ['Add location keyword'] })
        });
      }
      if (url.includes('auto_seo')) {
        return Promise.reject(new Error("Network Error"));
      }
      return Promise.resolve({ ok: true, json: async () => ({}) });
    });

    fireEvent.click(screen.getByText('Build Store'));
    await waitFor(() => {
      expect(screen.getByText('Pick your draft')).toBeInTheDocument();
    });

    fireEvent.click(screen.getByText('Draft 1'));
    fireEvent.click(screen.getByText('Customize Selected Draft'));
    await waitFor(() => {
      expect(screen.getByText('Mobile Editor')).toBeInTheDocument();
    });

    const launchBtn = screen.getByText('1-Tap Launch');
    fireEvent.click(launchBtn);
    await waitFor(() => {
        expect(screen.getByText("You're Live!")).toBeInTheDocument();
    });

    const geoBtn = await screen.findByRole('button', { name: /Analyze Visibility/i });
    fireEvent.click(geoBtn);
    await waitFor(() => {
      expect(screen.getByText('85')).toBeInTheDocument();
    });

    const seoBtn = await screen.findByRole('button', { name: /Auto-Apply SEO Metadata/i });
    fireEvent.click(seoBtn);

    // The button text shouldn't change to "Applied ✓" because it failed
    await waitFor(() => {
      expect(screen.getByText('Auto-Apply SEO Metadata')).toBeInTheDocument();
    });
  });

  it('handles Auto SEO click', async () => {
    render(<BuilderPage />);
    fireEvent.click(screen.getByText('Selling Products'));
    await waitFor(() => expect(screen.getByText("Let's build your store")).toBeInTheDocument());
    fireEvent.change(screen.getByPlaceholderText('e.g. Acme Corp'), { target: { value: 'Maya Cakes' } });
    fireEvent.change(screen.getByPlaceholderText('e.g. Retail, Consulting, Tech'), { target: { value: 'Bakery' } });
    fireEvent.click(screen.getByText('Next: Choose Vibe'));
    fireEvent.click(screen.getByText('Friendly'));
    fireEvent.click(screen.getByText('Next: Details'));
    fireEvent.change(screen.getByPlaceholderText(/e\.g\. I run a mobile dog grooming service/i), { target: { value: 'I bake amazing custom cakes.' } });

    (global.fetch as any).mockImplementation((url: string) => {
      if (url.includes('generate')) {
        return Promise.resolve({
          ok: true,
          json: async () => ({ pages: [{ blocks: [{ block_type: 'HeroBlock', content: { headline: 'T' } }] }] })
        });
      }
      if (url.includes('publish_draft')) {
        return Promise.resolve({
          ok: true,
          json: async () => ({ domain: 'test' })
        });
      }
      if (url.includes('geo_score')) {
        return Promise.resolve({
          ok: true,
          json: async () => ({ generative_score: 85, recommendations: ['Add location keyword'] })
        });
      }
      if (url.includes('auto_seo')) {
        return Promise.resolve({
          ok: true,
          json: async () => ({})
        });
      }
      return Promise.resolve({ ok: true, json: async () => ({}) });
    });

    fireEvent.click(screen.getByText('Build Store'));
    await waitFor(() => {
      expect(screen.getByText('Pick your draft')).toBeInTheDocument();
    });

    fireEvent.click(screen.getByText('Draft 1'));
    fireEvent.click(screen.getByText('Customize Selected Draft'));
    await waitFor(() => {
      expect(screen.getByText('Mobile Editor')).toBeInTheDocument();
    });

    const launchBtn = screen.getByText('1-Tap Launch');
    fireEvent.click(launchBtn);
    await waitFor(() => {
        expect(screen.getByText("You're Live!")).toBeInTheDocument();
    });

    const geoBtn = await screen.findByRole('button', { name: /Analyze Visibility/i });
    fireEvent.click(geoBtn);
    await waitFor(() => {
      expect(screen.getByText('85')).toBeInTheDocument();
    });

    const seoBtn = await screen.findByRole('button', { name: /Auto-Apply SEO Metadata/i });
    fireEvent.click(seoBtn);
    await waitFor(() => {
      expect(screen.getByText('Recommendations Applied ✓')).toBeInTheDocument();
    });
  });

  it('handles Geo Analysis click', async () => {
    render(<BuilderPage />);
    fireEvent.click(screen.getByText('Selling Products'));
    await waitFor(() => expect(screen.getByText("Let's build your store")).toBeInTheDocument());
    fireEvent.change(screen.getByPlaceholderText('e.g. Acme Corp'), { target: { value: 'Maya Cakes' } });
    fireEvent.change(screen.getByPlaceholderText('e.g. Retail, Consulting, Tech'), { target: { value: 'Bakery' } });
    fireEvent.click(screen.getByText('Next: Choose Vibe'));
    fireEvent.click(screen.getByText('Friendly'));
    fireEvent.click(screen.getByText('Next: Details'));
    fireEvent.change(screen.getByPlaceholderText(/e\.g\. I run a mobile dog grooming service/i), { target: { value: 'I bake amazing custom cakes.' } });

    (global.fetch as any).mockImplementation((url: string) => {
      if (url.includes('generate')) {
        return Promise.resolve({
          ok: true,
          json: async () => ({ pages: [{ blocks: [{ block_type: 'HeroBlock', content: { headline: 'T' } }] }] })
        });
      }
      if (url.includes('publish_draft')) {
        return Promise.resolve({
          ok: true,
          json: async () => ({ domain: 'test' })
        });
      }
      if (url.includes('geo_score')) {
        return Promise.resolve({
          ok: true,
          json: async () => ({ generative_score: 85, recommendations: ['Add location keyword'] })
        });
      }
      return Promise.resolve({ ok: true, json: async () => ({}) });
    });

    fireEvent.click(screen.getByText('Build Store'));
    await waitFor(() => {
      expect(screen.getByText('Pick your draft')).toBeInTheDocument();
    });

    fireEvent.click(screen.getByText('Draft 1'));
    fireEvent.click(screen.getByText('Customize Selected Draft'));
    await waitFor(() => {
      expect(screen.getByText('Mobile Editor')).toBeInTheDocument();
    });

    const launchBtn = screen.getByText('1-Tap Launch');
    fireEvent.click(launchBtn);
    await waitFor(() => {
        expect(screen.getByText("You're Live!")).toBeInTheDocument();
    });

    const geoBtn = await screen.findByRole('button', { name: /Analyze Visibility/i });
    fireEvent.click(geoBtn);
    await waitFor(() => {
      expect(screen.getByText('85')).toBeInTheDocument();
    });
  });

});
