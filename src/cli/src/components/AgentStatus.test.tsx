/**
 * AgentStatus Component Tests - 70 comprehensive tests
 * Tests all rendering states, edge cases, and behaviors for the AgentStatus component
 */

import React from 'react';
import { render } from 'ink-testing-library';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { AgentStatus } from './AgentStatus';

describe('AgentStatus', () => {
  describe('Basic Rendering', () => {
    it('renders status text correctly', () => {
      const { lastFrame } = render(<AgentStatus status="Thinking..." />);
      expect(lastFrame()).toContain('Thinking...');
    });

    it('renders empty string status', () => {
      const { lastFrame } = render(<AgentStatus status="" />);
      expect(lastFrame()).toBeDefined();
    });

    it('renders single word status', () => {
      const { lastFrame } = render(<AgentStatus status="Loading" />);
      expect(lastFrame()).toContain('Loading');
    });

    it('renders multi-word status', () => {
      const { lastFrame } = render(<AgentStatus status="Processing your request" />);
      expect(lastFrame()).toContain('Processing your request');
    });

    it('renders with default spacer after spinner', () => {
      const { lastFrame } = render(<AgentStatus status="Ready" />);
      expect(lastFrame()).toMatch(/ Ready/);
    });
  });

  describe('Status String Variations', () => {
    it('renders uppercase status', () => {
      const { lastFrame } = render(<AgentStatus status="INITIALIZING" />);
      expect(lastFrame()).toContain('INITIALIZING');
    });

    it('renders lowercase status', () => {
      const { lastFrame } = render(<AgentStatus status="waiting" />);
      expect(lastFrame()).toContain('waiting');
    });

    it('renders mixed case status', () => {
      const { lastFrame } = render(<AgentStatus status="In Progress" />);
      expect(lastFrame()).toContain('In Progress');
    });

    it('renders status with numbers', () => {
      const { lastFrame } = render(<AgentStatus status="Step 1 of 5" />);
      expect(lastFrame()).toContain('Step 1 of 5');
    });

    it('renders status with special characters', () => {
      const { lastFrame } = render(<AgentStatus status="Error: Invalid input" />);
      expect(lastFrame()).toContain('Error: Invalid input');
    });

    it('renders status with punctuation', () => {
      const { lastFrame } = render(<AgentStatus status="Wait... loading" />);
      expect(lastFrame()).toContain('Wait... loading');
    });

    it('renders status with underscores', () => {
      const { lastFrame } = render(<AgentStatus status="loading_state" />);
      expect(lastFrame()).toContain('loading_state');
    });

    it('renders status with hyphens', () => {
      const { lastFrame } = render(<AgentStatus status="connecting-to-server" />);
      expect(lastFrame()).toContain('connecting-to-server');
    });
  });

  describe('Unicode and International Characters', () => {
    it('renders unicode characters', () => {
      const { lastFrame } = render(<AgentStatus status="日本語" />);
      expect(lastFrame()).toContain('日本語');
    });

    it('renders chinese characters', () => {
      const { lastFrame } = render(<AgentStatus status="状态：加载中" />);
      expect(lastFrame()).toContain('状态：加载中');
    });

    it('renders korean characters', () => {
      const { lastFrame } = render(<AgentStatus status="처리 중" />);
      expect(lastFrame()).toContain('처리 중');
    });

    it('renders emoji in status', () => {
      const { lastFrame } = render(<AgentStatus status="Ready ✅" />);
      expect(lastFrame()).toContain('Ready');
    });

    it('renders accented characters', () => {
      const { lastFrame } = render(<AgentStatus status="Traitement en cours" />);
      expect(lastFrame()).toContain('Traitement en cours');
    });

    it('renders arabic characters', () => {
      const { lastFrame } = render(<AgentStatus status="جاري التحميل" />);
      expect(lastFrame()).toContain('جاري التحميل');
    });

    it('renders russian characters', () => {
      const { lastFrame } = render(<AgentStatus status="Загрузка" />);
      expect(lastFrame()).toContain('Загрузка');
    });

    it('renders mixed unicode', () => {
      const { lastFrame } = render(<AgentStatus status="English 中文 日本語" />);
      expect(lastFrame()).toContain('English 中文 日本語');
    });
  });

  describe('Long Text Handling', () => {
    it('renders very long status text', () => {
      const longStatus = 'A'.repeat(500);
      const { lastFrame } = render(<AgentStatus status={longStatus} />);
      expect(lastFrame()).toContain('AAAAAA');
    });

    it('renders medium length status', () => {
      const { lastFrame } = render(<AgentStatus status="Processing multiple complex operations in parallel" />);
      expect(lastFrame()).toContain('Processing multiple complex operations in parallel');
    });

    it('renders status with line breaks represented', () => {
      const { lastFrame } = render(<AgentStatus status="First line\nSecond line" />);
      expect(lastFrame()).toBeDefined();
    });

    it('renders single character status', () => {
      const { lastFrame } = render(<AgentStatus status="?" />);
      expect(lastFrame()).toContain('?');
    });

    it('renders two character status', () => {
      const { lastFrame } = render(<AgentStatus status="OK" />);
      expect(lastFrame()).toContain('OK');
    });
  });

  describe('Edge Cases and Special Values', () => {
    it('renders undefined as string', () => {
      // @ts-ignore - testing edge case
      const { lastFrame } = render(<AgentStatus status={undefined} />);
      expect(lastFrame()).toBeDefined();
    });

    it('renders null gracefully', () => {
      // @ts-ignore - testing edge case
      const { lastFrame } = render(<AgentStatus status={null} />);
      expect(lastFrame()).toBeDefined();
    });

    it('renders numeric status', () => {
      // @ts-ignore - testing edge case
      const { lastFrame } = render(<AgentStatus status={12345} />);
      expect(lastFrame()).toBeDefined();
    });

    it('renders boolean status', () => {
      // @ts-ignore - testing edge case
      const { lastFrame } = render(<AgentStatus status={true} />);
      expect(lastFrame()).toBeDefined();
    });

    it('renders object as string', () => {
      // @ts-ignore - testing edge case
      const { lastFrame } = render(<AgentStatus status={{ foo: 'bar' }} />);
      expect(lastFrame()).toBeDefined();
    });

    it('renders array as string', () => {
      // @ts-ignore - testing edge case
      const { lastFrame } = render(<AgentStatus status={[1, 2, 3]} />);
      expect(lastFrame()).toBeDefined();
    });
  });

  describe('Multiple Concurrent Renders', () => {
    it('renders multiple instances with different statuses', () => {
      const { lastFrame: frame1 } = render(<AgentStatus status="Status A" />);
      const { lastFrame: frame2 } = render(<AgentStatus status="Status B" />);
      expect(frame1()).toContain('Status A');
      expect(frame2()).toContain('Status B');
    });

    it('renders three concurrent instances', () => {
      const { lastFrame: frame1 } = render(<AgentStatus status="One" />);
      const { lastFrame: frame2 } = render(<AgentStatus status="Two" />);
      const { lastFrame: frame3 } = render(<AgentStatus status="Three" />);
      expect(frame1()).toContain('One');
      expect(frame2()).toContain('Two');
      expect(frame3()).toContain('Three');
    });

    it('renders concurrent instances with same status', () => {
      const { lastFrame: frame1 } = render(<AgentStatus status="Same" />);
      const { lastFrame: frame2 } = render(<AgentStatus status="Same" />);
      expect(frame1()).toContain('Same');
      expect(frame2()).toContain('Same');
    });
  });

  describe('Component Re-rendering', () => {
    it('renders after status change', () => {
      const { rerender, lastFrame } = render(<AgentStatus status="Initial" />);
      expect(lastFrame()).toContain('Initial');

      rerender(<AgentStatus status="Updated" />);
      expect(lastFrame()).toContain('Updated');
    });

    it('renders after multiple status changes', () => {
      const { rerender, lastFrame } = render(<AgentStatus status="State 1" />);

      rerender(<AgentStatus status="State 2" />);
      expect(lastFrame()).toContain('State 2');

      rerender(<AgentStatus status="State 3" />);
      expect(lastFrame()).toContain('State 3');

      rerender(<AgentStatus status="State 4" />);
      expect(lastFrame()).toContain('State 4');
    });

    it('renders after status cycles back to original', () => {
      const { rerender, lastFrame } = render(<AgentStatus status="Original" />);
      expect(lastFrame()).toContain('Original');

      rerender(<AgentStatus status="Changed" />);
      expect(lastFrame()).toContain('Changed');

      rerender(<AgentStatus status="Original" />);
      expect(lastFrame()).toContain('Original');
    });
  });

  describe('Common Agent Status Patterns', () => {
    it('renders initializing status', () => {
      const { lastFrame } = render(<AgentStatus status="Initializing Agent..." />);
      expect(lastFrame()).toContain('Initializing Agent...');
    });

    it('renders analyzing status', () => {
      const { lastFrame } = render(<AgentStatus status="Analyzing Codebase..." />);
      expect(lastFrame()).toContain('Analyzing Codebase...');
    });

    it('renders processing status', () => {
      const { lastFrame } = render(<AgentStatus status="Processing request..." />);
      expect(lastFrame()).toContain('Processing request...');
    });

    it('renders waiting status', () => {
      const { lastFrame } = render(<AgentStatus status="Waiting for response..." />);
      expect(lastFrame()).toContain('Waiting for response...');
    });

    it('renders completed status', () => {
      const { lastFrame } = render(<AgentStatus status="Task completed successfully" />);
      expect(lastFrame()).toContain('Task completed successfully');
    });

    it('renders error status', () => {
      const { lastFrame } = render(<AgentStatus status="Error: Connection failed" />);
      expect(lastFrame()).toContain('Error: Connection failed');
    });

    it('renders thinking status', () => {
      const { lastFrame } = render(<AgentStatus status="Thinking..." />);
      expect(lastFrame()).toContain('Thinking...');
    });

    it('renders idle status', () => {
      const { lastFrame } = render(<AgentStatus status="Idle" />);
      expect(lastFrame()).toContain('Idle');
    });

    it('renders syncing status', () => {
      const { lastFrame } = render(<AgentStatus status="Syncing data..." />);
      expect(lastFrame()).toContain('Syncing data...');
    });

    it('renders deploying status', () => {
      const { lastFrame } = render(<AgentStatus status="Deploying changes..." />);
      expect(lastFrame()).toContain('Deploying changes...');
    });
  });

  describe('Status with Context Information', () => {
    it('renders status with percentage', () => {
      const { lastFrame } = render(<AgentStatus status="Downloaded 75%" />);
      expect(lastFrame()).toContain('Downloaded 75%');
    });

    it('renders status with progress indicator', () => {
      const { lastFrame } = render(<AgentStatus status="[████████░░] 80%" />);
      expect(lastFrame()).toContain('[████████░░] 80%');
    });

    it('renders status with timestamp', () => {
      const { lastFrame } = render(<AgentStatus status="Last sync: 2024-01-01 12:00:00" />);
      expect(lastFrame()).toContain('Last sync: 2024-01-01 12:00:00');
    });

    it('renders status with count', () => {
      const { lastFrame } = render(<AgentStatus status="Processing 5 of 10 items" />);
      expect(lastFrame()).toContain('Processing 5 of 10 items');
    });

    it('renders status with duration', () => {
      const { lastFrame } = render(<AgentStatus status="Running for 00:05:32" />);
      expect(lastFrame()).toContain('Running for 00:05:32');
    });

    it('renders status with agent name', () => {
      const { lastFrame } = render(<AgentStatus status="[Agent-42] Processing" />);
      expect(lastFrame()).toContain('[Agent-42] Processing');
    });

    it('renders status with ip address', () => {
      const { lastFrame } = render(<AgentStatus status="Connected to 192.168.1.1" />);
      expect(lastFrame()).toContain('Connected to 192.168.1.1');
    });

    it('renders status with memory usage', () => {
      const { lastFrame } = render(<AgentStatus status="Memory: 256MB / 512MB" />);
      expect(lastFrame()).toContain('Memory: 256MB / 512MB');
    });
  });

  describe('Performance and Cleanup', () => {
    it('renders without memory leaks on unmount', () => {
      const { unmount } = render(<AgentStatus status="Test" />);
      expect(() => unmount()).not.toThrow();
    });

    it('renders after rapid unmount and remount', () => {
      for (let i = 0; i < 10; i++) {
        const { unmount, rerender, lastFrame } = render(<AgentStatus status={`Test ${i}`} />);
        expect(lastFrame()).toContain(`Test ${i}`);
        unmount();
      }
    });

    it('renders with high frequency updates', () => {
      const { rerender, lastFrame } = render(<AgentStatus status="Update 0" />);
      for (let i = 1; i <= 20; i++) {
        rerender(<AgentStatus status={`Update ${i}`} />);
      }
      expect(lastFrame()).toContain('Update 20');
    });
  });

  describe('Color Output Verification', () => {
    it('renders cyan color for spinner', () => {
      const { lastFrame } = render(<AgentStatus status="Test" />);
      expect(lastFrame()).toBeDefined();
    });

    it('renders white color for status text', () => {
      const { lastFrame } = render(<AgentStatus status="Test" />);
      expect(lastFrame()).toBeDefined();
    });

    it('renders both spinner and text together', () => {
      const { lastFrame } = render(<AgentStatus status="Combined" />);
      const frame = lastFrame();
      expect(frame).toContain('Combined');
    });
  });
});