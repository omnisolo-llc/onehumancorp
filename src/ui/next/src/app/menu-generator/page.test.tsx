import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import MenuGeneratorPage from './page';
import { vi, describe, it, expect, beforeEach } from 'vitest';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('MenuGeneratorPage', () => {
  beforeEach(() => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({ url: '/test-menu' }),
    });
    localStorage.clear();
  });

  it('renders initial state correctly and includes Powered by OHC watermark', () => {
    render(<MenuGeneratorPage />);
    expect(screen.getByText('Menu Details')).toBeTruthy();
    expect(screen.getByText('⚡ Powered by OHC')).toBeTruthy();
  });

  it('generates menu link on valid input', async () => {
    render(<MenuGeneratorPage />);

    const restaurantNameInput = screen.getByLabelText(/Restaurant \/ Store Name/i);
    const menuItemsInput = screen.getByLabelText(/Menu Items/i);

    fireEvent.change(restaurantNameInput, { target: { value: 'Test Restaurant' } });
    fireEvent.change(menuItemsInput, { target: { value: 'Test Item - $10' } });

    const generateBtn = screen.getByText('Generate AI Menu Link');
    fireEvent.click(generateBtn);

    await waitFor(() => {
      expect(screen.getByText('Link Ready! 🎉')).toBeTruthy();
    });
  });
});
