import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import { LoadingScreen } from '../LoadingScreen';
import { describe, it, expect } from 'vitest';

describe('LoadingScreen', () => {
  it('renders building message', () => {
    render( <LoadingScreen /> );
    expect(screen.getByText(/Building Your Future/i)).toBeInTheDocument();
  });

  it('shows cloud ready status', () => {
    render( <LoadingScreen /> );
    expect(screen.getByText(/Cloud Ready/i)).toBeInTheDocument();
  });
});
