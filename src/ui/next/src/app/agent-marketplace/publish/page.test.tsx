import '@testing-library/jest-dom';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { fireEvent, render, screen } from '@testing-library/react';
import PublishAgentPage from './page';
const mockFetch = vi.fn();
const values = { 'Agent Name': 'Test Agent', Description: 'Test description', Role: 'Writer', 'System Prompt': 'Private instructions' };
afterEach(() => vi.unstubAllGlobals());
describe('Publish Agent Page capability boundary', () => {
  beforeEach(() => { mockFetch.mockReset(); vi.stubGlobal('fetch', mockFetch); });
  it('renders the existing editable form and explains unavailable publication', () => {
    render(<PublishAgentPage />);
    expect(screen.getByText('Publish New Agent')).toBeVisible();
    expect(screen.getByText('Prepare agent details in this unsaved form.')).toBeVisible();
    for (const name of Object.keys(values)) expect(screen.getByLabelText(name)).toBeVisible();
    expect(screen.getByRole('button', { name: 'Publish to Marketplace' })).toBeDisabled();
    expect(screen.getByRole('status')).toHaveTextContent('Full-agent publication is unavailable');
  });
  it('retains every entered field when an unsupported submission is attempted', () => {
    render(<PublishAgentPage />);
    for (const [name, value] of Object.entries(values)) fireEvent.change(screen.getByLabelText(name), { target: { value } });
    fireEvent.submit(screen.getByRole('button', { name: 'Publish to Marketplace' }).closest('form')!);
    for (const [name, value] of Object.entries(values)) expect(screen.getByLabelText(name)).toHaveValue(value);
    expect(mockFetch).not.toHaveBeenCalled();
  });
  it('prevents default form navigation even for programmatic submission', () => {
    render(<PublishAgentPage />);
    const submit = new Event('submit', { bubbles: true, cancelable: true });
    screen.getByRole('button', { name: 'Publish to Marketplace' }).closest('form')!.dispatchEvent(submit);
    expect(submit.defaultPrevented).toBe(true);
    expect(mockFetch).not.toHaveBeenCalled();
  });
  it('does not send private form edits or create publication state during rendering', () => {
    render(<PublishAgentPage />);
    fireEvent.change(screen.getByLabelText('System Prompt'), { target: { value: 'Unsubmitted private draft' } });
    expect(mockFetch).not.toHaveBeenCalled();
    expect(screen.queryByText(/Publishing\.\.\./)).toBeNull();
    expect(screen.getByRole('status')).toHaveTextContent('kept only in this open form');
  });
});
