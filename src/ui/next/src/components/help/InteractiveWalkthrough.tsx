import React, { useState } from 'react';

// Interactive Walkthrough
// Step-by-step in-app tours. Implemented as an overlay highlight + speech bubble.
interface Step {
  targetId: string;
  text: string;
}

export const InteractiveWalkthrough: React.FC<{ steps: Step[], onComplete: () => void }> = ({ steps, onComplete }) => {
  const [currentStepIndex, setCurrentStepIndex] = useState(0);

  if (steps.length === 0) return null;

  const currentStep = steps[currentStepIndex];

  // In a real implementation, we'd use DOM APIs to get bounding rects of the targetId
  // and position this overlay absolutely. For this component, we simulate the overlay.

  const handleNext = () => {
    if (currentStepIndex < steps.length - 1) {
      setCurrentStepIndex(currentStepIndex + 1);
    } else {
      onComplete();
    }
  };

  return (
    <div style={{
      position: 'fixed',
      top: 0, left: 0, right: 0, bottom: 0,
      background: 'rgba(0,0,0,0.5)',
      zIndex: 9998,
      display: 'flex',
      justifyContent: 'center',
      alignItems: 'center'
    }}>
      {/* Speech bubble */}
      <div style={{
        background: 'white',
        padding: '24px',
        borderRadius: '12px',
        maxWidth: '300px',
        boxShadow: '0 8px 30px rgba(0,0,0,0.2)',
        fontFamily: 'Inter, sans-serif'
      }}>
        <div style={{ fontSize: '12px', color: '#888', marginBottom: '8px', fontWeight: 'bold' }}>
          STEP {currentStepIndex + 1} OF {steps.length}
        </div>
        <p style={{ margin: '0 0 20px 0', fontSize: '16px', lineHeight: '1.5' }}>
          {currentStep.text}
        </p>
        <div style={{ display: 'flex', justifyContent: 'flex-end', gap: '12px' }}>
          <button
            onClick={onComplete}
            style={{ background: 'none', border: 'none', color: '#666', cursor: 'pointer', fontWeight: 'bold' }}
          >
            Skip Tour
          </button>
          <button
            onClick={handleNext}
            style={{ background: '#0056b3', color: 'white', border: 'none', padding: '8px 16px', borderRadius: '6px', cursor: 'pointer', fontWeight: 'bold' }}
          >
            {currentStepIndex < steps.length - 1 ? 'Next' : 'Finish'}
          </button>
        </div>
      </div>
    </div>
  );
};
