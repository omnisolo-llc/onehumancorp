"use client";

import React, { useState, useEffect, useRef } from "react";

interface Expense {
  id: string;
  vendor: string | null;
  amount: number | null;
  category: string | null;
  date: string | null;
  status: string;
}

export default function ExpensesPage() {
  const [expenses, setExpenses] = useState<Expense[]>([]);
  const [loading, setLoading] = useState(true);
  const [showCamera, setShowCamera] = useState(false);
  const [uploading, setUploading] = useState(false);
  const videoRef = useRef<HTMLVideoElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);

  // In a real app, this would come from an auth context
  const tenantId = "test_tenant";

  useEffect(() => {
    fetchExpenses();
  }, []);

  const fetchExpenses = async () => {
    setLoading(true);
    try {
      const res = await fetch(`/api/v1/tenants/${tenantId}/expenses`);
      if (res.ok) {
        const data = await res.json();
        setExpenses(data);
      } else {
        console.error("Failed to fetch expenses", res.status);
      }
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  const startCamera = async () => {
    setShowCamera(true);
    try {
      const stream = await navigator.mediaDevices.getUserMedia({
        video: { facingMode: "environment" },
      });
      if (videoRef.current) {
        videoRef.current.srcObject = stream;
      }
    } catch (e) {
      console.error("Error accessing camera", e);
      alert("Could not access camera. Please allow camera permissions.");
      setShowCamera(false);
    }
  };

  const stopCamera = () => {
    if (videoRef.current && videoRef.current.srcObject) {
      const stream = videoRef.current.srcObject as MediaStream;
      stream.getTracks().forEach((track) => track.stop());
    }
    setShowCamera(false);
  };

  const takePhoto = async () => {
    if (videoRef.current && canvasRef.current) {
      const video = videoRef.current;
      const canvas = canvasRef.current;
      canvas.width = video.videoWidth;
      canvas.height = video.videoHeight;
      const ctx = canvas.getContext("2d");
      if (ctx) {
        ctx.drawImage(video, 0, 0, canvas.width, canvas.height);
        // We have the image, now stop the camera and start the "upload" process
        stopCamera();
        uploadExpense(canvas.toDataURL("image/jpeg"));
      }
    }
  };

  const uploadExpense = async (imageDataUrl: string) => {
    setUploading(true);
    try {
      // Simulate saving the image somewhere and passing the path to the API
      const fakeImagePath = `receipts/${Date.now()}.jpg`;

      const res = await fetch(`/api/v1/tenants/${tenantId}/expenses`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          image_path: fakeImagePath,
          date: new Date().toISOString(),
          // The AI agent would extract vendor and amount in the background,
          // but for the sake of the demo, we might leave them null for now,
          // or simulate an extraction:
          vendor: "Scanning...",
        }),
      });

      if (res.ok) {
        const newExpense = await res.json();
        setExpenses([newExpense, ...expenses]);

        // Simulate the AI agent updating the expense shortly after
        setTimeout(() => {
          simulateAIExtraction(newExpense.id);
        }, 3000);

      } else {
        console.error("Failed to upload expense", res.status);
        alert("Failed to upload receipt.");
      }
    } catch (e) {
      console.error(e);
      alert("Error uploading receipt.");
    } finally {
      setUploading(false);
    }
  };

  const simulateAIExtraction = async (expenseId: string) => {
    try {
      const res = await fetch(`/api/v1/tenants/${tenantId}/expenses/${expenseId}`, {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          vendor: "Office Supplies Co.",
          amount: 45.99,
          category: "Office Supplies",
          status: "pending_approval"
        }),
      });
      if (res.ok) {
        fetchExpenses(); // Refresh the list
      }
    } catch(e) {
      console.error("Simulation failed", e);
    }
  };

  const approveExpense = async (expenseId: string) => {
    try {
      const res = await fetch(`/api/v1/tenants/${tenantId}/expenses/${expenseId}`, {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          status: "reconciled"
        }),
      });
      if (res.ok) {
        fetchExpenses();
      }
    } catch(e) {
      console.error(e);
    }
  }

  return (
    <div className="min-h-screen bg-gray-50 pb-24">
      {/* Header */}
      <header className="bg-white px-4 py-6 shadow-sm sticky top-0 z-10">
        <h1 className="text-2xl font-semibold text-gray-900 tracking-tight">Expenses</h1>
        <p className="text-gray-500 text-sm mt-1">Autonomous Capture & Reconciliation</p>
      </header>

      {/* Main Content */}
      <main className="px-4 py-6 max-w-md mx-auto">

        {loading ? (
          <div className="flex justify-center py-10">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-indigo-600"></div>
          </div>
        ) : expenses.length === 0 ? (
          <div className="text-center py-12 bg-white rounded-2xl shadow-sm border border-gray-100">
            <svg className="mx-auto h-12 w-12 text-gray-300" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
            <h3 className="mt-4 text-sm font-medium text-gray-900">No expenses yet</h3>
            <p className="mt-1 text-sm text-gray-500 max-w-[200px] mx-auto">Snap a picture of a receipt to let the AI agent categorize it.</p>
          </div>
        ) : (
          <div className="space-y-4">
            {expenses.map((expense) => (
              <div
                key={expense.id}
                className={`bg-white rounded-2xl p-4 shadow-sm border transition-all ${
                  expense.status === 'pending_approval'
                    ? 'border-indigo-200 ring-1 ring-indigo-50'
                    : 'border-gray-100'
                }`}
              >
                <div className="flex justify-between items-start mb-2">
                  <div>
                    <h3 className="font-medium text-gray-900">
                      {expense.vendor || "Unknown Vendor"}
                    </h3>
                    <p className="text-xs text-gray-500">
                      {expense.date ? new Date(expense.date).toLocaleDateString() : 'No date'}
                    </p>
                  </div>
                  <div className="text-right">
                    <span className="font-semibold text-gray-900">
                      {expense.amount ? `$${expense.amount.toFixed(2)}` : '---'}
                    </span>
                    <p className="text-xs mt-1">
                      {expense.status === 'pending' && <span className="text-amber-500 bg-amber-50 px-2 py-0.5 rounded-full">Processing AI...</span>}
                      {expense.status === 'pending_approval' && <span className="text-indigo-600 bg-indigo-50 px-2 py-0.5 rounded-full">Needs Approval</span>}
                      {expense.status === 'reconciled' && <span className="text-emerald-600 bg-emerald-50 px-2 py-0.5 rounded-full">Reconciled</span>}
                    </p>
                  </div>
                </div>

                {expense.category && (
                  <div className="mb-3">
                    <span className="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-gray-100 text-gray-600">
                      {expense.category}
                    </span>
                  </div>
                )}

                {expense.status === 'pending_approval' && (
                  <div className="mt-4 pt-3 border-t border-gray-50 flex gap-2">
                    <button
                      onClick={() => approveExpense(expense.id)}
                      className="flex-1 bg-indigo-600 text-white text-sm font-medium py-2 rounded-xl active:bg-indigo-700 transition-colors"
                    >
                      Approve & Reconcile
                    </button>
                    <button className="flex-1 bg-gray-100 text-gray-700 text-sm font-medium py-2 rounded-xl active:bg-gray-200 transition-colors">
                      Edit
                    </button>
                  </div>
                )}
              </div>
            ))}
          </div>
        )}
      </main>

      {/* Floating Action Button */}
      {!showCamera && (
        <div className="fixed bottom-6 right-6">
          <button
            onClick={startCamera}
            disabled={uploading}
            className={`flex items-center justify-center w-14 h-14 rounded-full shadow-lg text-white transition-transform active:scale-95 ${
              uploading ? 'bg-gray-400' : 'bg-indigo-600 hover:bg-indigo-700'
            }`}
          >
            {uploading ? (
              <div className="w-6 h-6 border-2 border-white border-t-transparent rounded-full animate-spin" />
            ) : (
              <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 9a2 2 0 012-2h.93a2 2 0 001.664-.89l.812-1.22A2 2 0 0110.07 4h3.86a2 2 0 011.664.89l.812 1.22A2 2 0 0018.07 7H19a2 2 0 012 2v9a2 2 0 01-2 2H5a2 2 0 01-2-2V9z" />
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 13a3 3 0 11-6 0 3 3 0 016 0z" />
              </svg>
            )}
          </button>
        </div>
      )}

      {/* Fullscreen Camera Overlay */}
      {showCamera && (
        <div className="fixed inset-0 z-50 bg-black flex flex-col">
          <div className="relative flex-1">
            <video
              ref={videoRef}
              autoPlay
              playsInline
              className="absolute inset-0 w-full h-full object-cover"
            />
            {/* macOS Translucent Glass Overlay Guide */}
            <div className="absolute inset-0 pointer-events-none flex items-center justify-center p-8">
              <div className="w-full h-[60%] border-2 border-white/50 rounded-2xl backdrop-blur-sm bg-white/10 flex items-center justify-center">
                <span className="text-white/80 font-medium text-sm tracking-wide">Align receipt within frame</span>
              </div>
            </div>

            <button
              onClick={stopCamera}
              className="absolute top-6 right-6 w-10 h-10 bg-black/40 backdrop-blur-md text-white rounded-full flex items-center justify-center z-10"
            >
              <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>

          <div className="h-32 bg-black flex items-center justify-center pb-8">
            <button
              onClick={takePhoto}
              className="w-16 h-16 rounded-full border-4 border-white/30 p-1 active:scale-95 transition-transform"
            >
              <div className="w-full h-full bg-white rounded-full"></div>
            </button>
          </div>
          <canvas ref={canvasRef} className="hidden" />
        </div>
      )}
    </div>
  );
}
