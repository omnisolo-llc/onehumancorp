import { render, screen, waitFor } from '@testing-library/react';
import PlanPage from './page';

// Mock matchMedia
Object.defineProperty(window, 'matchMedia', {
  writable: true,
  value: vi.fn().mockImplementation(query => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: vi.fn(),
    removeListener: vi.fn(),
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    dispatchEvent: vi.fn(),
  })),
});

// Mock Next.js router
vi.mock('next/navigation', () => ({
  useRouter() {
    return {
      route: '/',
      pathname: '',
      query: '',
      asPath: '',
      push: vi.fn(),
      replace: vi.fn(),
    };
  },
}));

describe('PlanPage', () => {
  beforeEach(() => {
    vi.resetAllMocks();
    global.fetch = vi.fn();
  });

  it('renders the layout correctly', async () => {
    // Mock successful fetch with default empty data to prevent errors
    (global.fetch as ReturnType<typeof vi.fn>).mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        current_plan: 'Free',
        ai_actions_used: 10,
        storage_used_bytes: 1048576 * 5, // 5MB
        next_bill_estimated: 12.50
      })
    });

    render(<PlanPage />);

    expect(screen.getByText('My Plan')).toBeInTheDocument();
    expect(screen.getByText('Your Current Usage')).toBeInTheDocument();
    expect(screen.getByText('Estimated Next Bill:')).toBeInTheDocument();
    expect(screen.getByText('AI actions used this month')).toBeInTheDocument();
    expect(screen.getByText('Storage used')).toBeInTheDocument();

    await waitFor(() => {
        expect(screen.getByText('$12.50')).toBeInTheDocument();
    });
  });

  it('renders correctly on failed fetch', async () => {
    (global.fetch as ReturnType<typeof vi.fn>).mockRejectedValueOnce(new Error('API error'));

    render(<PlanPage />);

    expect(screen.getByText('My Plan')).toBeInTheDocument();
    expect(screen.getByText('Your Current Usage')).toBeInTheDocument();

    // Check fallback defaults
    expect(screen.getByText('Free')).toBeInTheDocument();
  });
});
