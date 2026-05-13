import React from 'react';
export default function WelcomeStep({ next }: { next: () => void }) {
    return (
        <div className="glass-panel p-12 text-center rounded-2xl border border-gray-700">
            <h1 className="text-5xl font-bold mb-6 outfit-font">Your business, live in minutes</h1>
            <p className="text-xl text-gray-400 mb-12">The simplest way to start your journey.</p>
            <button onClick={next} className="bg-green-500 hover:bg-green-600 text-white px-8 py-4 rounded-xl text-xl font-bold transition-all">Launch My Business -></button>
        </div>
    );
}