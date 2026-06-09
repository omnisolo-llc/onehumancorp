import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import WhatsAppLinkGeneratorPage from './page';

describe('WhatsAppLinkGeneratorPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders the WhatsApp Link Generator page correctly', () => {
    render(<WhatsAppLinkGeneratorPage />);
    expect(screen.getByText('WhatsApp Link Generator 📱')).toBeDefined();
    expect(screen.getByLabelText(/WhatsApp Phone Number/i)).toBeDefined();
    expect(screen.getByLabelText(/Pre-filled Message/i)).toBeDefined();
    expect(screen.getByRole('button', { name: 'Get Link' })).toBeDefined();
    expect(screen.getByText('Live Preview')).toBeDefined();
  });

  it('disables "Get Link" button when phone number is empty', () => {
    render(<WhatsAppLinkGeneratorPage />);
    const getLinkBtn = screen.getByRole('button', { name: 'Get Link' });
    expect((getLinkBtn as HTMLButtonElement).disabled).toBe(true);
  });

  it('enables "Get Link" button when phone number is provided', () => {
    render(<WhatsAppLinkGeneratorPage />);
    const phoneInput = screen.getByLabelText(/WhatsApp Phone Number/i);
    fireEvent.change(phoneInput, { target: { value: '1234567890' } });

    const getLinkBtn = screen.getByRole('button', { name: 'Get Link' });
    expect((getLinkBtn as HTMLButtonElement).disabled).toBe(false);
  });

  it('generates correct link in modal', () => {
    render(<WhatsAppLinkGeneratorPage />);

    const phoneInput = screen.getByLabelText(/WhatsApp Phone Number/i);
    fireEvent.change(phoneInput, { target: { value: '1234567890' } });

    const msgInput = screen.getByLabelText(/Pre-filled Message/i);
    fireEvent.change(msgInput, { target: { value: 'Hello' } });

    const getLinkBtn = screen.getByRole('button', { name: 'Get Link' });
    fireEvent.click(getLinkBtn);

    expect(screen.getByText('Your WhatsApp Link')).toBeDefined();

    const textareas = screen.getAllByRole('textbox');
    // Find the textarea that isn't the main message input
    const linkTextarea = textareas.find(ta => (ta as HTMLTextAreaElement).value.includes('wa.me')) as HTMLTextAreaElement;
    expect(linkTextarea).toBeDefined();
    expect(linkTextarea.value).toContain('https://wa.me/1234567890?text=');
    expect(linkTextarea.value).toContain(encodeURIComponent('Hello'));
    expect(linkTextarea.value).toContain(encodeURIComponent('\n\n⚡ Powered by OHC'));
  });

  it('shows paywall when trying to remove branding', () => {
    render(<WhatsAppLinkGeneratorPage />);

    const toggle = screen.getByRole('checkbox', { name: /Remove "Powered by OHC" Badge \(Pro\)/i });
    fireEvent.click(toggle);

    expect(screen.getByText('Upgrade to Pro')).toBeDefined();
    expect(screen.getByText(/Make your links 100% yours/i)).toBeDefined();
  });

  it('renders Powered by OHC footer', () => {
    render(<WhatsAppLinkGeneratorPage />);
    const footerLinks = screen.getAllByText(/Powered by OHC/i);
    expect(footerLinks.length).toBeGreaterThan(0);
  });
});
