import React, { useState, useEffect } from 'react';

interface Step {
    targetId: string;
    title: string;
    content: string;
}

interface WalkthroughProps {
    steps: Step[];
    onComplete?: () => void;
}

export const InteractiveWalkthrough: React.FC<WalkthroughProps> = ({ steps, onComplete }) => {
    const [currentStep, setCurrentStep] = useState(0);
    const [targetRect, setTargetRect] = useState<DOMRect | null>(null);

    useEffect(() => {
        const target = document.getElementById(steps[currentStep].targetId);
        if (target) {
            setTargetRect(target.getBoundingClientRect());
        }
    }, [currentStep, steps]);

    if (!targetRect) return null;

    const nextStep = () => {
        if (currentStep < steps.length - 1) {
            setCurrentStep(currentStep + 1);
        } else {
            if (onComplete) onComplete();
        }
    };

    return (
        <div style={{
            position: 'fixed',
            top: 0,
            left: 0,
            width: '100vw',
            height: '100vh',
            pointerEvents: 'none',
            zIndex: 9998
        }}>
            {/* Overlay mask with cutout */}
            <div style={{
                position: 'absolute',
                top: 0, left: 0, right: 0, bottom: 0,
                backgroundColor: 'rgba(0,0,0,0.5)',
                clipPath: `polygon(0% 0%, 0% 100%, ${targetRect.left}px 100%, ${targetRect.left}px ${targetRect.top}px, ${targetRect.right}px ${targetRect.top}px, ${targetRect.right}px ${targetRect.bottom}px, ${targetRect.left}px ${targetRect.bottom}px, ${targetRect.left}px 100%, 100% 100%, 100% 0%)`
            }} />

            {/* Speech bubble */}
            <div style={{
                position: 'absolute',
                top: targetRect.bottom + 15,
                left: targetRect.left,
                width: '300px',
                backgroundColor: 'white',
                padding: '20px',
                borderRadius: '8px',
                pointerEvents: 'auto',
                boxShadow: '0 10px 25px rgba(0,0,0,0.2)',
                fontFamily: 'Inter, sans-serif'
            }}>
                <h3 style={{ margin: '0 0 10px 0', fontSize: '16px' }}>{steps[currentStep].title}</h3>
                <p style={{ margin: '0 0 15px 0', fontSize: '14px', color: '#555' }}>{steps[currentStep].content}</p>

                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                    <span style={{ fontSize: '12px', color: '#888' }}>Step {currentStep + 1} of {steps.length}</span>
                    <button
                        onClick={nextStep}
                        style={{
                            padding: '8px 16px',
                            backgroundColor: '#0070f3',
                            color: 'white',
                            border: 'none',
                            borderRadius: '4px',
                            cursor: 'pointer'
                        }}
                    >
                        {currentStep < steps.length - 1 ? 'Next' : 'Got it'}
                    </button>
                </div>

                <div style={{
                    position: 'absolute',
                    bottom: '100%',
                    left: '20px',
                    borderWidth: '10px',
                    borderStyle: 'solid',
                    borderColor: 'transparent transparent white transparent'
                }} />
            </div>
        </div>
    );
};
