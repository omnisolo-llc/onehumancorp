"use client";

import React, { useState } from 'react';

export default function WalkthroughOverlay() {
  const [activeStep, setActiveStep] = useState<number | null>(null);

  if (activeStep === null) return null;

  return (
    <div className="fixed inset-0 z-40 bg-black bg-opacity-50 flex items-center justify-center pointer-events-none">
      <div className="bg-white p-6 rounded-lg pointer-events-auto">
        <h3 className="font-bold mb-2">Interactive Tour</h3>
        <p>This is a step-by-step guide.</p>
        <button onClick={() => setActiveStep(null)} className="mt-4 bg-blue-500 text-white px-4 py-2 rounded">Close</button>
      </div>
    </div>
  );
}
