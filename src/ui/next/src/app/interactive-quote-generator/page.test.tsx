import { render, screen, fireEvent, act, waitFor } from '@testing-library/react';
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
    render(<InteractiveQuoteGeneratorPage />);
    await act(async () => {});
    expect(screen.getByText('Interactive Quote Generator 🧮')).toBeDefined();
    expect(screen.getByText('Widget Settings')).toBeDefined();
    expect(screen.getByText('Live Preview')).toBeDefined();
  });

  it('updates preview when inputs change', async () => {
    render(<InteractiveQuoteGeneratorPage />);

    const titleInput = screen.getByPlaceholderText('e.g. Custom Cake Design');
    fireEvent.change(titleInput, { target: { value: 'Landscaping Service' } });
    await act(async () => {}); // wait for state updates

    // Check if the preview updates
    const textElements = screen.getAllByText('Landscaping Service Quote');
    expect(textElements.length).toBeGreaterThan(0);
  });
});
