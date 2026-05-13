"use client";
import React, { useState, useEffect, useRef } from 'react';
import { createPortal } from 'react-dom';

export default function Walkthrough() {
    const [steps, setSteps] = useState<{id: string, target: string, message: string, position: string}[]>([]);
    const [currentStep, setCurrentStep] = useState(0);
    const [coords, setCoords] = useState({top: 0, left: 0, opacity: 0});
    const observerRef = useRef<MutationObserver | null>(null);

    useEffect(() => {
        fetch('/api/help/walkthroughs')
            .then(res => res.json())
            .then(data => setSteps(data))
            .catch(() => {});
    }, []);

    useEffect(() => {
        if (steps.length > 0 && currentStep < steps.length && typeof document !== 'undefined') {
            const currentTarget = steps[currentStep].target;

            const positionHighlight = () => {
                const el = document.querySelector(currentTarget);
                if (el) {
                    const rect = el.getBoundingClientRect();
                    setCoords({
                        top: rect.bottom + window.scrollY,
                        left: rect.left + window.scrollX,
                        opacity: 1
                    });

                    document.querySelectorAll('.ohc-walkthrough-highlight').forEach(e => e.classList.remove('ohc-walkthrough-highlight'));

                    el.classList.add('ohc-walkthrough-highlight');
                    if (document.getElementById('ohc-walkthrough-style') === null) {
                        const style = document.createElement('style');
                        style.id = 'ohc-walkthrough-style';
                        style.innerHTML = `
                            .ohc-walkthrough-highlight {
                                position: relative;
                                z-index: 99998;
                                box-shadow: 0 0 0 9999px rgba(0,0,0,0.5);
                                border-radius: 4px;
                            }
                        `;
                        document.head.appendChild(style);
                    }

                } else {
                    setCoords(c => ({...c, opacity: 0}));
                }
            };

            // Initial check
            positionHighlight();

            // Setup MutationObserver to watch for dynamic DOM changes (better than setInterval)
            observerRef.current = new MutationObserver((mutations) => {
                positionHighlight();
            });

            observerRef.current.observe(document.body, {
                childList: true,
                subtree: true,
                attributes: true,
                attributeFilter: ['style', 'class']
            });

            return () => {
                if (observerRef.current) observerRef.current.disconnect();
                document.querySelectorAll('.ohc-walkthrough-highlight').forEach(e => e.classList.remove('ohc-walkthrough-highlight'));
            };
        } else {
            setCoords(c => ({...c, opacity: 0}));
            if (typeof document !== 'undefined') {
                document.querySelectorAll('.ohc-walkthrough-highlight').forEach(e => e.classList.remove('ohc-walkthrough-highlight'));
            }
        }
    }, [currentStep, steps]);

    if (steps.length === 0 || currentStep >= steps.length || coords.opacity === 0 || typeof document === 'undefined') return null;

    return createPortal(
        <div className="walkthrough-overlay speech-bubble" style={{
            position: 'absolute',
            top: coords.top + 10,
            left: coords.left,
            zIndex: 99999,
            padding: '16px',
            background: 'rgba(255,255,255,0.95)',
            backdropFilter: 'blur(20px) saturate(200%)',
            border: '1px solid rgba(0,0,0,0.1)',
            borderRadius: '12px',
            boxShadow: '0 10px 25px rgba(0,0,0,0.2)',
            maxWidth: '300px'
        }}>
            <h4 style={{ margin: '0 0 8px 0' }}>Step {currentStep + 1} of {steps.length}</h4>
            <p style={{ margin: '0 0 16px 0' }}>{steps[currentStep].message}</p>
            <div style={{ display: 'flex', justifyContent: 'space-between' }}>
                <button
                    onClick={() => setCurrentStep(steps.length)}
                    style={{ background: 'transparent', border: 'none', color: '#666', cursor: 'pointer' }}
                >Skip</button>
                <button
                    onClick={() => setCurrentStep(c => c + 1)}
                    style={{ background: '#0070f3', color: '#fff', border: 'none', padding: '6px 12px', borderRadius: '4px', cursor: 'pointer' }}
                >Next →</button>
            </div>
        </div>,
        document.body
    );
}
