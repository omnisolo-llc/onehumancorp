import { describe, it, expect, vi } from "vitest";
import React from "react";

/**
 * @jest-environment jsdom
 */

import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import VisualWorkflowPage from './page';

describe('Block-Based Visual Workflow Page', () => {
  it('renders the workflow header', async () => {
    render(<VisualWorkflowPage />);
    expect(screen.getByText('Block-Based Visual Workflow')).toBeInTheDocument();
    expect(screen.getByText(/drag-and-drop node interface/)).toBeInTheDocument();
  });

  it('renders initial nodes', () => {
    render(<VisualWorkflowPage />);
    expect(screen.getByText('Start')).toBeInTheDocument();
    expect(screen.getByText('Process Text')).toBeInTheDocument();
    expect(screen.getByText('End')).toBeInTheDocument();
  });

  it('adds new nodes when buttons are clicked', () => {
    render(<VisualWorkflowPage />);

    fireEvent.click(screen.getByText('+ Add LLM Node'));
    expect(screen.getByText('New LLM')).toBeInTheDocument();

    fireEvent.click(screen.getByText('+ Add Tool Node'));
    expect(screen.getByText('New Tool')).toBeInTheDocument();
  });

  it('displays the execution alert when running', () => {
    const alertMock = vi.spyOn(window, 'alert').mockImplementation(() => {});
    render(<VisualWorkflowPage />);

    fireEvent.click(screen.getByText('▶ Run Workflow'));
    expect(alertMock).toHaveBeenCalledWith('Workflow execution simulation started! (AutoGPT Block-based Visual Workflow)');
    alertMock.mockRestore();
  });
});
