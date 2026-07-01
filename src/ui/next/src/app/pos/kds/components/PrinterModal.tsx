import React, { useState } from 'react';

export function PrinterModal({ onClose }: { onClose: () => void }) {
  const [connecting, setConnecting] = useState(false);
  const [connected, setConnected] = useState(false);

  const connectPrinter = () => {
    setConnecting(true);
    setTimeout(() => {
      setConnecting(false);
      setConnected(true);
    }, 1000);
  };

  const testPrint = () => {
    alert('Test Print Sent via ESC/POS');
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm">
      <div className="bg-white w-[340px] rounded-2xl p-6 shadow-2xl">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-xl font-bold text-gray-900">Printer Settings</h2>
          <button onClick={onClose} className="text-gray-500 font-bold text-xl hover:text-gray-900">&times;</button>
        </div>

        <p className="text-gray-600 mb-6 text-sm">Connect a Bluetooth Thermal Printer (ESC/POS compatible) to print receipts.</p>

        {connected ? (
          <div className="flex flex-col gap-3">
            <div className="p-3 bg-green-50 border border-green-200 text-green-700 rounded-lg font-medium text-center">
              ✓ Printer Connected
            </div>
            <button onClick={testPrint} className="w-full py-3 bg-blue-50 text-blue-700 font-bold rounded-xl border border-blue-200 active:scale-95 transition" data-testid="btn-test-print">
              Test Print
            </button>
            <button onClick={() => setConnected(false)} className="w-full py-3 bg-gray-100 text-gray-700 font-bold rounded-xl active:scale-95 transition">
              Disconnect
            </button>
          </div>
        ) : (
          <button onClick={connectPrinter} disabled={connecting} className="w-full py-4 bg-[#0071E3] text-white font-bold rounded-xl active:scale-95 transition" data-testid="btn-connect-printer">
            {connecting ? 'Scanning...' : 'Connect Printer'}
          </button>
        )}
      </div>
    </div>
  );
}
