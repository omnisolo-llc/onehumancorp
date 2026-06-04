'use client';
import { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function CustomerIntakePage({ params }: { params: { templateId: string } }) {
  const router = useRouter();
  const [template, setTemplate] = useState<any>(null);
  const [currentStep, setCurrentStep] = useState(0);
  const [answers, setAnswers] = useState<Record<string, { answer_text?: string; photo_url?: string }>>({});
  const [loading, setLoading] = useState(true);
  const [submitting, setSubmitting] = useState(false);
  const [submitted, setSubmitted] = useState(false);

  useEffect(() => {
    const fetchTemplate = async () => {
      try {
        const tenant = typeof window !== 'undefined' ? new URLSearchParams(window.location.search).get('tenant') || 'my-store' : 'my-store';
        const res = await fetch(`/api/v1/questionnaires/${params.templateId}`, {
           headers: { 'X-Tenant-ID': tenant }
        });
        if (res.ok) {
           const data = await res.json();
           setTemplate(data);
        }
      } catch (e) {
        console.error(e);
      } finally {
        setLoading(false);
      }
    };
    fetchTemplate();
  }, [params.templateId]);

  if (loading) return <div className="min-h-screen flex items-center justify-center">Loading...</div>;
  if (!template || !template.questions || template.questions.length === 0) return <div className="min-h-screen flex items-center justify-center">Form not found.</div>;

  const questions = template.questions;
  const isLastStep = currentStep === questions.length - 1;
  const currentQuestion = questions[currentStep];

  const handleNext = async () => {
     if (isLastStep) {
        setSubmitting(true);
        try {
           const formattedAnswers = Object.entries(answers).map(([qId, val]) => ({
               question_id: qId,
               answer_text: val.answer_text,
               photo_url: val.photo_url
           }));

           const tenant = typeof window !== 'undefined' ? new URLSearchParams(window.location.search).get('tenant') || 'my-store' : 'my-store';
           const res = await fetch(`/api/v1/questionnaires/${params.templateId}/submit`, {
               method: 'POST',
               headers: { 'Content-Type': 'application/json', 'X-Tenant-ID': tenant },
               body: JSON.stringify({ answers: formattedAnswers, customer_id: 'guest-customer' })
           });
           if (res.ok) {
               setSubmitted(true);
           }
        } catch (e) {
           console.error(e);
        } finally {
           setSubmitting(false);
        }
     } else {
        setCurrentStep(prev => prev + 1);
     }
  };

  const setAnswer = (val: string) => {
     setAnswers(prev => ({
         ...prev,
         [currentQuestion.id]: { ...prev[currentQuestion.id], answer_text: val }
     }));
  };

  const handlePhotoUpload = (e: any) => {
     // Mocking photo upload for UI flow
     const file = e.target.files[0];
     if (file) {
         const url = URL.createObjectURL(file);
         setAnswers(prev => ({
            ...prev,
            [currentQuestion.id]: { ...prev[currentQuestion.id], photo_url: url }
         }));
     }
  };

  if (submitted) {
     return (
        <div className="min-h-screen flex flex-col items-center justify-center p-6 bg-gray-50">
           <div className="bg-white p-8 rounded-2xl shadow-sm text-center max-w-sm w-full">
              <div className="w-16 h-16 bg-green-100 text-green-600 rounded-full flex items-center justify-center mx-auto mb-4 text-2xl">✓</div>
              <h2 className="text-2xl font-bold mb-2">Request Sent!</h2>
              <p className="text-gray-600 mb-6">Thanks for the details. We'll review your request and get back to you with a quote shortly.</p>
              <button
                 onClick={() => window.location.href = '/'}
                 className="bg-black text-white w-full py-3 rounded-xl font-medium"
              >
                 Return to Store
              </button>
           </div>
        </div>
     );
  }

  const answerVal = answers[currentQuestion.id]?.answer_text || '';
  const photoVal = answers[currentQuestion.id]?.photo_url;

  // Basic validation
  const canProceed = currentQuestion.is_required ? (currentQuestion.type === 'photo_upload' ? !!photoVal : answerVal.trim().length > 0) : true;

  return (
    <div className="min-h-screen bg-white flex flex-col">
       <div className="h-1 bg-gray-100 w-full fixed top-0 left-0">
          <div
             className="h-full bg-blue-600 transition-all duration-300 ease-out"
             style={{ width: `${((currentStep) / questions.length) * 100}%` }}
          />
       </div>

       <div className="flex-1 flex flex-col justify-center max-w-2xl w-full mx-auto p-6 md:p-12 mt-10">
          <div className="mb-8 transition-opacity duration-500">
             <span className="text-blue-600 font-medium text-sm mb-4 block">
                {currentStep + 1} of {questions.length} {currentQuestion.is_required && <span className="text-gray-400 ml-2">* Required</span>}
             </span>
             <h1 className="text-3xl md:text-4xl font-bold text-gray-900 leading-tight">
                {currentQuestion.text}
             </h1>
          </div>

          <div className="w-full mb-12">
             {currentQuestion.type === 'text' && (
                <input
                   type="text"
                   autoFocus
                   value={answerVal}
                   onChange={e => setAnswer(e.target.value)}
                   onKeyDown={e => e.key === 'Enter' && canProceed && handleNext()}
                   className="w-full text-2xl border-b-2 border-gray-300 pb-2 bg-transparent focus:border-blue-600 outline-none transition-colors"
                   placeholder="Type your answer here..."
                />
             )}

             {currentQuestion.type === 'multiple_choice' && (
                <div className="space-y-3">
                   {['Hardwood', 'Laminate', 'Carpet', 'Other'].map(opt => (
                      <button
                         key={opt}
                         onClick={() => {
                            setAnswer(opt);
                            setTimeout(handleNext, 300); // Auto advance after selection
                         }}
                         className={`w-full text-left px-6 py-4 rounded-xl border-2 text-lg transition-all ${
                            answerVal === opt
                               ? 'border-blue-600 bg-blue-50 text-blue-900'
                               : 'border-gray-200 hover:border-blue-300 hover:bg-gray-50'
                         }`}
                      >
                         {opt}
                      </button>
                   ))}
                </div>
             )}

             {currentQuestion.type === 'photo_upload' && (
                <div className="w-full">
                   {photoVal ? (
                      <div className="relative w-full h-64 rounded-xl overflow-hidden border-2 border-gray-200">
                         <img src={photoVal} className="w-full h-full object-cover" alt="Preview" />
                         <button
                            onClick={() => setAnswers(prev => ({...prev, [currentQuestion.id]: {}}))}
                            className="absolute top-2 right-2 bg-white/90 text-black px-3 py-1 rounded-lg text-sm shadow-sm backdrop-blur-sm hover:bg-white"
                         >
                            Remove
                         </button>
                      </div>
                   ) : (
                      <label className="flex flex-col items-center justify-center w-full h-48 border-2 border-dashed border-gray-300 rounded-xl cursor-pointer hover:bg-gray-50 hover:border-blue-400 transition-colors">
                         <div className="flex flex-col items-center justify-center pt-5 pb-6">
                            <svg className="w-10 h-10 mb-3 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"></path></svg>
                            <p className="mb-2 text-sm text-gray-500"><span className="font-medium">Click to upload</span> or drag and drop</p>
                            <p className="text-xs text-gray-500">PNG, JPG up to 10MB</p>
                         </div>
                         <input type="file" className="hidden" accept="image/*" onChange={handlePhotoUpload} />
                      </label>
                   )}
                </div>
             )}
          </div>

          <div className="mt-auto pt-6 flex items-center">
             <button
                onClick={handleNext}
                disabled={!canProceed || submitting}
                className={`flex items-center justify-center px-8 py-4 rounded-xl text-lg font-medium transition-all ${
                   canProceed && !submitting
                      ? 'bg-blue-600 text-white hover:bg-blue-700 shadow-md hover:shadow-lg'
                      : 'bg-gray-200 text-gray-400 cursor-not-allowed'
                }`}
             >
                {submitting ? 'Submitting...' : isLastStep ? 'Submit Request' : 'OK'}
             </button>

             {currentStep > 0 && (
                 <button
                    onClick={() => setCurrentStep(prev => prev - 1)}
                    className="ml-4 px-4 py-4 text-gray-500 hover:text-gray-800 font-medium"
                 >
                    Back
                 </button>
             )}
          </div>
       </div>
    </div>
  );
}
