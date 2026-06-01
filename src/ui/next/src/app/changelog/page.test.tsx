import React from 'react';
import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import ChangelogPage from './page';

describe('ChangelogPage', () => {
  it('renders the changelog title and sections', () => {
    render(<ChangelogPage />);

    expect(screen.getByText('Release Notes & Changelog')).toBeInTheDocument();
    expect(screen.getByText('Version 1.0 (Latest)')).toBeInTheDocument();
    expect(screen.getByText('🌟 New Features')).toBeInTheDocument();
    expect(screen.getByText('🛠️ Improvements')).toBeInTheDocument();
    expect(screen.getByText(/\*\*Interactive AI Store Builder:\*\*/)).toBeInTheDocument();
    expect(screen.getByText(/\*\*Smart Tooltips:\*\*/)).toBeInTheDocument();
  });
});
