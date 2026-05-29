export default function Payments() {
  return (
    <div className="max-w-3xl mx-auto p-6 mt-10 bg-white rounded-xl shadow-sm border border-gray-100 font-inter">
      <h1 className="text-3xl font-extrabold font-outfit text-gray-900 mb-6">Getting Paid</h1>

      <p className="text-gray-700 mb-4 leading-relaxed text-lg">
        Getting paid is the most exciting part! We make it secure and easy for your customers to pay you.
      </p>

      <h2 className="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Connecting Your Bank Account</h2>
      <p className="text-gray-700 mb-4">
        To start taking money, you need to connect a bank account. We use Stripe, a safe and trusted system. Just click the "Connect Stripe" button in your setup to securely link your bank.
      </p>

      <h2 className="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Viewing Your Deposits</h2>
      <p className="text-gray-700 mb-4">
        When a customer buys something, the money goes into your connected bank account. You can check the Dashboard to see your recent sales and see when the money will arrive in your bank.
      </p>

      <h2 className="text-2xl font-bold font-outfit text-gray-800 mt-8 mb-4">Taxes and Fees</h2>
      <p className="text-gray-700 mb-4">
        We help handle simple taxes for you at checkout. A small fee is taken out of each sale to cover the cost of securely moving the money from the customer's card to your bank.
      </p>
    </div>
  );
}