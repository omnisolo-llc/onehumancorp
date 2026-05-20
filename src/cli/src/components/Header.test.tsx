import React from 'react';
import { render } from 'ink-testing-library';
import { expect, test } from 'vitest';
import { Header } from './Header';

test('renders Header correctly', () => {
  const { lastFrame } = render(<Header />);
  expect(lastFrame()).toMatch(/ONE HUMAN CORP/);
  expect(lastFrame()).toMatch(/- Standalone Agent Mode/);
});
