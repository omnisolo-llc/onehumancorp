import { render, screen, fireEvent, act } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import Page from './page';

describe('Viral Secret Menu Generator Page', () => {
  it('renders the form and default iframe URL', () => {
    render(<Page />);
    expect(screen.getByText('Viral Secret Menu Generator 🤫')).toBeDefined();

    const iframe = document.querySelector('iframe#previewFrame') as HTMLIFrameElement;
    expect(iframe.src).toContain('/api/v1/growth/secret-menu/embed');
  });

  it('updates the embed URL based on input', () => {
    render(<Page />);

    const itemNameInput = document.querySelector('#itemName') as HTMLInputElement;
    fireEvent.change(itemNameInput, { target: { value: 'Test Burger' } });

    const iframe = document.querySelector('iframe#previewFrame') as HTMLIFrameElement;
    expect(iframe.src).toContain('item_name=Test%20Burger');
  });

  it('handles copy button click', async () => {
    vi.useFakeTimers();
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn().mockImplementation(() => Promise.resolve()),
      },
    });

    render(<Page />);

    const copyBtn = document.querySelector('#copyBtn') as HTMLButtonElement;

    await act(async () => {
      fireEvent.click(copyBtn);
    });

    expect(copyBtn.textContent).toBe('Copied!');

    act(() => {
      vi.advanceTimersByTime(2500);
    });

    expect(copyBtn.textContent).toBe('Copy Link');
    vi.useRealTimers();
  });
});
