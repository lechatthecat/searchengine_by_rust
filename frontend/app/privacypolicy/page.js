'use client';

import '../globals.css';
import Link from "next/link";
import { useRouter } from 'next/navigation';

function PrivacyPolicy() {
  const router = useRouter();
  return (
    <main className="min-h-screen flex flex-col items-center bg-gray-100 text-black">
      <div className="w-full max-w-3xl p-8 bg-white rounded-lg shadow-lg mt-10 mb-10 lg:px-10">
        <h1 className="text-3xl font-bold mb-6 text-center">Privacy Policy</h1>

        {/* Introduction */}
        <section className="mb-6">
          <h2 className="text-xl font-semibold mb-4">Introduction</h2>
          <p>
            We are committed to safeguarding the privacy of our website visitors. This policy sets out how we will
            treat your personal information. By using our website, you consent to the practices described in this
            policy.
          </p>
        </section>

        {/* What Data We Collect */}
        <section className="mb-6">
          <h2 className="text-xl font-semibold mb-4">What Data We Collect</h2>
          <p>We may collect, store, and use the following kinds of personal data:</p>
          <ul className="list-disc list-inside mt-4">
            <li>Your IP address and where the IP address accessed for security and analytics purposes.</li>
          </ul>
        </section>

        {/* Footer / Last updated */}
        <footer className="mt-10 text-center">
          <p>Last updated: January 4, 2025</p>
        </footer>
      </div>
      {/* Footer Section */}
      <footer className="w-full p-4 flex justify-center bg-gray-800 text-white">
        <div className="flex space-x-6">
          <Link href="/" onClick={(e) => {
            e.preventDefault();
            router.push(`/`);
          }}>
            <div className="text-white hover:underline">Home</div>
          </Link>
          <Link href="/contact" onClick={(e) => {
            e.preventDefault();
            router.push(`/contact`);
          }}>
            <div className="text-white hover:underline">Contact Me</div>
          </Link>
          <Link href="/privacypolicy" onClick={(e) => {
            e.preventDefault();
            router.push(`/privacypolicy`);
          }}>
            <div className="text-white hover:underline">Privacy Policy</div>
          </Link>
        </div>
      </footer>
    </main>
  );
}

export default PrivacyPolicy;
