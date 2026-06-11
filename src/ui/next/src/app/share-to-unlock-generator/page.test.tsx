import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ShareToUnlockGeneratorPage from './page';

const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: mockPush,
  }),
}));

describe('ShareToUnlockGeneratorPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders correctly', () => {
    render(<ShareToUnlockGeneratorPage />);
    expect(screen.getByText('Share-to-Unlock Generator 🔓')).toBeDefined();
    expect(screen.getByText('Campaign Settings')).toBeDefined();
    expect(screen.getByText('Preview: Your Share-to-Unlock Page')).toBeDefined();
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
