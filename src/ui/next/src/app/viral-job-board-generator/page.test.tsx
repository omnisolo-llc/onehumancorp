import { render,screen,fireEvent } from '@testing-library/react';
import { describe,it,expect,vi,beforeEach } from 'vitest';
import { renderToString } from 'react-dom/server';
import ViralJobBoardGeneratorPage from './page';

const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: mockPush }),
}));

vi.mock('../components/PoweredByOmniSolo', () => ({
  PoweredByOmniSolo: () => <div data-testid="powered-by-omnisolo" />,
}));

describe('ViralJobBoardGeneratorPage', () => {
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
    render(<ViralJobBoardGeneratorPage />);
    expect(screen.getByText('Viral Job Board Generator 📢')).toBeDefined();
    expect(screen.getByText('Board Settings')).toBeDefined();
    expect(screen.getByText('Preview: Your Job Board Page')).toBeDefined();
  });

  it('updates form inputs and preview', () => {
    render(<ViralJobBoardGeneratorPage />);

    const titleInput = screen.getByPlaceholderText('e.g. We are hiring!');
    fireEvent.change(titleInput, { target: { value: 'Join the Revolution' } });

    const descInput = screen.getByPlaceholderText('e.g. Join our team and help us build the future.');
    fireEvent.change(descInput, { target: { value: 'We are looking for misfits.' } });

    // Check if preview updates
    const titleDisplays = screen.getAllByText('Join the Revolution');
    expect(titleDisplays.length).toBeGreaterThan(0);

    const descDisplays = screen.getAllByText('We are looking for misfits.');
    expect(descDisplays.length).toBeGreaterThan(0);
  });

  it('enables text editing after hydration and clears the description to its fallback', () => {
    const container = document.createElement('div');
    container.innerHTML = renderToString(<ViralJobBoardGeneratorPage />);

    const titleInput = container.querySelector('input')!;
    const descInput = container.querySelector('textarea')!;
    expect(titleInput).toBeDisabled();
    expect(descInput).toBeDisabled();

    document.body.appendChild(container);
    render(<ViralJobBoardGeneratorPage />, { container, hydrate: true });
    expect(titleInput).toBeEnabled();
    expect(descInput).toBeEnabled();
    fireEvent.change(descInput, { target: { value: '' } });
    expect(descInput).toHaveValue('');
    expect(screen.getByText('Join our team.', { exact: true })).toBeVisible();
  });

  it('holds copying while no job board publication exists', async () => {
    render(<ViralJobBoardGeneratorPage />);

    const copyBtn = screen.getByRole('button', { name: 'Copy Link' });
    fireEvent.click(copyBtn);

    expect(copyBtn).toBeDisabled();
    expect(navigator.clipboard.writeText).not.toHaveBeenCalled();
    expect(screen.queryByRole('button', { name: 'Copied!' })).toBeNull();
  });

  it('navigates back to dashboard', () => {
    render(<ViralJobBoardGeneratorPage />);

    const backBtn = screen.getByRole('button', { name: /Back to Dashboard/i });
    fireEvent.click(backBtn);

    expect(mockPush).toHaveBeenCalledWith('/dashboard');
  });
});
