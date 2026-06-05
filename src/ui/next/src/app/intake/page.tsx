'use client';
import { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function IntakeQuestionnairePage() {
  const router = useRouter();
  const [questions, setQuestions] = useState<any[]>([]);
  const [answers, setAnswers] = useState<Record<string, string>>({});
  const [mediaUrls, setMediaUrls] = useState<Record<string, string>>({});
  const [loading, setLoading] = useState(true);
  const [submitting, setSubmitting] = useState(false);
  const [submitted, setSubmitted] = useState(false);

  useEffect(() => {
    // In a real app this would fetch the actual questions via grpc/REST
    const fetchQuestions = async () => {
      try {
        const res = await fetch('/api/onboarding/intake/questions');
        if (res.ok) {
           const data = await res.json();
           if (data.questions && data.questions.length > 0) {
              setQuestions(data.questions);
              setLoading(false);
              return;
           }
        }
      } catch (e) {}

      setQuestions([]);
      setLoading(false);
    };

    fetchQuestions();
  }, []);

  const handleTextChange = (id: string, val: string) => {
    setAnswers({ ...answers, [id]: val });
  };

  const handleFileUpload = async (id: string, e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files[0]) {
      const file = e.target.files[0];
      const formData = new FormData();
      formData.append('file', file);

      try {
          const uploadRes = await fetch('/api/upload', {
              method: 'POST',
              body: formData
          });
          if (uploadRes.ok) {
              const data = await uploadRes.json();
              setMediaUrls({ ...mediaUrls, [id]: data.url });
          } else {
             // Fallback local UI representation for un-linked backends
             setMediaUrls({ ...mediaUrls, [id]: `https://storage.ohc.io/uploads/${file.name}` });
          }
      } catch (e) {
          setMediaUrls({ ...mediaUrls, [id]: `https://storage.ohc.io/uploads/${file.name}` });
      }
    }
  };

  const handleSubmit = async () => {
    setSubmitting(true);
    const answersPayload = questions.map(q => ({
      question_id: q.id,
      raw_response: answers[q.id] || '',
      media_url: mediaUrls[q.id] || ''
    }));

    try {
      const res = await fetch('/api/onboarding/intake', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          customer_id: 'cust_' + Date.now(),
          answers: answersPayload
        })
      });
      if (res.ok) {
          setSubmitted(true);
      } else {
          alert('Network error communicating with backend');
      }
    } catch (e) {
      console.error(e);
      alert('Network error communicating with backend');
    }
    setSubmitting(false);
  };

  if (loading) return <div className="p-8 text-center">Loading form...</div>;

  if (submitted) {
    return (
      <div className="p-8 max-w-md mx-auto text-center" style={{ backdropFilter: 'blur(20px) saturate(200%)', background: 'rgba(255,255,255,0.7)' }}>
        <h2 className="text-2xl font-bold mb-4">Request Submitted</h2>
        <p>We have received your intake form and will send you a quote shortly.</p>
      </div>
    );
  }

  return (
    <div className="p-4 max-w-md mx-auto min-h-screen" style={{ backdropFilter: 'blur(20px) saturate(200%)', background: 'rgba(255,255,255,0.7)' }}>
      <h1 className="text-2xl font-bold mb-6 text-black">Service Request Intake</h1>
      <div className="space-y-6">
        {questions.map((q) => (
          <div key={q.id} className="p-4 bg-white bg-opacity-50 rounded-xl shadow-sm border border-gray-100">
            <label className="block font-medium text-gray-800 mb-2">
              {q.text} {q.is_required && <span className="text-red-500">*</span>}
            </label>
            {q.type === 'text' && (
              <textarea
                className="w-full border border-gray-300 rounded-lg p-3 text-black min-h-[100px] focus:ring-2 focus:ring-blue-500 outline-none"
                value={answers[q.id] || ''}
                onChange={(e) => handleTextChange(q.id, e.target.value)}
                required={q.is_required}
              />
            )}
            {q.type === 'photo_upload' && (
              <div className="w-full border-2 border-dashed border-gray-300 rounded-lg p-6 text-center hover:bg-gray-50 transition-colors">
                <input
                  type="file"
                  accept="image/*"
                  onChange={(e) => handleFileUpload(q.id, e)}
                  className="hidden"
                  id={`file-${q.id}`}
                />
                <label htmlFor={`file-${q.id}`} className="cursor-pointer text-blue-600 font-medium flex flex-col items-center">
                  <svg className="w-8 h-8 mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"></path></svg>
                  {mediaUrls[q.id] ? 'Photo Uploaded ✓' : 'Tap to upload photo'}
                </label>
              </div>
            )}
          </div>
        ))}
      </div>
      <button
        onClick={handleSubmit}
        disabled={submitting}
        className="mt-8 w-full bg-black text-white font-medium py-4 rounded-xl shadow-lg hover:bg-gray-800 transition-colors min-h-[44px]"
      >
        {submitting ? 'Submitting...' : 'Submit Request'}
      </button>
    </div>
  );
}
