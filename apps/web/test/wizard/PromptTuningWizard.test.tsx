import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import PromptTuningWizard from '../../src/components/wizard/PromptTuningWizard';
import '@testing-library/jest-dom';

global.fetch = jest.fn(() =>
  Promise.resolve({
    ok: true,
    json: () => Promise.resolve({}),
  })
) as jest.Mock;

test('renders prompt tuning wizard and steps through', async () => {
  render(<PromptTuningWizard />);
  expect(screen.getByText('Personality & Tone')).toBeInTheDocument();

  fireEvent.click(screen.getByText('Next'));
  expect(screen.getByText('Domain Focus')).toBeInTheDocument();

  fireEvent.click(screen.getByText('Next'));
  expect(screen.getByText('Example Interactions')).toBeInTheDocument();

  fireEvent.click(screen.getByText('Next'));
  expect(screen.getByText('Generated Prompt')).toBeInTheDocument();

  global.alert = jest.fn();
  const saveButton = screen.getByRole('button', { name: /Save Agent/i });
  fireEvent.click(saveButton);

  await waitFor(() => {
    expect(global.fetch).toHaveBeenCalledWith('/api/agents/tune', expect.any(Object));
  });
});
