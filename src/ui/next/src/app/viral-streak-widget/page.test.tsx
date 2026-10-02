import { render,screen,fireEvent } from '@testing-library/react';
import { describe,it,expect,vi,beforeEach } from 'vitest';
import ViralStreakWidgetPage from './page';

const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: mockPush }),
}));

vi.mock('../components/PoweredByOmniSolo', () => ({
  PoweredByOmniSolo: () => <div data-testid="powered-by-omnisolo" />,
}));

describe('ViralStreakWidgetPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn().mockResolvedValue(undefined),
      },
    });

    const localStorageMock = {
      getItem: vi.fn((key) => {
        if (key === 'tenant_id' || key === 'tenant') return 'test-tenant';
        return null;
      }),
      setItem: vi.fn(),
      clear: vi.fn()
    };
    Object.defineProperty(window, 'localStorage', {
      value: localStorageMock,
      writable: true
    });
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders correctly', () => {
    render(<ViralStreakWidgetPage />);
    expect(screen.getByText('Viral Streak Widget 📅')).toBeDefined();
    expect(screen.getByText('Streak Title')).toBeDefined();
    expect(screen.getByText('Live Preview')).toBeDefined();
  });

  it('updates form inputs and preview', () => {
    render(<ViralStreakWidgetPage />);

    const titleInput = screen.getByDisplayValue('Daily Login Streak');
    fireEvent.change(titleInput, { target: { value: '7-Day Coffee Run' } });

    const goalInput = screen.getByDisplayValue('7');
    fireEvent.change(goalInput, { target: { value: '5' } });

    const rewardInput = screen.getByDisplayValue('Free Coffee');
    fireEvent.change(rewardInput, { target: { value: 'Free Donut' } });

    // Check if preview updates
    const titleDisplays = screen.getAllByText('7-Day Coffee Run');
    expect(titleDisplays.length).toBeGreaterThan(0);

    const goalDisplays = screen.getAllByText(/Configured goal: 5 days/);
    expect(goalDisplays.length).toBeGreaterThan(0);

    const rewardDisplays = screen.getAllByText(/Free Donut/);
    expect(rewardDisplays.length).toBeGreaterThan(0);
  });

  it('does not copy an embed for an unavailable streak endpoint', () => {
    render(<ViralStreakWidgetPage />);
    const getCodeBtn = screen.getByRole('button', { name: 'Get Embed Code' });
    expect(getCodeBtn).toBeDisabled(); fireEvent.click(getCodeBtn);
    expect(screen.queryByRole('button', { name: 'Copy Code' })).toBeNull();
    expect(navigator.clipboard.writeText).not.toHaveBeenCalled();
  });

  it('shows paywall when removing branding without pro', () => {
    render(<ViralStreakWidgetPage />);

    const checkbox = screen.getByLabelText(/Remove "Powered by OmniSolo" Badge/);
    fireEvent.click(checkbox);

    expect(screen.getByText('Upgrade to Remove Branding')).toBeDefined();
  });

  it('navigates back to dashboard', () => {
    render(<ViralStreakWidgetPage />);

    const backBtn = screen.getByRole('button', { name: /Back to Dashboard/i });
    fireEvent.click(backBtn);

    expect(mockPush).toHaveBeenCalledWith('/dashboard');
  });
});
