'use client';

import React, { useState, useRef } from 'react';
import { useRouter } from 'next/navigation';

export default function SnapReceiptPage() {
  const [isUploading, setIsUploading] = useState(false);
  const [toastMessage, setToastMessage] = useState<string | null>(null);
  const [fileSelected, setFileSelected] = useState<File | null>(null);

  // To avoid mock data, we allow the user to input the amount and vendor,
  // or default to something based on the file. In a real app, the backend would OCR this.
  const [amount, setAmount] = useState<number | "">("");
  const [vendor, setVendor] = useState<string>("");

  const fileInputRef = useRef<HTMLInputElement>(null);
  const router = useRouter();

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files.length > 0) {
      setFileSelected(e.target.files[0]);
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!fileSelected) return;

    setIsUploading(true);
    setToastMessage("AI is categorizing your expense...");

    try {
      // Send data to backend
      const response = await fetch('/api/v1/payments/ledger/receipt', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          file_name: fileSelected.name,
          amount: amount,
          vendor: vendor
        }),
      });

      if (response.ok) {
        const data = await response.json();
        setToastMessage(`AI is categorizing your $${data.amount.toFixed(2)} expense at ${data.vendor}... Done. Marked as '${data.category}'.`);

        // Wait a bit to show the success message, then redirect to dashboard
        setTimeout(() => {
          router.push('/dashboard');
        }, 2000);
      } else {
        setToastMessage("Failed to process receipt.");
        setIsUploading(false);
      }
    } catch (error) {
      setToastMessage("Error connecting to server.");
      setIsUploading(false);
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900 flex flex-col items-center justify-center p-4">
      <div className="p-6 rounded-2xl shadow-lg w-full max-w-md glassmorphism">
        <h1 className="text-2xl font-bold font-outfit mb-6 text-gray-900 dark:text-gray-100">Snap Receipt</h1>

        <form onSubmit={handleSubmit} className="flex flex-col gap-4">
          <div className="border-2 border-dashed border-gray-300 dark:border-gray-600 rounded-xl p-8 flex flex-col items-center justify-center text-gray-500 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors cursor-pointer relative">
            <input
              type="file"
              accept="image/*"
              capture="environment"
              className="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
              onChange={handleFileChange}
              data-testid="receipt-file-input"
              ref={fileInputRef}
            />
            <div className="text-4xl mb-2">📸</div>
            <p className="text-sm font-medium">{fileSelected ? `Selected: ${fileSelected.name}` : "Tap to snap or upload"}</p>
          </div>

          <div className="flex flex-col gap-2">
            <label className="text-sm font-semibold text-gray-700 dark:text-gray-300">Amount (Detected)</label>
            <input
              type="number"
              step="0.01"
              value={amount}
              required
              onChange={e => setAmount(e.target.value ? parseFloat(e.target.value) : "")}
              className="glass-control w-full p-3 border border-white/40 dark:border-white/10 text-gray-900 dark:text-gray-100 placeholder-gray-500"
              data-testid="receipt-amount-input"
            />
          </div>

          <div className="flex flex-col gap-2">
            <label className="text-sm font-semibold text-gray-700 dark:text-gray-300">Vendor (Detected)</label>
            <input
              type="text"
              value={vendor}
              required
              onChange={e => setVendor(e.target.value)}
              className="glass-control w-full p-3 border border-white/40 dark:border-white/10 text-gray-900 dark:text-gray-100 placeholder-gray-500"
              data-testid="receipt-vendor-input"
            />
          </div>

          <button
            type="submit"
            disabled={!fileSelected || isUploading || amount === "" || vendor === ""}
            className={`w-full py-3 rounded-xl font-bold text-white transition-all shadow-md mt-4 min-h-[44px] ${fileSelected && !isUploading && amount !== "" && vendor !== "" ? 'bg-[#0066FF] hover:bg-blue-700' : 'bg-gray-400 cursor-not-allowed'}`}
            data-testid="submit-receipt-btn"
          >
            {isUploading ? 'Processing...' : 'Upload Receipt'}
          </button>
        </form>
      </div>

      {toastMessage && (
        <div className="fixed bottom-0 left-0 right-0 p-4 z-50 animate-in slide-in-from-bottom-10" data-testid="receipt-toast">
          <div className="bg-gray-900 text-white p-4 rounded-xl shadow-2xl text-sm font-medium mx-auto max-w-md flex items-center justify-between">
            <span>{toastMessage}</span>
          </div>
        </div>
      )}
    </div>
  );
}
