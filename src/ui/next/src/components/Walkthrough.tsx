"use client";
import React, { useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';

const tours = {
  storeSetup: [
    { target: 'store-name', content: 'First, enter a name for your store.' },
    { target: 'add-product', content: 'Next, add your first product to sell.' },
  ],
  payment: [
    { target: 'bank-details', content: 'Enter your bank details to receive money.' },
  ]
};

export const Walkthrough: React.FC = () => {
  const [activeTour, setActiveTour] = useState<string | null>(null);
  const [stepIndex, setStepIndex] = useState(0);

  const startTour = (tourName: string) => {
    setActiveTour(tourName);
    setStepIndex(0);
  };

  const nextStep = () => {
    if (activeTour && stepIndex < tours[activeTour as keyof typeof tours].length - 1) {
      setStepIndex(stepIndex + 1);
    } else {
      setActiveTour(null);
    }
  };

  return (
    <>
      <div style={{ padding: '20px' }}>
        <h3>Interactive Guides</h3>
        <button onClick={() => startTour('storeSetup')} style={btnStyle}>How to set up your store</button>
        <button onClick={() => startTour('payment')} style={{...btnStyle, marginLeft: '10px'}}>How to accept payments</button>
      </div>

      <AnimatePresence>
        {activeTour && (
          <motion.div
            initial={{ opacity: 0, scale: 0.9 }}
            animate={{ opacity: 1, scale: 1 }}
            exit={{ opacity: 0, scale: 0.9 }}
            style={{
              position: 'fixed',
              bottom: '20px',
              right: '20px',
              background: 'rgba(255, 255, 255, 0.95)',
              backdropFilter: 'blur(15px)',
              padding: '20px',
              borderRadius: '12px',
              boxShadow: '0 10px 25px rgba(0,0,0,0.1)',
              zIndex: 1001,
              fontFamily: 'Outfit, sans-serif',
              width: '300px'
            }}
          >
            <h4>Guide: {activeTour}</h4>
            <p>{tours[activeTour as keyof typeof tours]?.[stepIndex]?.content}</p>
            <div style={{ display: 'flex', justifyContent: 'space-between', marginTop: '15px' }}>
              <button onClick={() => setActiveTour(null)} style={outlineBtnStyle}>Close</button>
              <button onClick={nextStep} style={primaryBtnStyle}>Next</button>
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </>
  );
};

const btnStyle = { padding: '8px 16px', borderRadius: '6px', border: '1px solid #ccc', cursor: 'pointer', background: '#fff' };
const primaryBtnStyle = { ...btnStyle, background: '#0070f3', color: '#fff', border: 'none' };
const outlineBtnStyle = { ...btnStyle, background: 'transparent' };
