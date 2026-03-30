import { describe, test, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { App } from './App';
import React from 'react';

describe('App', () => {
  test('renders heading', () => {
    render(<App />);
    expect(screen.getByText('One Human Corp')).toBeTruthy();
  });
});
