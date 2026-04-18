import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import BusinessSetupWizard from '../../src/components/wizard/BusinessSetupWizard';
import '@testing-library/jest-dom';

global.fetch = jest.fn((url) => {
  if (url === '/api/wizard/state') {
    return Promise.resolve({
      ok: true,
      json: () => Promise.resolve({ step: 1 }),
    });
  }
  return Promise.resolve({
    ok: true,
    json: () => Promise.resolve({}),
  });
}) as jest.Mock;

test('renders welcome screen and navigates through wizard', async () => {
  render(<BusinessSetupWizard />);
  expect(await screen.findByText('Your AI team, ready in minutes')).toBeInTheDocument();

  fireEvent.click(screen.getByText('Next'));
  expect(await screen.findByText('Business Profile')).toBeInTheDocument();

  fireEvent.click(screen.getByText('Next'));
  expect(await screen.findByText('Goal Selection')).toBeInTheDocument();

  fireEvent.click(screen.getByText('Next'));
  expect(await screen.findByText('Deployment Preference')).toBeInTheDocument();

  fireEvent.click(screen.getByText('Next'));
  expect(await screen.findByText('Administrator Account')).toBeInTheDocument();

  // Test password strength indicator
  const passwordInput = screen.getByPlaceholderText('Password');
  fireEvent.change(passwordInput, { target: { value: 'short' } });
  expect(screen.getByText(/Strength: Weak/)).toBeInTheDocument();
  fireEvent.change(passwordInput, { target: { value: 'longerpass' } });
  expect(screen.getByText(/Strength: Strong/)).toBeInTheDocument();

  fireEvent.click(screen.getByText('Next'));
  expect(await screen.findByText('Review & Launch')).toBeInTheDocument();

  global.alert = jest.fn();
  const launchButton = screen.getByRole('button', { name: /Launch My AI Team/i });
  fireEvent.click(launchButton);

  await waitFor(() => {
    expect(global.fetch).toHaveBeenCalledWith('/api/provision', expect.any(Object));
  });

  expect(await screen.findByTestId('progress-overlay')).toBeInTheDocument();
});
