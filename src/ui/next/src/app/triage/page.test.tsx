import React from 'react';
import { render, screen } from '@testing-library/react';
import { beforeEach, expect, test, vi } from 'vitest';
import TriagePage from './page';

vi.mock('@/app/components/AppShell', () => ({
  AppShell: ({ children }: { children: React.ReactNode }) => <div>{children}</div>,
}));

test('renders triage items correctly', () => {
  render(<TriagePage />);
});
