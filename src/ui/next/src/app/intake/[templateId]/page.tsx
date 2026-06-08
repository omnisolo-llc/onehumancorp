"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function ClientIntakePage({ params }: { params: { templateId: string } }) {
    const router = useRouter();
    const [template, setTemplate] = useState<any>(null);
    const [currentStep, setCurrentStep] = useState(0);
    const [answers, setAnswers] = useState<Record<string, string>>({});
    const [customerName, setCustomerName] = useState("");
    const [customerEmail, setCustomerEmail] = useState("");
    const [loading, setLoading] = useState(true);
    const [submitting, setSubmitting] = useState(false);
    const [submitted, setSubmitted] = useState(false);

    useEffect(() => {
        const fetchTemplate = async () => {
            try {
                const response = await fetch(`/api/v1/intake/templates/${params.templateId}`);
                if (response.ok) {
                    const data = await response.json();
                    setTemplate(data);
                }
            } catch (e) {
                console.error("Failed to fetch template", e);
            } finally {
                setLoading(false);
            }
        };
        fetchTemplate();
    }, [params.templateId]);

    if (loading) {
        return <div className="min-h-screen flex items-center justify-center bg-gray-50">Loading...</div>;
    }

    if (!template) {
        return <div className="min-h-screen flex items-center justify-center bg-gray-50">Intake form not found.</div>;
    }

    // We add two custom initial steps for Name and Email
    const steps = [
        { id: 'name', type_name: 'text', text: 'What is your name?' },
        { id: 'email', type_name: 'text', text: 'What is your email address?' },
        ...template.questions
    ];

    const currentQuestion = steps[currentStep];

    const handleNext = () => {
        if (currentStep < steps.length - 1) {
            setCurrentStep(currentStep + 1);
        } else {
            handleSubmit();
        }
    };

    const handleAnswerChange = (val: string) => {
        if (currentQuestion.id === 'name') setCustomerName(val);
        else if (currentQuestion.id === 'email') setCustomerEmail(val);
        else setAnswers({ ...answers, [currentQuestion.id]: val });
    };

    const getCurrentValue = () => {
        if (currentQuestion.id === 'name') return customerName;
        if (currentQuestion.id === 'email') return customerEmail;
        return answers[currentQuestion.id] || "";
    };

    const handleSubmit = async () => {
        setSubmitting(true);
        try {
            const formattedAnswers = Object.entries(answers).map(([question_id, raw_response]) => ({
                question_id,
                raw_response,
                media_url: null // simplified for now
            }));

            const response = await fetch(`/api/v1/intake/submit/${params.templateId}?tenant=my-business`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    customer_name: customerName,
                    customer_email: customerEmail,
                    answers: formattedAnswers
                })
            });

            if (response.ok) {
                setSubmitted(true);
            }
        } catch (e) {
            console.error("Submission failed", e);
        } finally {
            setSubmitting(false);
        }
    };

    if (submitted) {
        return (
            <div className="min-h-screen flex flex-col items-center justify-center bg-gray-50 p-6 font-inter">
                <div className="max-w-md w-full bg-white rounded-3xl p-10 shadow-2xl text-center backdrop-blur-xl border border-white/50">
                    <div className="w-20 h-20 bg-green-100 text-green-500 rounded-full flex items-center justify-center mx-auto mb-6 text-4xl shadow-inner">
                        ✓
                    </div>
                    <h1 className="text-3xl font-bold font-outfit mb-4 text-gray-900">Request Sent!</h1>
                    <p className="text-gray-600 mb-8 text-lg">Thanks {customerName}, we'll review your details and send you a custom quote shortly.</p>
                </div>
            </div>
        );
    }

    return (
        <div className="min-h-screen flex flex-col items-center justify-center bg-[#f8fafc] p-4 font-inter transition-colors duration-500">
            <div className="w-full max-w-lg mb-8">
                <div className="h-1.5 w-full bg-gray-200 rounded-full overflow-hidden">
                    <div
                        className="h-full bg-blue-600 transition-all duration-300 ease-out"
                        style={{ width: `${((currentStep + 1) / steps.length) * 100}%` }}
                    />
                </div>
                <div className="text-right text-xs text-gray-400 mt-2 font-medium">
                    Step {currentStep + 1} of {steps.length}
                </div>
            </div>

            <div className="max-w-lg w-full">
                <h2 className="text-3xl sm:text-4xl font-bold text-gray-900 mb-8 leading-tight font-outfit animate-fade-in-up">
                    {currentQuestion.text}
                </h2>

                <div className="animate-fade-in-up" style={{ animationDelay: '0.1s' }}>
                    {currentQuestion.type_name === 'multiple_choice' ? (
                        <div className="space-y-3">
                            {currentQuestion.options?.map((opt: string, i: number) => (
                                <button
                                    key={i}
                                    onClick={() => { handleAnswerChange(opt); setTimeout(handleNext, 300); }}
                                    className={`w-full text-left p-5 rounded-2xl border-2 transition-all duration-200 min-h-[64px] font-medium text-lg
                                        ${getCurrentValue() === opt
                                            ? 'border-blue-600 bg-blue-50 text-blue-700 shadow-md transform scale-[1.02]'
                                            : 'border-gray-200 bg-white hover:border-gray-300 hover:bg-gray-50 text-gray-700'}`}
                                >
                                    {opt}
                                </button>
                            ))}
                        </div>
                    ) : (
                        <div className="space-y-6">
                            <input
                                type={currentQuestion.id === 'email' ? 'email' : 'text'}
                                autoFocus
                                value={getCurrentValue()}
                                onChange={(e) => handleAnswerChange(e.target.value)}
                                onKeyDown={(e) => { if (e.key === 'Enter' && getCurrentValue()) handleNext() }}
                                placeholder="Type your answer here..."
                                className="w-full text-xl p-0 py-2 border-0 border-b-2 border-gray-300 bg-transparent focus:ring-0 focus:border-blue-600 transition-colors placeholder-gray-400"
                            />

                            <button
                                onClick={handleNext}
                                disabled={!getCurrentValue() || submitting}
                                className="px-8 py-4 bg-blue-600 text-white rounded-full font-semibold text-lg shadow-lg hover:bg-blue-700 hover:shadow-xl transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2 mt-4"
                            >
                                {submitting ? 'Submitting...' : (currentStep === steps.length - 1 ? 'Submit' : 'OK')}
                                {!submitting && currentStep < steps.length - 1 && <span>→</span>}
                            </button>
                        </div>
                    )}
                </div>
            </div>

            <style dangerouslySetInnerHTML={{__html: `
                @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
                .font-inter { font-family: 'Inter', sans-serif; }
                .font-outfit { font-family: 'Outfit', sans-serif; }

                @keyframes fade-in-up {
                    0% { opacity: 0; transform: translateY(20px); }
                    100% { opacity: 1; transform: translateY(0); }
                }
                .animate-fade-in-up { animation: fade-in-up 0.4s cubic-bezier(0.16, 1, 0.3, 1) forwards; }
            `}} />
        </div>
    );
}
