import { render, screen, fireEvent } from '@testing-library/react';
import { LivePreviewCard } from './LivePreviewCard';
import { describe, it, expect, vi } from 'vitest';

describe('LivePreviewCard', () => {
  it('renders correctly with given props', () => {
    const mockOnBook = vi.fn();
    render(
      <LivePreviewCard
        theme="light"
        serviceName="Test Service"
        onBook={mockOnBook}
        previewStatus=""
        removeBranding={false}
        tenant="my-store"
      />
    );
    expect(screen.getByText('Test Service')).toBeDefined();
    fireEvent.click(screen.getByText('Request a Service'));
    expect(mockOnBook).toHaveBeenCalled();
  });
});
