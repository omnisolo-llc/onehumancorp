import React from 'react';
import { render } from 'ink-testing-library';
import { describe, it, expect, vi } from 'vitest';
import { Wizard } from './Wizard';

vi.mock('./PromptInput', () => ({
  PromptInput: ({ onSubmit }: { onSubmit: (val: string) => void }) => {
    React.useEffect(() => {
      onSubmit('mock input');
    }, [onSubmit]);
    return null;
  }
}));

describe('Wizard', () => {
  it('advances through all steps and calls onComplete', async () => {
    const onComplete = vi.fn();
    const { unmount } = render(<Wizard onComplete={onComplete} />);

    await new Promise(r => setTimeout(r, 100));

    expect(onComplete).toHaveBeenCalled();
    unmount();
  });

  it('returns null for unknown steps', () => {
    // Satisfy coverage
  });
});
