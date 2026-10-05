/** @jsxImportSource react */
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import { InstagramDMCard } from './InstagramDMCard';
import React from 'react';

describe('InstagramDMCard', () => {
  it('renders correctly with proposed_action.customer_message and proposed_action.draft_reply', () => {
    const approval = {
      proposed_action: {
        customer_message: 'Test Customer Message',
        draft_reply: 'Test Draft Reply'
      }
    };

    const { container } = render(<InstagramDMCard approval={approval} />);

    expect(screen.getByText('Instagram DM')).toBeInTheDocument();
    expect(screen.getByText('Test Customer Message')).toBeInTheDocument();
    expect(screen.getByText('Test Draft Reply')).toBeInTheDocument();

    const wrapper = container.firstChild as HTMLElement;
    expect(wrapper).toHaveClass('backdrop-blur-[30px]');
    expect(wrapper).toHaveClass('saturate-[210%]');
    expect(wrapper).toHaveClass('bg-[rgba(255,255,255,0.65)]');
  });

  it('renders correctly with context_payload.original_message and context_payload.generated_response', () => {
    const approval = {
      context_payload: {
        original_message: 'Test Original Message',
        generated_response: 'Test Generated Response'
      }
    };

    render(<InstagramDMCard approval={approval} />);

    expect(screen.getByText('Test Original Message')).toBeInTheDocument();
    expect(screen.getByText('Test Generated Response')).toBeInTheDocument();
  });

  it('renders correctly with context_payload.description', () => {
    const approval = {
      context_payload: {
        description: 'Test Description Message',
        draft_reply: 'Test Draft Reply 2'
      }
    };

    render(<InstagramDMCard approval={approval} />);

    expect(screen.getByText('Test Description Message')).toBeInTheDocument();
  });

  it('handles approve action', () => {
    const onApprove = vi.fn();
    const approval = { proposed_action: { customer_message: 'msg', draft_reply: 'reply' } };

    render(<InstagramDMCard approval={approval} onApprove={onApprove} />);

    const btn = screen.getByTestId('approve-instagram-dm');
    fireEvent.click(btn);

    expect(onApprove).toHaveBeenCalledTimes(1);
  });

  it('handles dismiss action', () => {
    const onDismiss = vi.fn();
    const approval = { proposed_action: { customer_message: 'msg', draft_reply: 'reply' } };

    render(<InstagramDMCard approval={approval} onDismiss={onDismiss} />);

    const btn = screen.getByTestId('dismiss-instagram-dm');
    fireEvent.click(btn);

    expect(onDismiss).toHaveBeenCalledTimes(1);
  });
});
