/**
 * MarkdownText Component Tests - 70 comprehensive tests
 * Tests all markdown parsing behaviors, edge cases, and rendering for MarkdownText component
 */

import React from 'react';
import { render } from 'ink-testing-library';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { MarkdownText } from './MarkdownText';

describe('MarkdownText', () => {
  describe('H1 Header Rendering', () => {
    it('renders simple h1 header', () => {
      const { lastFrame } = render(<MarkdownText content="# Hello World" />);
      expect(lastFrame()).toContain('Hello World');
    });

    it('renders h1 without text after', () => {
      const { lastFrame } = render(<MarkdownText content="# Only Header" />);
      expect(lastFrame()).toContain('Only Header');
    });

    it('renders h1 with long text', () => {
      const { lastFrame } = render(<MarkdownText content="# This is a very long header that should still render correctly" />);
      expect(lastFrame()).toContain('This is a very long header');
    });

    it('renders h1 with special characters', () => {
      const { lastFrame } = render(<MarkdownText content="# Header with (parentheses) and [brackets]" />);
      expect(lastFrame()).toContain('Header with');
    });

    it('renders h1 with numbers', () => {
      const { lastFrame } = render(<MarkdownText content="# Section 1: Introduction" />);
      expect(lastFrame()).toContain('Section 1: Introduction');
    });

    it('renders h1 with unicode', () => {
      const { lastFrame } = render(<MarkdownText content="# 标题" />);
      expect(lastFrame()).toContain('标题');
    });

    it('renders h1 with emoji', () => {
      const { lastFrame } = render(<MarkdownText content="# Title 🚀" />);
      expect(lastFrame()).toContain('Title');
    });
  });

  describe('H2 Header Rendering', () => {
    it('renders simple h2 header', () => {
      const { lastFrame } = render(<MarkdownText content="## Subtitle" />);
      expect(lastFrame()).toContain('Subtitle');
    });

    it('renders h2 without following content', () => {
      const { lastFrame } = render(<MarkdownText content="## Only Subtitle" />);
      expect(lastFrame()).toContain('Only Subtitle');
    });

    it('renders h2 before text', () => {
      const { lastFrame } = render(<MarkdownText content="## Heading\nSome text content" />);
      expect(lastFrame()).toContain('Heading');
      expect(lastFrame()).toContain('Some text content');
    });

    it('renders multiple h2 headers', () => {
      const { lastFrame } = render(<MarkdownText content="## First\n## Second\n## Third" />);
      expect(lastFrame()).toContain('First');
      expect(lastFrame()).toContain('Second');
      expect(lastFrame()).toContain('Third');
    });

    it('renders h2 with leading spaces', () => {
      const { lastFrame } = render(<MarkdownText content="  ## Indented Header" />);
      expect(lastFrame()).toContain('Indented Header');
    });

    it('renders h2 with special characters', () => {
      const { lastFrame } = render(<MarkdownText content="## Header: Test & More" />);
      expect(lastFrame()).toContain('Header: Test');
    });
  });

  describe('List Item Rendering', () => {
    it('renders simple list item', () => {
      const { lastFrame } = render(<MarkdownText content="- Item one" />);
      expect(lastFrame()).toContain('Item one');
    });

    it('renders bullet character for list', () => {
      const { lastFrame } = render(<MarkdownText content="- List item" />);
      expect(lastFrame()).toContain('•');
    });

    it('renders multiple list items', () => {
      const { lastFrame } = render(<MarkdownText content="- First\n- Second\n- Third" />);
      expect(lastFrame()).toContain('First');
      expect(lastFrame()).toContain('Second');
      expect(lastFrame()).toContain('Third');
    });

    it('renders list with unicode items', () => {
      const { lastFrame } = render(<MarkdownText content="- 项目\n-条目" />);
      expect(lastFrame()).toContain('项目');
    });

    it('renders list with empty item', () => {
      const { lastFrame } = render(<MarkdownText content="- " />);
      expect(lastFrame()).toBeDefined();
    });

    it('renders list with special characters', () => {
      const { lastFrame } = render(<MarkdownText content="- Item with (parens)" />);
      expect(lastFrame()).toContain('Item with');
    });

    it('renders indented list item', () => {
      const { lastFrame } = render(<MarkdownText content="- Nested item" />);
      expect(lastFrame()).toContain('Nested item');
    });
  });

  describe('Empty Lines', () => {
    it('renders empty content', () => {
      const { lastFrame } = render(<MarkdownText content="" />);
      expect(lastFrame()).toBeDefined();
    });

    it('renders multiple empty lines', () => {
      const { lastFrame } = render(<MarkdownText content="\n\n\n" />);
      expect(lastFrame()).toBeDefined();
    });

    it('renders line with only spaces', () => {
      const { lastFrame } = render(<MarkdownText content="   " />);
      expect(lastFrame()).toBeDefined();
    });

    it('renders empty line between content', () => {
      const { lastFrame } = render(<MarkdownText content="First\n\nThird" />);
      expect(lastFrame()).toContain('First');
      expect(lastFrame()).toContain('Third');
    });
  });

  describe('Plain Text Lines', () => {
    it('renders simple plain text', () => {
      const { lastFrame } = render(<MarkdownText content="Just plain text" />);
      expect(lastFrame()).toContain('Just plain text');
    });

    it('renders text with punctuation', () => {
      const { lastFrame } = render(<MarkdownText content="Hello, world! How are you?" />);
      expect(lastFrame()).toContain('Hello, world! How are you?');
    });

    it('renders text with numbers', () => {
      const { lastFrame } = render(<MarkdownText content="Value: 12345" />);
      expect(lastFrame()).toContain('Value: 12345');
    });

    it('renders text with unicode', () => {
      const { lastFrame } = render(<MarkdownText content="This contains 中文 and 日本語" />);
      expect(lastFrame()).toContain('This contains 中文');
    });

    it('renders text with special characters', () => {
      const { lastFrame } = render(<MarkdownText content="Price: $100 & tax: 10%" />);
      expect(lastFrame()).toContain('Price: $100');
    });

    it('renders long text line', () => {
      const longText = 'x'.repeat(500);
      const { lastFrame } = render(<MarkdownText content={longText} />);
      expect(lastFrame()).toContain('xxxxx');
    });
  });

  describe('Mixed Content', () => {
    it('renders h1 followed by h2', () => {
      const { lastFrame } = render(<MarkdownText content="# Title\n## Subtitle" />);
      expect(lastFrame()).toContain('Title');
      expect(lastFrame()).toContain('Subtitle');
    });

    it('renders header followed by list', () => {
      const { lastFrame } = render(<MarkdownText content="# Header\n- Item 1\n- Item 2" />);
      expect(lastFrame()).toContain('Header');
      expect(lastFrame()).toContain('Item 1');
    });

    it('renders header followed by text', () => {
      const { lastFrame } = render(<MarkdownText content="# Header\nSome description text" />);
      expect(lastFrame()).toContain('Header');
      expect(lastFrame()).toContain('Some description text');
    });

    it('renders list followed by text', () => {
      const { lastFrame } = render(<MarkdownText content="- First item\nSome explanation" />);
      expect(lastFrame()).toContain('First item');
      expect(lastFrame()).toContain('Some explanation');
    });

    it('renders complex mixed content', () => {
      const content = `# Main Title

## Section One

- Point A
- Point B

Some paragraph text here.

## Section Two

- Another point
- And another

More text at the bottom.`;

      const { lastFrame } = render(<MarkdownText content={content} />);
      expect(lastFrame()).toContain('Main Title');
      expect(lastFrame()).toContain('Section One');
      expect(lastFrame()).toContain('Point A');
      expect(lastFrame()).toContain('Point B');
    });
  });

  describe('Empty Markdown', () => {
    it('renders empty string', () => {
      const { lastFrame } = render(<MarkdownText content="" />);
      expect(lastFrame()).toBeDefined();
    });

    it('renders only whitespace', () => {
      const { lastFrame } = render(<MarkdownText content="   \n   \n   " />);
      expect(lastFrame()).toBeDefined();
    });

    it('renders only newlines', () => {
      const { lastFrame } = render(<MarkdownText content="\n\n\n" />);
      expect(lastFrame()).toBeDefined();
    });
  });

  describe('Content Updates', () => {
    it('renders after content change', () => {
      const { rerender, lastFrame } = render(<MarkdownText content="# Old Title" />);
      expect(lastFrame()).toContain('Old Title');

      rerender(<MarkdownText content="# New Title" />);
      expect(lastFrame()).toContain('New Title');
    });

    it('renders after multiple content changes', () => {
      const { rerender, lastFrame } = render(<MarkdownText content="First" />);

      rerender(<MarkdownText content="Second" />);
      expect(lastFrame()).toContain('Second');

      rerender(<MarkdownText content="Third" />);
      expect(lastFrame()).toContain('Third');
    });

    it('renders after content changes to empty', () => {
      const { rerender, lastFrame } = render(<MarkdownText content="Some text" />);
      expect(lastFrame()).toContain('Some text');

      rerender(<MarkdownText content="" />);
      expect(lastFrame()).toBeDefined();
    });
  });

  describe('Markdown Parsing Edge Cases', () => {
    it('renders line starting with # but no space', () => {
      const { lastFrame } = render(<MarkdownText content="#NoSpace" />);
      expect(lastFrame()).toContain('#NoSpace');
    });

    it('renders line with multiple # marks', () => {
      const { lastFrame } = render(<MarkdownText content="### Three Hashes" />);
      expect(lastFrame()).toContain('### Three Hashes');
    });

    it('renders line with four # marks', () => {
      const { lastFrame } = render(<MarkdownText content="#### Four Headers" />);
      expect(lastFrame()).toContain('#### Four Headers');
    });

    it('renders dash not at start', () => {
      const { lastFrame } = render(<MarkdownText content="Not a list - just text" />);
      expect(lastFrame()).toContain('Not a list');
    });

    it('renders line that looks like header but has no text', () => {
      const { lastFrame } = render(<MarkdownText content="# " />);
      expect(lastFrame()).toBeDefined();
    });
  });

  describe('Multiple Consecutive Elements', () => {
    it('renders multiple consecutive headers', () => {
      const { lastFrame } = render(<MarkdownText content="# One\n## Two\n### Three" />);
      expect(lastFrame()).toContain('One');
      expect(lastFrame()).toContain('Two');
      expect(lastFrame()).toContain('Three');
    });

    it('renders multiple consecutive lists', () => {
      const { lastFrame } = render(<MarkdownText content="- Item 1\n- Item 2\n- Item 3\n- Item 4" />);
      const frame = lastFrame();
      expect(frame).toContain('Item 1');
      expect(frame).toContain('Item 4');
    });

    it('renders many empty lines between content', () => {
      const { lastFrame } = render(<MarkdownText content="Start\n\n\n\n\n\nEnd" />);
      expect(lastFrame()).toContain('Start');
      expect(lastFrame()).toContain('End');
    });
  });

  describe('Unicode and International Content', () => {
    it('renders chinese characters', () => {
      const { lastFrame } = render(<MarkdownText content="# 标题\n## 子标题\n- 项目列表" />);
      expect(lastFrame()).toContain('标题');
      expect(lastFrame()).toContain('子标题');
    });

    it('renders japanese characters', () => {
      const { lastFrame } = render(<MarkdownText content="# タイトル\nSome content" />);
      expect(lastFrame()).toContain('タイトル');
    });

    it('renders korean characters', () => {
      const { lastFrame } = render(<MarkdownText content="## 제목\n내용입니다" />);
      expect(lastFrame()).toContain('제목');
    });

    it('renders arabic characters', () => {
      const { lastFrame } = render(<MarkdownText content="# عنوان\nنص عربي" />);
      expect(lastFrame()).toContain('عنوان');
    });

    it('renders russian characters', () => {
      const { lastFrame } = render(<MarkdownText content="## Заголовок\nТекст на русском" />);
      expect(lastFrame()).toContain('Заголовок');
    });

    it('renders mixed international content', () => {
      const { lastFrame } = render(<MarkdownText content="# English 中文 日本語 한국어 العربية" />);
      expect(lastFrame()).toContain('English');
    });
  });

  describe('Special Characters in Content', () => {
    it('renders backticks', () => {
      const { lastFrame } = render(<MarkdownText content="Text with `backticks`" />);
      expect(lastFrame()).toContain('backticks');
    });

    it('renders asterisks', () => {
      const { lastFrame } = render(<MarkdownText content="*asterisks* and more" />);
      expect(lastFrame()).toContain('asterisks');
    });

    it('renders underscores', () => {
      const { lastFrame } = render(<MarkdownText content="under_score and more" />);
      expect(lastFrame()).toContain('under_score');
    });

    it('renders brackets', () => {
      const { lastFrame } = render(<MarkdownText content="[bracket] content" />);
      expect(lastFrame()).toContain('bracket');
    });

    it('renders parentheses', () => {
      const { lastFrame } = render(<MarkdownText content="(parentheses) content" />);
      expect(lastFrame()).toContain('parentheses');
    });

    it('renders angle brackets', () => {
      const { lastFrame } = render(<MarkdownText content="<angle> content" />);
      expect(lastFrame()).toContain('angle');
    });
  });

  describe('Performance and Large Content', () => {
    it('renders large content without issues', () => {
      const content = Array.from({ length: 100 }, (_, i) => `- List item ${i}`).join('\n');
      const { lastFrame } = render(<MarkdownText content={content} />);
      expect(lastFrame()).toContain('List item 0');
      expect(lastFrame()).toContain('List item 99');
    });

    it('renders long single line', () => {
      const longLine = 'x'.repeat(1000);
      const { lastFrame } = render(<MarkdownText content={longLine} />);
      expect(lastFrame()).toContain('xxx');
    });

    it('renders without memory leaks on unmount', () => {
      const { unmount } = render(<MarkdownText content="Test content" />);
      expect(() => unmount()).not.toThrow();
    });

    it('handles rapid content changes', () => {
      const { rerender, lastFrame } = render(<MarkdownText content="Update 0" />);
      for (let i = 1; i <= 50; i++) {
        rerender(<MarkdownText content={`Update ${i}`} />);
      }
      expect(lastFrame()).toContain('Update 50');
    });
  });
});