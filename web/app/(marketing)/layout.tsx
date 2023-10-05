import React from "react";

import { Footer } from "@/components/Footer";
import { Header } from "@/components/marketing/Header";

import type { Metadata } from "next";

interface MarketingLayoutProps {
  children: React.ReactNode;
}

export default async function MarketingLayout({
  children,
}: MarketingLayoutProps) {
  const res = await fetch(
    "https://api.github.com/repos/tierrun/tier-vercel-openai",
    {
      method: "GET",
      next: { revalidate: 60 },
    }
  );
  const data = await res.json();

  const stargazers_count: number = data.stargazers_count;

  console.log(data);
  return (
    <>
      <Header stargazers_count={stargazers_count} />
      <main>{children}</main>
      <Footer />
    </>
  );
}

// import './globals.css'
// import type { Metadata } from 'next'
// import { Inter } from 'next/font/google'

// const inter = Inter({ subsets: ['latin'] })

// export const metadata: Metadata = {
//   title: 'Create Next App',
//   description: 'Generated by create next app',
// }

// export default function RootLayout({
//   children,
// }: {
//   children: React.ReactNode
// }) {
//   return (
//     <html lang="en">
//       <body className={inter.className}>{children}</body>
//     </html>
//   )
// }
