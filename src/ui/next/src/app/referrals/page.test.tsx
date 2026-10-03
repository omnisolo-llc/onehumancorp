import { render,screen,waitFor,act } from '@testing-library/react';
import { describe,it,expect,vi,beforeEach } from 'vitest';
import ReferralsPage from './page';

vi.mock('../components/PoweredByOmniSolo', () => ({
  PoweredByOmniSolo: () => <div data-testid="powered-by-omnisolo" />
}));

vi.mock('../components/GrowthReferralWidget', () => ({
  default: () => <div data-testid="growth-referral-widget" />
}));

function mockReferral(reply: () => Promise<Response>) {
  vi.mocked(global.fetch).mockImplementation(async url => {
    if (url === '/api/v1/auth/session-identity') return Response.json({error:'identity unavailable in this referral-only fixture'}, {status:401});
    if (url === '/api/v1/growth/referrals/generate') return reply();
    throw new Error(`Unexpected request: ${String(url)}`);
  });
}

describe('ReferralsPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    global.fetch = vi.fn();

    // Mock localStorage
    const localStorageMock = {
      getItem: vi.fn(),
      setItem: vi.fn(),
      clear: vi.fn()
    };
    Object.defineProperty(window, 'localStorage', {
      value: localStorageMock
    });
  });

  it('renders loading state initially', async () => {
    // Return an unresolved promise to keep it in loading state
    vi.mocked(global.fetch, { partial: true }).mockImplementation(() => new Promise(() => {}));

    await act(async () => {
      render(<ReferralsPage />);
    });
    expect(screen.getByText('Generating your unique link...')).toBeDefined();

    // Copy button should be disabled
    const copyButton = screen.getByText('Copy Link');
    expect(copyButton.hasAttribute('disabled')).toBe(true);
  });

  it('renders how it works section', async () => {
    mockReferral(async () => Response.json({referral_link:'https://cloud.omnisolo.co/ref/test1234'}));
    await act(async () => {
      render(<ReferralsPage />);
    });
    expect(screen.getByText('How it works')).toBeDefined();
    expect(screen.getByText('Share Link')).toBeDefined();
    expect(screen.getByText('They Sign Up')).toBeDefined();
    expect(screen.getByText('You Get $50')).toBeDefined();
  });

  it('fetches and displays dynamic referral link', async () => {
    mockReferral(async () => Response.json({referral_link:'https://cloud.omnisolo.co/ref/test1234'}));

    await act(async () => {
      render(<ReferralsPage />);
    });

    // Wait for the fetch to resolve
    await waitFor(() => {
      expect(screen.queryByText('Generating your unique link...')).toBeNull();
    });

    const referralSpan = document.getElementById('referral-link');
    expect(referralSpan?.textContent).toBe('https://cloud.omnisolo.co/ref/test1234');

    // Copy button should be enabled
    const copyButton = screen.getByText('Copy Link');
    expect(copyButton.hasAttribute('disabled')).toBe(false);
  });

  it('falls back to tenant link on api error', async () => {
    const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
    mockReferral(async () => {throw new Error('API failed');});
    vi.mocked(window.localStorage.getItem, { partial: true }).mockReturnValue('my-tenant-store');

    await act(async () => {
      render(<ReferralsPage />);
    });

    await waitFor(() => {
      expect(screen.queryByText('Generating your unique link...')).toBeNull();
    });

    const referralSpan = document.getElementById('referral-link');
    expect(referralSpan?.textContent).toBe('http://localhost:3000/onboarding?ref=my-tenant-store');

    consoleErrorSpy.mockRestore();
  });

  it('renders Powered by OmniSolo footer', async () => {
    mockReferral(async () => Response.json({referral_link:'https://cloud.omnisolo.co/ref/test1234'}));
    await act(async () => {
      render(<ReferralsPage />);
    });
    expect(screen.getByTestId('powered-by-omnisolo')).toBeDefined();
  });

  it('renders embed code snippet area', async () => {
    mockReferral(async () => Response.json({referral_link:'https://cloud.omnisolo.co/ref/test1234'}));
    await act(async () => {
      render(<ReferralsPage />);
    });
    const embedCodeEl = document.getElementById('embed-code');
    expect(embedCodeEl).not.toBeNull();
    expect(embedCodeEl?.textContent).toContain('<iframe src="https://mybusiness.cloud.omnisolo.co');
  });
});
