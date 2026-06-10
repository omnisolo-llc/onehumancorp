"use client";

import React, { useState, Suspense } from 'react';
import { useSearchParams } from 'next/navigation';

function WidgetContent() {
    const searchParams = useSearchParams();
    const tenantId = searchParams.get('tenant_id') || 'demo';
    const type = searchParams.get('type') || 'intake';
    const theme = searchParams.get('theme') || 'light';

    const [submitted, setSubmitted] = useState(false);

    const isDark = theme === 'dark';

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();

        try {
            const formData = new FormData(e.target as HTMLFormElement);
            // Post to the actual intake API
            const response = await fetch(`/api/v1/work-intake/submit?tenant=${encodeURIComponent(tenantId)}`, {
                method: 'POST',
                body: formData,
            });

            if (response.ok) {
                setSubmitted(true);
            } else {
                console.error('Failed to submit form');
            }
        } catch (error) {
            console.error('Error submitting form', error);
        }
    };

    const getThemeClasses = () => {
        if (isDark) {
            return {
                bg: 'bg-gray-900',
                text: 'text-white',
                textMuted: 'text-gray-400',
                inputBg: 'bg-gray-800',
                inputBorder: 'border-gray-700',
                inputText: 'text-white',
                btnBg: 'bg-blue-600',
                btnHover: 'hover:bg-blue-700',
                btnText: 'text-white',
            };
        }
        return {
            bg: 'bg-white',
            text: 'text-gray-900',
            textMuted: 'text-gray-500',
            inputBg: 'bg-gray-50',
            inputBorder: 'border-gray-200',
            inputText: 'text-gray-900',
            btnBg: 'bg-blue-600',
            btnHover: 'hover:bg-blue-700',
            btnText: 'text-white',
        };
    };

    const t = getThemeClasses();

    if (submitted) {
        return (
            <div className={`flex flex-col items-center justify-center h-full w-full p-6 text-center font-sans ${t.bg}`}>
                <div className="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center mb-4">
                    <svg className="w-8 h-8 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                </div>
                <h2 className={`text-xl font-bold mb-2 ${t.text}`}>Success!</h2>
                <p className={`text-sm ${t.textMuted}`}>Your request has been sent to the team. We'll be in touch shortly.</p>
            </div>
        );
    }

    return (
        <div className={`flex flex-col h-full w-full p-6 font-sans ${t.bg} transition-colors duration-200`}>
            <div className="mb-6">
                <h2 className={`text-2xl font-bold mb-1 ${t.text}`}>
                    {type === 'intake' && 'Get Started'}
                    {type === 'booking' && 'Book an Appointment'}
                    {type === 'quote' && 'Request a Quote'}
                </h2>
                <p className={`text-sm ${t.textMuted}`}>
                    {type === 'intake' && 'Tell us what you need and we will help you out.'}
                    {type === 'booking' && 'Select a time that works best for you.'}
                    {type === 'quote' && 'Provide details for an accurate estimate.'}
                </p>
            </div>

            <form onSubmit={handleSubmit} className="flex-1 flex flex-col gap-4">
                <div>
                    <label className={`block text-xs font-semibold mb-1 ${t.text}`}>Name</label>
                    <input
                        required
                        name="name"
                        type="text"
                        placeholder="Your full name"
                        className={`w-full px-3 py-2 rounded-lg text-sm border focus:outline-none focus:ring-2 focus:ring-blue-500 transition-colors ${t.inputBg} ${t.inputBorder} ${t.inputText}`}
                    />
                </div>

                <div>
                    <label className={`block text-xs font-semibold mb-1 ${t.text}`}>Email</label>
                    <input
                        required
                        name="email"
                        type="email"
                        placeholder="you@example.com"
                        className={`w-full px-3 py-2 rounded-lg text-sm border focus:outline-none focus:ring-2 focus:ring-blue-500 transition-colors ${t.inputBg} ${t.inputBorder} ${t.inputText}`}
                    />
                </div>

                {type === 'booking' && (
                    <div>
                        <label className={`block text-xs font-semibold mb-1 ${t.text}`}>Preferred Date</label>
                        <input
                            required
                            name="date"
                            type="date"
                            className={`w-full px-3 py-2 rounded-lg text-sm border focus:outline-none focus:ring-2 focus:ring-blue-500 transition-colors ${t.inputBg} ${t.inputBorder} ${t.inputText}`}
                        />
                    </div>
                )}

                <div className="flex-1">
                    <label className={`block text-xs font-semibold mb-1 ${t.text}`}>
                        {type === 'intake' && 'How can we help?'}
                        {type === 'booking' && 'Additional Notes'}
                        {type === 'quote' && 'Project Details'}
                    </label>
                    <textarea
                        required
                        name="details"
                        rows={type === 'booking' ? 2 : 4}
                        placeholder={type === 'quote' ? 'Describe your project...' : 'Provide more details...'}
                        className={`w-full px-3 py-2 rounded-lg text-sm border focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none transition-colors ${t.inputBg} ${t.inputBorder} ${t.inputText}`}
                    />
                </div>

                <button
                    type="submit"
                    className={`mt-auto w-full py-3 rounded-xl font-bold text-sm shadow-sm transition-transform active:scale-[0.98] ${t.btnBg} ${t.btnHover} ${t.btnText}`}
                >
                    {type === 'intake' && 'Submit Request'}
                    {type === 'booking' && 'Confirm Booking'}
                    {type === 'quote' && 'Get My Quote'}
                </button>
            </form>
        </div>
    );
}

export default function WidgetEmbedPage() {
    return (
        <Suspense fallback={<div className="p-8 text-center text-gray-500 font-sans">Loading widget...</div>}>
            <WidgetContent />
        </Suspense>
    );
}
