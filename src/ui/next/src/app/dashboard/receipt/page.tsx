"use client";

import { useState, useRef } from "react";
import { useRouter } from "next/navigation";

export default function SnapReceiptPage() {
  const [isUploading, setIsUploading] = useState(false);
  const [toastMessage, setToastMessage] = useState<string | null>(null);
  const [fileSelected, setFileSelected] = useState<File | null>(null);

  // In a real app, the backend would OCR this from the file.
  const [amount, setAmount] = useState<number | "">("");
  const [vendor, setVendor] = useState<string>("");

  const fileInputRef = useRef<HTMLInputElement>(null);
  const router = useRouter();

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files.length > 0) {
      setFileSelected(e.target.files[0]);
    }
  };

  const handleUploadClick = () => {
    fileInputRef.current?.click();
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!fileSelected && !amount && !vendor) {
      showToast("Please provide receipt details or an image.");
      return;
    }

    setIsUploading(true);

    try {
      // Create FormData to send file and data
      const formData = new FormData();
      if (fileSelected) formData.append("receipt_image", fileSelected);
      if (amount) formData.append("amount", amount.toString());
      if (vendor) formData.append("vendor", vendor);

      const res = await fetch("/api/v1/expenses/receipts", {
        method: "POST",
        body: formData,
      });

      if (!res.ok) {
        throw new Error("Failed to process receipt");
      }

      showToast("Receipt saved to Expense Tracker!");

      // Delay slightly for the toast to be seen before navigating
      setTimeout(() => {
        router.push("/dashboard/expenses");
      }, 1500);

    } catch (err) {
      console.error(err);
      showToast("Error saving receipt. Please try again.");
    } finally {
      setIsUploading(false);
    }
  };

  const showToast = (msg: string) => {
    setToastMessage(msg);
    setTimeout(() => setToastMessage(null), 3000);
  };

  return (
    <div className="flex flex-col h-full bg-white p-4 max-w-md mx-auto relative">
      <h1 className="text-2xl font-bold mb-4 font-outfit">Snap a Receipt</h1>
      <p className="text-gray-500 mb-6 text-sm">Upload a photo of your expense receipt or enter the details manually.</p>

      <form onSubmit={handleSubmit} className="flex flex-col gap-6">

        {/* Mock OCR Results Area */}
        <div className="flex flex-col gap-4 p-4 border rounded-xl bg-gray-50">
          <h2 className="text-sm font-semibold text-gray-400 uppercase tracking-wider">Receipt Details</h2>

          <div className="flex flex-col gap-1">
            <label className="text-sm font-semibold text-gray-700">Amount (Detected)</label>
            <input
              type="number"
              step="0.01"
              value={amount}
              onChange={e => setAmount(e.target.value ? parseFloat(e.target.value) : "")}
              className="border p-2 rounded-lg"
              data-testid="receipt-amount-input"
              required
            />
          </div>

          <div className="flex flex-col gap-1">
            <label className="text-sm font-semibold text-gray-700">Vendor (Detected)</label>
            <input
              type="text"
              value={vendor}
              onChange={e => setVendor(e.target.value)}
              className="border p-2 rounded-lg"
              data-testid="receipt-vendor-input"
              required
            />
          </div>

        </div>

        {/* File Upload Area */}
        <div
          className="flex flex-col items-center justify-center border-2 border-dashed border-gray-300 rounded-2xl p-6 gap-2 cursor-pointer hover:bg-gray-50 transition-colors"
          onClick={handleUploadClick}
        >
          <svg className="w-8 h-8 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 9a2 2 0 012-2h.93a2 2 0 001.664-.89l.812-1.22A2 2 0 0110.07 4h3.86a2 2 0 011.664.89l.812 1.22A2 2 0 0018.07 7H19a2 2 0 012 2v9a2 2 0 01-2 2H5a2 2 0 01-2-2V9z" />
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 13a3 3 0 11-6 0 3 3 0 016 0z" />
          </svg>
          <span className="text-sm font-medium text-gray-600 text-center">
            {fileSelected ? fileSelected.name : "Tap to snap photo or select file"}
          </span>
          <input
            type="file"
            accept="image/*"
            capture="environment"
            className="hidden"
            ref={fileInputRef}
            onChange={handleFileChange}
          />
        </div>

        <button
          type="submit"
          disabled={isUploading}
          className="bg-blue-600 hover:bg-blue-700 text-white font-bold py-3 rounded-xl shadow-md disabled:opacity-50 transition-colors"
        >
          {isUploading ? "Processing..." : "Save Expense"}
        </button>

      </form>

      {/* Temporary Toast for manual feedback */}
      {toastMessage && (
        <div className="absolute bottom-10 left-1/2 -translate-x-1/2 bg-gray-800 text-white px-4 py-2 rounded-full text-sm whitespace-nowrap z-50">
          {toastMessage}
        </div>
      )}
    </div>
  );
}
