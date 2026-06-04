'use client';
import { useState, useEffect } from 'react';
import Link from 'next/link';

export default function ReviewIntakePage({ params }: { params: { submissionId: string } }) {
  const [submission, setSubmission] = useState<any>(null);
  const [loading, setLoading] = useState(true);
  const [approved, setApproved] = useState(false);

  useEffect(() => {
    const fetchSubmission = async () => {
      try {
        const tenant = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'my-store' : 'my-store';
        const res = await fetch(`/api/questionnaires/submissions/${params.submissionId}`, {
            headers: { 'X-Tenant-ID': tenant }
        });
        if (res.ok) {
            const data = await res.json();
            setSubmission(data);
        }
      } catch (err) {
        console.error(err);
      } finally {
        setLoading(false);
      }
    };
    fetchSubmission();
  }, [params.submissionId]);

  if (loading) return <div className="p-6">Loading AI summary...</div>;
  if (!submission) return <div className="p-6">Submission not found.</div>;

  return (
    <div className="p-4 max-w-2xl mx-auto mt-6">
      <Link href="/dashboard/intakes" className="text-blue-600 mb-4 inline-block hover:underline">&larr; Back to Intakes</Link>

      <div className="bg-white rounded-2xl shadow-sm border p-6 backdrop-blur-md bg-opacity-90 relative overflow-hidden">
        <div className="absolute top-0 right-0 bg-blue-50 text-blue-800 text-xs px-3 py-1 font-medium rounded-bl-lg">
          AI Sales Agent Analysis
        </div>

        <h1 className="text-2xl font-bold mb-2">Review Intake: Custom Request</h1>

        <div className="my-6 bg-blue-50/50 p-4 rounded-xl border border-blue-100">
           <h3 className="font-semibold text-blue-900 mb-2">✨ AI Summary</h3>
           <p className="text-gray-800">{submission.summary}</p>
        </div>

        <div className="space-y-4 mb-8">
           <h3 className="font-semibold text-lg border-b pb-2">Original Answers</h3>
           {submission.answers.map((ans: any, idx: number) => (
              <div key={idx} className="mb-3">
                 <p className="text-sm font-medium text-gray-500">{ans.question_text}</p>
                 <p className="text-gray-900">{ans.answer_text || (ans.photo_url ? 'Attached Photo' : 'No answer')}</p>
                 {ans.photo_url && (
                    <div className="mt-2 w-32 h-32 bg-gray-200 rounded-lg flex items-center justify-center text-xs text-gray-500 overflow-hidden">
                       <img src={ans.photo_url} alt="Uploaded attachment" className="w-full h-full object-cover" />
                    </div>
                 )}
              </div>
           ))}
        </div>

        <div className="border-t pt-6">
           <h3 className="font-semibold text-xl mb-4">Draft Quote</h3>
           <div className="flex justify-between items-center bg-gray-50 p-4 rounded-xl mb-6">
              <span className="text-gray-600 font-medium">Estimated Price</span>
              <span className="text-2xl font-bold">{submission.draft_quote}</span>
           </div>

           {approved ? (
              <div className="text-green-600 font-medium text-center p-3 bg-green-50 rounded-lg">
                 Quote approved and sent to customer!
              </div>
           ) : (
              <button
                 onClick={() => setApproved(true)}
                 className="w-full bg-black text-white font-medium py-3 rounded-xl hover:bg-gray-800 transition"
              >
                 Review Quote & Send
              </button>
           )}
        </div>
      </div>
    </div>
  );
}
