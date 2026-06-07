import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import ReviewWallPage from './page';
import { vi } from 'vitest';

describe('ReviewWallPage', () => {
  beforeEach(() => {
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn().mockImplementation(() => Promise.resolve()),
      },
    });
    window.alert = vi.fn();
  });

  it('renders the Review Wall page correctly', () => {
    render(<ReviewWallPage />);
    expect(screen.getByText('Review Wall Widget ⭐')).toBeInTheDocument();
    expect(screen.getByText('Embed Your Best Reviews')).toBeInTheDocument();
  });

  it('updates embed code when tenant changes', () => {
    render(<ReviewWallPage />);
    const tenantInput = screen.getByDisplayValue('my-business');
    fireEvent.change(tenantInput, { target: { value: 'new-tenant' } });

    // There are multiple textboxes (input and textarea), so we get the textarea explicitly
    const textboxes = screen.getAllByRole('textbox');
    const textarea = textboxes.find(t => t.tagName === 'TEXTAREA') as HTMLTextAreaElement;
    expect(textarea.value).toContain('tenant=new-tenant');
  });

  it('copies embed code to clipboard', () => {
    render(<ReviewWallPage />);
    const copyButton = screen.getByText('Copy Embed Code');
    fireEvent.click(copyButton);
    expect(navigator.clipboard.writeText).toHaveBeenCalled();
    expect(window.alert).toHaveBeenCalledWith('Embed code copied to clipboard!');
  });
});
