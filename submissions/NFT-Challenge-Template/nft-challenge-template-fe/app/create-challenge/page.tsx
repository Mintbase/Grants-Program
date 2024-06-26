"use client";
import ChallengeCreator from "@/components/ChallengeCreator";
import { Web3Toggle } from "@/components/Web3Toggle";

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <div>
        <Web3Toggle />
        <h1 className="text-3xl text-center font-bold my-5">Create Your NFT Challenge</h1>
        <ChallengeCreator />
      </div>
    </main>
  );
}
