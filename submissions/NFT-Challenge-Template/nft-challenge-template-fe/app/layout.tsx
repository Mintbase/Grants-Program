"use client";

import { Inter } from "next/font/google";
import "./globals.css";
import "@near-wallet-selector/modal-ui/styles.css";

import { MintbaseWalletContextProvider } from "@mintbase-js/react";
import { Suspense } from "react";
import React from "react";
import { CONNECTION_CONFIG, CONTRACT_ID, NETWORK, NetworkContext } from "@/toolkit/blockchain";
import { Network } from "@mintbase-js/sdk";

const inter = Inter({ subsets: ["latin"] });

export default function RootLayout({ children }: { children: React.ReactNode }) {
  const mbContext = {
    network: (process.env.NEXT_PUBLIC_NETWORK || NETWORK) as Network,
    callbackUrl: process.env.NEXT_PUBLIC_CALLBACK_URL || (typeof window !== "undefined" ? window.location.origin : ""),
    challengeFactoryContractId: CONTRACT_ID!,
    failureUrl:
      process.env.NEXT_PUBLIC_FAILURE_URL ||
      (typeof window !== "undefined" ? `${window.location.origin}?success=false` : ""),
  };

  return (
    <MintbaseWalletContextProvider {...mbContext}>
      <html lang="en">
        <body className={inter.className}>
          <NetworkContext.Provider value={{ ...mbContext, ...CONNECTION_CONFIG }}>
            <div className=" flex-1 flex-col min-h-screen gradient w-full  h-full flex justify-center items-center bold text-white">
              <Suspense fallback={<div>Loading...</div>}>{children}</Suspense>
            </div>
          </NetworkContext.Provider>
        </body>
      </html>
    </MintbaseWalletContextProvider>
  );
}
