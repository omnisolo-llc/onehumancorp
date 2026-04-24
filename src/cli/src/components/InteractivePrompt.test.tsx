import React from 'react';
import { render, cleanup, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, afterEach } from 'vitest';
import { InteractivePrompt } from './InteractivePrompt';

// Mock ink-text-input
vi.mock('ink-text-input', () => {
  return {
    default: ({ value, onChange, onSubmit, placeholder }: any) => {
      return (
        <div data-testid="text-input">
          <input
            type="text"
            value={value}
            onChange={(e) => onChange(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === 'Enter') {
                onSubmit(value);
              }
            }}
            placeholder={placeholder}
            data-testid="ink-text-input-mock"
          />
        </div>
      );
    }
  };
});

describe('InteractivePrompt', () => {
  afterEach(() => {
    cleanup();
  });

  it('renders correctly', () => {
    const { getByTestId, getByText } = render(<InteractivePrompt onSubmit={vi.fn()} />);
    expect(getByText('❯')).toBeTruthy();
    expect(getByTestId('ink-text-input-mock')).toBeTruthy();
  });

  it('handles input and submission', () => {
    const onSubmit = vi.fn();
    const { getByTestId } = render(<InteractivePrompt onSubmit={onSubmit} />);

    const input = getByTestId('ink-text-input-mock') as HTMLInputElement;

    fireEvent.change(input, { target: { value: 'hello' } });
    fireEvent.keyDown(input, { key: 'Enter', code: 'Enter', charCode: 13 });

    expect(onSubmit).toHaveBeenCalledWith('hello');
  });
});
