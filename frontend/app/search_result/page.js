'use client';

import '../globals.css';
import Link from "next/link";
import { useState, useEffect, Suspense } from "react";
import { useSearchParams, useRouter } from 'next/navigation';
import { FiSearch } from 'react-icons/fi';

function SearchResults() {
  const router = useRouter();
  const searchParams = useSearchParams();

  // Constants
  const RESULTS_PER_PAGE = 20;

  // Read initial search term from URL (?s=...)
  const initialSearchTerm = searchParams.get('s') || '';

  // State for input box
  const [inputValue, setInputValue] = useState(initialSearchTerm);

  // State for entire list of results (we'll append new ones)
  const [results, setResults] = useState([]);

  // We track the current offset in state (not in the URL)
  const [from, setFrom] = useState(0);

  const [noMoreResults, setNoMoreResults] = useState(false);
  const [searchAfter, setSearchAfter] = useState(null);

  // Loading states:
  //  - "loading" = any data fetch is in progress
  //  - "appending" = specifically for Load More
  const [loading, setLoading] = useState(false);
  const [appending, setAppending] = useState(false);
  const [error, setError] = useState(null);

  /**
   * On first mount, if there's already an 's' param, do an initial fetch.
   * (But only if it’s not empty.)
   */
  useEffect(() => {
    if (initialSearchTerm.trim()) {
      doSearch(initialSearchTerm, 0, false, searchAfter);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  /**
   * Main fetch function to get data from backend/Elasticsearch.
   * @param {string} searchTerm - The user’s search query.
   * @param {number} offset     - The "from" position (like ES offset).
   * @param {boolean} append    - Whether to append or replace existing results.
   * @param {string} after  - Optional setsa parameter for pagination.
   */
  const doSearch = async (searchTerm, offset, append, after) => {
    // If we're appending, show the "Loading more..." spinner at the bottom
    setAppending(append);

    // If we’re doing a brand-new search (not appending), show the main loading
    setLoading(!append);
    setError(null);

    try {
      // const url = `http://localhost:8000/api/search?s=${encodeURIComponent(searchTerm)}&f=${offset}&size=${RESULTS_PER_PAGE}&v=1.03${after ? `&sa=${encodeURIComponent(after)}` : ''}`;
      const url = `/api/search?s=${encodeURIComponent(searchTerm)}&f=${offset}&size=${RESULTS_PER_PAGE}&v=1.03${after ? `&sa=${encodeURIComponent(after)}` : ''}`;
      const response = await fetch(url);

      if (!response.ok) {
        throw new Error('Network response was not ok');
      }

      const res = await response.json();
      const searchResult = JSON.parse(res.msg);

      if (searchResult.hits.length !== 0) {
        // Append new hits to the existing array
        setResults((prev) => [...prev, ...searchResult.hits]);
        const lastSort = searchResult.hits[(searchResult.hits.length - 1)];
        setSearchAfter(lastSort.sort);
      }

      if (append) {
        if (searchResult.hits.length === 0) {
          // no new items were fetched
          setNoMoreResults(true);
        }
      } else {
        // brand-new search
        setResults(searchResult.hits);
        // reset the noMoreResults flag if we do get results
        setNoMoreResults(searchResult.hits.length === 0);
      }
    } catch (err) {
      console.error('Error fetching data:', err);
      setError(err);
    } finally {
      setLoading(false);
      setAppending(false);
    }
  };

  /**
   * Handle the user clicking "Search".
   * Reset offset to 0, replace existing results,
   * update the URL param (?s=...) without scrolling to top.
   */
  const handleSearch = (e) => {
    e.preventDefault();
    const sanitizedQuery = inputValue.trim();
    if (!sanitizedQuery) return;

    setFrom(0);
    setSearchAfter(null);
    setResults([]); // Start fresh
    router.push(`?s=${encodeURIComponent(sanitizedQuery)}`, {
      shallow: true,
      scroll: false, // Do NOT scroll to top
    });

    doSearch(sanitizedQuery, 0, false, searchAfter);
  };

  /**
   * Handle "Load more" to fetch the next batch of results.
   * We do *not* change the URL or scroll position.
   */
  const handleLoadMore = () => {
    const sanitizedQuery = inputValue.trim();
    if (!sanitizedQuery) return;

    const newOffset = from + RESULTS_PER_PAGE;
    setFrom(newOffset);

    // Append results at the bottom
    doSearch(sanitizedQuery, newOffset, true, searchAfter);
  };

  return (
    <main className="min-h-screen flex flex-col bg-gray-100">
      {/* Main content container */}
      <div className="flex flex-col lg:flex-row lg:justify-center lg:space-x-6 max-w-7xl mx-auto w-full p-4 text-black">
        <div className="flex-grow lg:w-3/4">
          {/* Logo / Title */}
          <Link href="/" className="pt-1 pb-1 font-mono text-xl font-bold text-black">
            Dev Search
          </Link>

          {/* Search Form */}
          <form className="w-full max-w-2xl mb-6 relative" onSubmit={handleSearch}>
            <input
              type="text"
              placeholder="Search without being tracked"
              value={inputValue}
              onChange={(e) => setInputValue(e.target.value)}
              className="w-full p-4 pr-12 border border-gray-300 rounded-full shadow-md focus:outline-none text-black"
            />
            <button
              type="submit"
              className="absolute right-4 top-1/2 transform -translate-y-1/2 text-gray-500"
            >
              <FiSearch size={24} />
            </button>
          </form>

          <div className="mt-4">
            {/* Error or no results */}
            {error && (
              <div className="p-4 bg-white rounded-lg shadow-md">
                <p className="text-red-500">{error.message}</p>
              </div>
            )}

            {/* If no results yet and not loading/error */}
            {!loading && results.length === 0 && !error && (
              <div className="p-4 bg-white rounded-lg shadow-md">
                <p>No results found</p>
              </div>
            )}

            {/* If we do have results, show them */}
            {results.length > 0 && (
              <>
                {results.map((result, index) => (
                  <Link href={result.url} key={index} legacyBehavior>
                    <a
                      target="_blank"
                      rel="noopener noreferrer"
                      className="block p-4 mb-4 bg-white rounded-lg shadow-md"
                    >
                      <h2 className="text-xl font-bold">{result.title}</h2>
                      <p>{result.url}</p>
                      {result.highlight?.content && (
                        <p
                          className="text-sm text-gray-700"
                          dangerouslySetInnerHTML={{ __html: result.highlight.content }}
                        />
                      )}
                    </a>
                  </Link>
                ))}

                {/* "Load more" button */}
                {!noMoreResults && (
                  <div className="flex justify-center mb-6">
                    <button
                      onClick={handleLoadMore}
                      // Disable the button if either loading or appending is true
                      disabled={loading || appending}
                      className={`px-4 py-2 bg-blue-500 text-white font-semibold rounded 
                    ${loading || appending ? 'opacity-50 cursor-not-allowed' : 'hover:bg-blue-600'}
                  `}
                    >
                      More results
                    </button>
                  </div>
                )}
              </>
            )}

            {/* If we are NOT appending, but are in the middle of a brand-new search, show "Loading..." */}
            {loading && !appending && (
              <div className="p-4 bg-white rounded-lg shadow-md">
                <p className="text-gray-700">Loading...</p>
              </div>
            )}

            {/* If we are appending, show "Loading more..." at the bottom */}
            {appending && (
              <div className="p-4 bg-white rounded-lg shadow-md">
                <p className="text-gray-700">Loading more...</p>
              </div>
            )}

          </div>
        </div>
      </div>

      {/* Footer */}
      <footer className="w-full p-4 flex justify-center bg-gray-800 text-white">
        <div className="flex space-x-6">
          <Link href="/" className="hover:underline">Home</Link>
          <Link href="/contact" className="hover:underline">Contact Me</Link>
          <Link href="/privacypolicy" className="hover:underline">Privacy Policy</Link>
        </div>
      </footer>
    </main>
  );
}

export default function SearchPage() {
  return (
    <Suspense fallback={<div>Loading search results...</div>}>
      <SearchResults />
    </Suspense>
  );
}
