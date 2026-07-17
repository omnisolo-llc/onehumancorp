import React from 'react';
import { render } from '@testing-library/react';
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
});
