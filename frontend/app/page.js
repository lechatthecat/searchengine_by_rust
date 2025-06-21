'use client'
import './globals.css';
import { useState } from "react";
import { useRouter } from 'next/navigation';
import Link from 'next/link'; // Import Link for navigation
import { FiSearch } from 'react-icons/fi'; // Import search icon from react-icons

export default function Home() {
  const [searchTerm, setSearchTerm] = useState('');
  const router = useRouter();

  const handleChange = (e) => {
    setSearchTerm(e.target.value);
  };

  const handleSubmit = (e) => {
    e.preventDefault();
    if (searchTerm) {
      router.push(`/search_result?s=${encodeURIComponent(searchTerm)}&f=0&page=1`);
    }
  };

  return (
    <main className="min-h-screen h-screen flex flex-col items-center justify-between bg-gray-100">
      <div className="w-full flex flex-col items-center search-box">
        <div className="p-4 pb-1 font-mono text-3xl font-bold text-black">
          Dev Search
        </div>
        <div className="p-2 pb-1 font-mono text-lg mb-3 text-center sm:text-left text-black">
          Find the best <br className="block sm:hidden text-black" /> dev information
        </div>
        <form
          className="w-full max-w-lg mt-2 lg:w-1/2 md:w-3/5 sm:w-4/5 relative"
          onSubmit={handleSubmit}
        >
          <input
            type="text"
            placeholder="Search..."
            className="search-input w-full p-4 pr-12 border border-gray-300 rounded-full shadow-md focus:outline-none text-black"
            value={searchTerm}
            onChange={handleChange}
          />
          <button
            type="submit"
            className="absolute right-4 top-1/2 transform -translate-y-1/2 text-gray-500"
          >
            <FiSearch size={24} />
          </button>
        </form>
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
