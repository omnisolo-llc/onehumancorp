import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import WorkIntakeWidgetPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('WorkIntakeWidgetPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders the page and initial state correctly', () => {
    render(<WorkIntakeWidgetPage />);
    expect(screen.getByText('Work-Intake Widget 📋')).toBeDefined();
    expect(screen.getByText('Get Widget Code')).toBeDefined();
  });

  it('updates embed code when inputs change', () => {
    render(<WorkIntakeWidgetPage />);

    // Change Tenant ID
    const tenantInput = screen.getByDisplayValue('my-business');
    fireEvent.change(tenantInput, { target: { value: 'test-tenant' } });

    // Change Form Title
    const titleInput = screen.getByDisplayValue('Work Request');
    fireEvent.change(titleInput, { target: { value: 'Custom Request' } });

    // Open Modal
    fireEvent.click(screen.getByText('Get Widget Code'));

    // Verify textarea content (it may find multiple if there are other textboxes, so use getAllByRole)
    const textareas = screen.getAllByRole('textbox') as HTMLTextAreaElement[];
    // Find the textarea that contains the iframe code
    const textarea = textareas.find(ta => ta.value.includes('<iframe'))!;
    expect(textarea).toBeDefined();
    expect(textarea.value).toContain('tenant=test-tenant');
    expect(textarea.value).toContain('title=Custom%20Request');
    expect(textarea.value).toContain('Powered by OHC');
  });

  it('shows soft paywall when checkbox is checked', () => {
    render(<WorkIntakeWidgetPage />);

    // Click checkbox
    const checkbox = screen.getByLabelText('Remove "Powered by OHC" branding');
    fireEvent.click(checkbox);

    // Check if soft paywall shows up
    expect(screen.getAllByText('Upgrade to Pro')).toBeDefined();

    // Checkbox should be un-checked
    expect((checkbox as HTMLInputElement).checked).toBe(false);
  });

  it('changes theme when theme buttons are clicked', () => {
    render(<WorkIntakeWidgetPage />);

    const darkButton = screen.getByText('Dark');
    fireEvent.click(darkButton);

    // Open Modal to check the generated URL
    fireEvent.click(screen.getByText('Get Widget Code'));

    const textareas = screen.getAllByRole('textbox') as HTMLTextAreaElement[];
    const textarea = textareas.find(ta => ta.value.includes('<iframe'))!;
    expect(textarea).toBeDefined();
    expect(textarea.value).toContain('theme=dark');
  });
});
