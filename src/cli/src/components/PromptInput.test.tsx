/* @vitest-environment jsdom */
import React from 'react';
import { render, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import { PromptInput } from './PromptInput.js';

// Mock ink components
vi.mock('ink', () => {
  return {
    Box: ({ children }: any) => <div data-testid="ink-box">{children}</div>,
    Text: ({ children, color }: any) => <span data-testid="ink-text" data-color={color}>{children}</span>,
  };
});

// Mock ink-text-input since it's hard to test directly in jsdom
vi.mock('ink-text-input', () => {
  return {
    default: ({ value, onChange, onSubmit }: any) => (
      <input
        data-testid="ink-text-input"
        value={value}
        onChange={(e) => onChange(e.target.value)}
        onKeyDown={(e) => {
          if (e.key === 'Enter') {
            onSubmit(value);
          }
        }}
      />
    ),
  };
});

describe('PromptInput', () => {
  it('renders with default prompt text', () => {
    const { container, getByText } = render(<PromptInput onSubmit={vi.fn()} />);
    expect(getByText('>')).toBeDefined();
    expect(container.querySelector('[data-testid="ink-text-input"]')).toBeDefined();
  });

  it('renders with custom prompt text', () => {
    const { getByText } = render(<PromptInput onSubmit={vi.fn()} promptText="Test >" />);
    expect(getByText('Test >')).toBeDefined();
  });

  it('calls onSubmit and clears value on enter', () => {
    const onSubmit = vi.fn();
    const { container } = render(<PromptInput onSubmit={onSubmit} />);

    const input = container.querySelector('[data-testid="ink-text-input"]') as HTMLInputElement;
    fireEvent.change(input, { target: { value: 'hello agent' } });
    expect(input.value).toBe('hello agent');

    fireEvent.keyDown(input, { key: 'Enter' });

    expect(onSubmit).toHaveBeenCalledWith('hello agent');
    expect(input.value).toBe('');
  });
});
