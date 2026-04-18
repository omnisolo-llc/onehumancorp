import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import BusinessSetupWizard from '../../src/components/wizard/BusinessSetupWizard';
import '@testing-library/jest-dom';

global.fetch = jest.fn(() =>
  Promise.resolve({
    ok: true,
    json: () => Promise.resolve({}),
  })
) as jest.Mock;

test('renders welcome screen and navigates through wizard', async () => {
  render(<BusinessSetupWizard />);
  expect(screen.getByText('Your AI team, ready in minutes')).toBeInTheDocument();

  fireEvent.click(screen.getByText('Next'));
  expect(screen.getByText('Business Profile')).toBeInTheDocument();

  fireEvent.click(screen.getByText('Next'));
  expect(screen.getByText('Goal Selection')).toBeInTheDocument();

  fireEvent.click(screen.getByText('Next'));
  expect(screen.getByText('Deployment Preference')).toBeInTheDocument();

  fireEvent.click(screen.getByText('Next'));
  expect(screen.getByText('Administrator Account')).toBeInTheDocument();

  fireEvent.click(screen.getByText('Next'));
  expect(screen.getByText('Review & Launch')).toBeInTheDocument();

  global.alert = jest.fn();
  const launchButton = screen.getByRole('button', { name: /Launch My AI Team/i });
  fireEvent.click(launchButton);

  await waitFor(() => {
    expect(global.fetch).toHaveBeenCalledWith('/api/provision', expect.any(Object));
  });
});
