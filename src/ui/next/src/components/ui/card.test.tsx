import React from 'react';
import { render } from '@testing-library/react';
import '@testing-library/jest-dom/vitest';
import { describe, it, expect } from 'vitest';
import { Card, CardHeader, CardFooter, CardTitle, CardDescription, CardContent } from './card';

describe('Card Component', () => {
  it('renders correctly', () => {
    const { getByText } = render(
      <Card className="custom-card-class">
        <CardHeader className="custom-header-class">
          <CardTitle className="custom-title-class">Title</CardTitle>
          <CardDescription className="custom-desc-class">Description</CardDescription>
        </CardHeader>
        <CardContent className="custom-content-class">Content</CardContent>
        <CardFooter className="custom-footer-class">Footer</CardFooter>
      </Card>
    );

    const titleElement = getByText('Title');
    expect(titleElement).toBeInTheDocument();
    expect(titleElement).toHaveClass('custom-title-class');

    const descElement = getByText('Description');
    expect(descElement).toBeInTheDocument();
    expect(descElement).toHaveClass('custom-desc-class');

    const contentElement = getByText('Content');
    expect(contentElement).toBeInTheDocument();
    expect(contentElement).toHaveClass('custom-content-class');

    const footerElement = getByText('Footer');
    expect(footerElement).toBeInTheDocument();
    expect(footerElement).toHaveClass('custom-footer-class');
  });

  it('uses canonical glass design tokens', () => {
    const { container } = render(<Card>Glass Card</Card>);
    const card = container.firstChild as HTMLElement;
    expect(card).toHaveClass('rounded-[16px]');
    expect(card).toHaveClass('bg-[rgba(255,255,255,0.65)]');
    expect(card).toHaveClass('dark:bg-[rgba(22,22,26,0.7)]');
    expect(card).toHaveClass('border-[rgba(255,255,255,0.4)]');
    expect(card).toHaveClass('dark:border-[rgba(255,255,255,0.1)]');
    expect(card).toHaveClass('backdrop-blur-[30px]');
    expect(card).toHaveClass('backdrop-saturate-[210%]');
  });
});
