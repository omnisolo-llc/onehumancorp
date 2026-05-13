import React from 'react';
import { WizardState } from './types';
export default function PaymentStep({ state, update, next, prev }: { state: WizardState, update: (k: keyof WizardState, v: string) => void, next: () => void, prev: () => void }) {
    const pays = ['Online only', 'In-person (POS)', 'Both', 'Skip for now'];
    return (
        <div className="glass-panel p-8 rounded-2xl border border-gray-700">
            <h2 className="text-3xl font-bold mb-8">How do you want to receive payments?</h2>
            <div className="flex flex-col gap-4 mb-8">
                {pays.map(p => (
                    <button key={p} onClick={() => update('payment', p)} className={`p-6 border rounded-xl text-left text-lg ${state.payment === p ? 'border-green-500 bg-green-500/10' : 'border-gray-600 hover:border-gray-400'}`}>
                        {p}
                    </button>
                ))}
            </div>
            <div className="flex justify-between">
                <button onClick={prev} className="px-6 py-2 border border-gray-600 rounded-lg">Back</button>
                <button onClick={next} disabled={!state.payment} className="px-6 py-2 bg-blue-500 rounded-lg disabled:opacity-50">Next</button>
            </div>
        </div>
    );
}