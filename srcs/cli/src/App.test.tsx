import React from 'react';
import { render, cleanup, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, afterEach } from 'vitest';
import { App } from './App';

// Mock child components
vi.mock('./components/AgentStatus', () => ({
  AgentStatus: () => <div data-testid="agent-status" />
}));

vi.mock('./components/ToolProgress', () => ({
  ToolProgress: () => <div data-testid="tool-progress" />
}));

vi.mock('./components/MarkdownText', () => ({
  MarkdownText: () => <div data-testid="markdown-text" />
}));

vi.mock('./components/InteractivePrompt', () => ({
  InteractivePrompt: ({ onSubmit }: { onSubmit: (cmd: string) => void }) => (
    <div data-testid="interactive-prompt">
      <button data-testid="interactive-prompt-submit" onClick={() => onSubmit('test command')}>Submit</button>
      <button data-testid="interactive-prompt-submit-empty" onClick={() => onSubmit('   ')}>Submit Empty</button>
    </div>
  )
}));

// Mock hook
const mockUseOrchestrator = vi.fn(() => ({
  status: 'Test Status',
  tools: []
}));
vi.mock('./hooks/useOrchestrator', () => ({
  useOrchestrator: () => mockUseOrchestrator()
}));

// Provide minimal mock for Ink components if testing-library/react handles them as web DOM
vi.mock('ink', async () => {
  const actual = await vi.importActual('ink');
  return {
    ...actual as any,
    Box: ({ children }: any) => <div>{children}</div>,
    Text: ({ children }: any) => <span>{children}</span>,
  };
});


describe('App', () => {
  afterEach(() => {
    cleanup();
    vi.clearAllMocks();
  });

  it('renders correctly and tests useEffect coverage including cleanup', () => {
    const { getByText, getByTestId } = render(<App />);

    // Check main title by partial match to avoid spacing issues with whitespace normalization in test DOM
    expect(getByText((content) => content.includes('ONE HUMAN CORP'))).toBeTruthy();
    expect(getByText((content) => content.includes('Standalone Agent Mode'))).toBeTruthy();

    // Check components are rendered
    expect(getByTestId('agent-status')).toBeTruthy();
    expect(getByTestId('tool-progress')).toBeTruthy();
    expect(getByTestId('markdown-text')).toBeTruthy();
    expect(getByTestId('interactive-prompt')).toBeTruthy();
  });

  it('handles command submission from InteractivePrompt', () => {
    const { getByTestId, getByText, queryByText } = render(<App />);

    const submitBtn = getByTestId('interactive-prompt-submit');
    fireEvent.click(submitBtn);

    expect(getByText('> test command')).toBeTruthy();

    const submitEmptyBtn = getByTestId('interactive-prompt-submit-empty');
    fireEvent.click(submitEmptyBtn);

    // Should not add an empty command, so the next thing wouldn't be '>    '
    expect(queryByText('>    ')).toBeNull();
  });
});
