import { render, screen, fireEvent, act } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import InteractiveQuoteGeneratorPage from './page';

const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: mockPush,
  }),
}));

describe('InteractiveQuoteGeneratorPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders correctly', async () => {
    await act(async () => {
      render(<InteractiveQuoteGeneratorPage />);
    });
    expect(screen.getByText('Interactive Quote Generator 🧮')).toBeDefined();
    expect(screen.getByText('Widget Settings')).toBeDefined();
    expect(screen.getByText('Live Preview')).toBeDefined();
  });

  it('updates preview when inputs change', async () => {
    await act(async () => {
      render(<InteractiveQuoteGeneratorPage />);
    });

    const titleInput = screen.getByPlaceholderText('e.g. Custom Cake Design');
    await act(async () => {
        fireEvent.change(titleInput, { target: { value: 'Landscaping Service' } });
    });

    // Check if the preview updates
    const textElements = screen.getAllByText('Landscaping Service Quote');
    expect(textElements.length).toBeGreaterThan(0);
  });
});
