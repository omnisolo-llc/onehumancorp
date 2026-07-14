import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { HelpWidget } from './help';

describe('HelpWidget', () => {
  it('should be completely removed per help_widget_removal_signoff.md', () => {
    const { container } = render(<HelpWidget />);
    expect(container.innerHTML).toBe('');
  });
});
