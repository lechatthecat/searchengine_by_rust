'use client';

import '../globals.css';
import Link from "next/link";
import { useRouter } from 'next/navigation';

function Contact() {
  const router = useRouter();
  return (
    // 1) Use min-h-screen + flex + flex-col so we can push the footer down
    <main className="min-h-screen flex flex-col bg-gray-100">
      {/* 2) Make this wrapper flex-1 so it grows, pushing footer to bottom */}
      <div className="flex-1 flex flex-col items-center justify-center">
        <div className="w-full max-w-2xl p-8 bg-white rounded-lg shadow-lg">
          <h1 className="text-3xl font-bold mb-6 text-center">Contact Me</h1>
          <p className="text-lg text-gray-700 mb-6 text-center">
            If you have any questions, feel free to reach out to me at:
          </p>

          <div className="flex justify-center mb-6">
            <a
              href="mailto:test@gmail.com"
              className="text-blue-500 text-xl font-semibold hover:underline"
            >
              ---- @ gmail.com
            </a>
          </div>
        </div>
      </div>

      {/* 3) Footer goes last, with no "justify-center" or "items-center" 
             so it stays at the bottom */}
      <footer className="w-full p-4 flex justify-center bg-gray-800 text-white">
        <div className="flex space-x-6">
          <Link
            href="/"
            onClick={(e) => {
              e.preventDefault();
              router.push(`/`);
            }}
          >
            <div className="text-white hover:underline">Home</div>
          </Link>
          <Link
            href="/contact"
            onClick={(e) => {
              e.preventDefault();
              router.push(`/contact`);
            }}
          >
            <div className="text-white hover:underline">Contact Me</div>
          </Link>
          <Link
            href="/privacypolicy"
            onClick={(e) => {
              e.preventDefault();
              router.push(`/privacypolicy`);
            }}
          >
            <div className="text-white hover:underline">Privacy Policy</div>
          </Link>
        </div>
      </footer>
    </main>
  );
}

export default Contact;
