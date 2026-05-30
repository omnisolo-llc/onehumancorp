import React from 'react';
import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import DepartmentCard from './DepartmentCard';
import { TooltipProvider } from '../../../components/TooltipRegistry';

describe('DepartmentCard', () => {
  it('renders safely when name is empty', () => {
    render(
      <TooltipProvider>
        <DepartmentCard name="" pendingCount={0} onClick={() => {}} />
      </TooltipProvider>
    );
    expect(screen.getByText('Active and running')).toBeInTheDocument();
  });
});
