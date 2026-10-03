import { render,screen,fireEvent } from '@testing-library/react';
import { describe,it,expect,vi,beforeEach } from 'vitest';
import ShareToUnlockGeneratorPage from './page';

const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: mockPush,
  }),
}));

describe('ShareToUnlockGeneratorPage', () => {
  it('exposes the chosen theme and repeating selection leaves its link unchanged', () => {
    render(<ShareToUnlockGeneratorPage />);
    const light=screen.getByRole('button',{name:'Light'});
    const dark=screen.getByRole('button',{name:'Dark'});
    expect(light).toHaveAttribute('aria-pressed','true');
    expect(dark).toHaveAttribute('aria-pressed','false');
    fireEvent.click(dark);
    expect(light).toHaveAttribute('aria-pressed','false');
    expect(dark).toHaveAttribute('aria-pressed','true');
    const link=screen.getByText(/http.*theme=dark/).textContent;
    fireEvent.click(dark);
    expect(screen.getByText(/http.*theme=dark/).textContent).toBe(link);
    expect(dark).toHaveAttribute('aria-pressed','true');
  });

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders correctly', () => {
    render(<ShareToUnlockGeneratorPage />);
    expect(screen.getByText('Share-to-Unlock Generator 🔓')).toBeDefined();
    expect(screen.getByText('Campaign Settings')).toBeDefined();
    expect(screen.getByText('Preview only: Share-to-Unlock Page')).toBeDefined();
    expect(screen.getByText(/Share verification and automatic coupon unlocking are unavailable/)).toBeDefined();
  });

  it('updates preview when inputs change', () => {
    render(<ShareToUnlockGeneratorPage />);

    // In the actual component, the input doesn't have an id or htmlFor matching the label exactly for getByLabelText to work perfectly without id mapping
    const titleInputs = screen.getAllByPlaceholderText('e.g. Secret Weekend Deal');
    const titleInput = titleInputs[0];
    fireEvent.change(titleInput, { target: { value: 'My Awesome Promo' } });

    // Check if the preview updates (it appears twice: one in input, one in preview)
    const textElements = screen.getAllByText('My Awesome Promo');
    expect(textElements.length).toBeGreaterThan(0);
  });
});
