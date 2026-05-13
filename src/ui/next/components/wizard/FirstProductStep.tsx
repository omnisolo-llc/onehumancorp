import React from 'react';
import { WizardState } from './types';
export default function FirstProductStep({ state, update, next, prev }: { state: WizardState, update: (k: keyof WizardState, v: any) => void, next: () => void, prev: () => void }) {
    return (
        <div className="glass-panel p-8 rounded-2xl border border-gray-700">
            <h2 className="text-3xl font-bold mb-8">Add your first product or service</h2>
            <input type="text" onChange={e => update('products', [{name: e.target.value}])} placeholder="Product Name" className="w-full p-4 bg-gray-800 border border-gray-600 rounded-xl mb-4" />
            <input type="number" placeholder="Price" className="w-full p-4 bg-gray-800 border border-gray-600 rounded-xl mb-4" />
            <div className="flex justify-between mt-8">
                <button onClick={prev} className="px-6 py-2 border border-gray-600 rounded-lg">Back</button>
                <button onClick={next} className="px-6 py-2 bg-blue-500 rounded-lg">Next</button>
            </div>
        </div>
    );
}