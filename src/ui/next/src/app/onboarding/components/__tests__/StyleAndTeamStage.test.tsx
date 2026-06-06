import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import { StyleAndTeamStage } from '../StyleAndTeamStage';
import { useOnboardingStore } from '../../store';
import { TooltipProvider } from '../../../../components/TooltipRegistry';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import userEvent from '@testing-library/user-event';

describe('StyleAndTeamStage', () => {
  const renderComponent = () => render(
    <TooltipProvider>
      <StyleAndTeamStage onLaunch={vi.fn()} onSaveDraft={vi.fn()} />
    </TooltipProvider>
  );

  beforeEach(() => {
    useOnboardingStore.setState({
      websiteTemplate: 'Modern',
      domainChoice: 'subdomain',
      adminName: '',
      adminEmail: '',
      adminPassword: '',
      aiAgents: ['Sales Agent'],
      aiAutoRespond: true,
    });
  });

  it('renders template choices', () => {
    renderComponent();
    expect(screen.getByText('Modern')).toBeInTheDocument();
    expect(screen.getByText('Minimal')).toBeInTheDocument();
  });

  it('shows error when launching with empty fields', async () => {
    const user = userEvent.setup();
    renderComponent();
    const launchBtn = screen.getByRole('button', { name: /Launch My Business/i });
    await user.click(launchBtn);
    expect(screen.getByText(/Admin Name is required/i)).toBeInTheDocument();
  });
});
