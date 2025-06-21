import { Inter } from "next/font/google";
import "./globals.css";

const inter = Inter({ subsets: ["latin"] });

export const metadata = {
  title: "Dev search",
  description: "Find information on development topics with Dev Search, the search engine crafted for developers.",
};

export default function RootLayout({ children }) {
  return (
    <html lang="en">
      <head>
        {/* Include the AdSense script */}
        <script
          id="adsense-init"
          async
          src="https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js?client=ca-pub-6645780066744108"
          crossOrigin="anonymous"
          strategy="lazyOnload"
        />
      </head>
      <body className={inter.className}>{children}</body>
    </html>
  );
}
