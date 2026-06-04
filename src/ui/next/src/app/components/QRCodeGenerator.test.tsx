import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import QRCodeGenerator from './QRCodeGenerator';
import React from 'react';

describe('QRCodeGenerator', () => {
    it('renders the QR code with the provided value', () => {
        render(<QRCodeGenerator value="https://ohc.store/test" />);
        expect(screen.getByTestId('qr-code-container')).toBeDefined();
        expect(screen.getByTestId('qr-code-svg')).toBeDefined();
    });

    it('renders the store name if provided', () => {
        render(<QRCodeGenerator value="https://ohc.store/test" storeName="Maya's Cakes" />);
        expect(screen.getByText("Scan to visit Maya's Cakes")).toBeDefined();
    });

    it('renders default text if store name is missing', () => {
        render(<QRCodeGenerator value="https://ohc.store/test" />);
        expect(screen.getByText("Scan to visit the store")).toBeDefined();
    });

    it('contains the viral loop branding', () => {
        render(<QRCodeGenerator value="https://ohc.store/test" />);
        expect(screen.getByText(/Powered by OHC/i)).toBeDefined();
    });
});
