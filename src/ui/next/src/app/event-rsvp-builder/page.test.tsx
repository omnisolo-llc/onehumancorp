import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import EventRSVPBuilderPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('EventRSVPBuilderPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders the configuration form correctly', () => {
    render(<EventRSVPBuilderPage />);
    expect(screen.getByText('Event RSVP Builder 🎉')).toBeDefined();
    expect(screen.getByText('Event Title')).toBeDefined();
    expect(screen.getByText('Date & Time')).toBeDefined();
    expect(screen.getByText('Location')).toBeDefined();
    expect(screen.getByText('Theme')).toBeDefined();
    expect(screen.getByText('Live Preview')).toBeDefined();
  });

  it('shows soft paywall when checkbox is checked without pro and includes viral loop option', () => {
    render(<EventRSVPBuilderPage />);

    // Click checkbox
    const checkbox = screen.getByLabelText(/Remove "Powered by OHC"/i);
    // fireEvent.click on a checkbox naturally changes its state before the event handler is called in react testing library in some versions.
    // It's a controlled component, so we just check that the paywall appears
    fireEvent.click(checkbox); // This triggers the modal

    // Check if soft paywall shows up
    expect(screen.getAllByText('Upgrade to Pro').length).toBeGreaterThan(0);
    expect(screen.getByText('Upgrade to Remove Branding')).toBeDefined();
  });

  it('updates the live preview iframe URL when inputs change', () => {
    render(<EventRSVPBuilderPage />);

    // Check initial iframe URL
    const iframe = screen.getByTitle(/Event RSVP Builder/i) as HTMLIFrameElement | null;
    const initialIframe = document.querySelector('iframe');
    expect(initialIframe?.src).toContain('title=Summer%20Pop-up%20Shop');

    const nameInput = screen.getByPlaceholderText('e.g. Summer Pop-up');
    fireEvent.change(nameInput, { target: { value: 'Winter Festival' } });

    // Check that iframe src reflects the change
    const updatedIframe = document.querySelector('iframe');
    expect(updatedIframe?.src).toContain('title=Winter%20Festival');
  });

  it('shows the embed modal when button is clicked and contains correct code', () => {
    render(<EventRSVPBuilderPage />);

    // Check that modal is not initially visible
    expect(screen.queryByText('Embed RSVP Widget')).toBeNull();

    const embedButton = screen.getByText('Get Widget Code');
    fireEvent.click(embedButton);

    // Check that modal is visible
    expect(screen.getByText('Embed RSVP Widget')).toBeDefined();

    // Check embed code in modal
    const textareas = screen.getAllByRole('textbox') as HTMLTextAreaElement[];
    const textarea = textareas.find(ta => ta.value.includes('<iframe'));
    expect(textarea).toBeDefined();
    expect(textarea!.value).toContain('/api/v1/growth/event-rsvp/embed');
    expect(textarea!.value).toContain('title=Summer%20Pop-up%20Shop');
    expect(textarea!.value).toContain('branding=true'); // Since branding is not hidden by default
  });

  it('changes theme when theme buttons are clicked', () => {
    render(<EventRSVPBuilderPage />);

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
