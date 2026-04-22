import React from 'react';
import { render } from 'ink-testing-library';
import { describe, it, expect } from 'vitest';
import { MarkdownText } from './MarkdownText';

describe('MarkdownText', () => {
  it('renders correctly', () => {
    const md = `# Title
## Subtitle
- item 1
- item 2
text line
`;
    const { lastFrame } = render(<MarkdownText content={md} />);
    const output = lastFrame();

    expect(output).toContain('Title');
    expect(output).toContain('Subtitle');
    expect(output).toContain('• item 1');
    expect(output).toContain('• item 2');
    expect(output).toContain('text line');
  });
});
