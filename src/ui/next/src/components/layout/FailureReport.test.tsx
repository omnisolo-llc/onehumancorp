import React from 'react';
import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import { FailureReport } from './FailureReport';
import '@testing-library/jest-dom';

describe('FailureReport', () => {
  it('renders title and message correctly', () => {
    render(<FailureReport title="System Error" message="Something went wrong." />);
    expect(screen.getByText('System Error')).toBeInTheDocument();
    expect(screen.getByText('Something went wrong.')).toBeInTheDocument();
  });

  it('renders without a title', () => {
    render(<FailureReport message="Just a message." />);
    expect(screen.getByText('Just a message.')).toBeInTheDocument();
    expect(screen.queryByRole('heading', { level: 3 })).not.toBeInTheDocument();
  });

  it('renders error rate data correctly', () => {
    const errorRateData = [
      { time: '10:00', error_rate: 5 },
      { time: '10:05', error_rate: 10 }
    ];
    render(<FailureReport message="Data present" errorRateData={errorRateData} />);

    expect(screen.getByText('Error Rate Over Time')).toBeInTheDocument();
    expect(screen.getByText('10:00')).toBeInTheDocument();
    expect(screen.getByText('5%')).toBeInTheDocument();
    expect(screen.getByText('10:05')).toBeInTheDocument();
    expect(screen.getByText('10%')).toBeInTheDocument();
  });

  it('renders latency data correctly', () => {
    const latencyData = [
      { bucket: '0-100ms', count: 50 },
      { bucket: '100-200ms', count: 20 }
    ];
    render(<FailureReport message="Data present" latencyData={latencyData} />);

    expect(screen.getByText('Latency Histogram')).toBeInTheDocument();
    expect(screen.getByText('0-100ms')).toBeInTheDocument();
    expect(screen.getByText('100-200ms')).toBeInTheDocument();
  });

  it('contains proper translucent classes', () => {
    const { container } = render(<FailureReport message="Check styles" />);
    const divElement = container.firstChild as HTMLElement;
    expect(divElement).toHaveClass('backdrop-blur-[30px]');
    expect(divElement).toHaveClass('backdrop-saturate-[210%]');
    expect(divElement).toHaveClass('bg-[rgba(255,255,255,0.65)]');
    expect(divElement).toHaveClass('dark:bg-[rgba(22,22,26,0.7)]');
    expect(divElement).toHaveClass('border');
    expect(divElement).toHaveClass('border-[rgba(255,255,255,0.4)]');
    expect(divElement).toHaveClass('dark:border-[rgba(255,255,255,0.1)]');
  });
});
