import React from 'react';
import { QRCodeSVG } from 'qrcode.react';

interface QRCodeGeneratorProps {
    value: string;
    storeName?: string;
}

export default function QRCodeGenerator({ value, storeName }: QRCodeGeneratorProps) {
    return (
        <div data-testid="qr-code-container" className="flex flex-col items-center justify-center p-6 bg-white/80 backdrop-blur-xl rounded-2xl shadow-sm border border-white/40">
            <h3 className="text-lg font-outfit font-semibold mb-4 text-gray-800">
                Storefront QR Code
            </h3>
            <div className="bg-white p-4 rounded-xl shadow-inner border border-gray-100">
                <QRCodeSVG
                    value={value}
                    size={200}
                    level="H"
                    includeMargin={true}
                    data-testid="qr-code-svg"
                />
            </div>
            <p className="mt-4 text-sm text-gray-500 font-medium text-center">
                Scan to visit {storeName || 'the store'}
            </p>
            <div className="mt-2 flex items-center justify-center gap-1 opacity-70">
                <span className="text-xs font-semibold tracking-wider uppercase text-indigo-600">Powered by OHC</span>
            </div>
        </div>
    );
}
