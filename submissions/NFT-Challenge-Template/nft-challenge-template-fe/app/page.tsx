"use client";

import { useState } from "react";

import Link from "next/link";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";

export default function Home() {
  const [challengeName, setChallengeName] = useState<string>("");

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <div>
        <h1 className="text-3xl text-center font-bold my-5">NFT Challenge Generator!</h1>

        <h1 className="text-l font-bold my-5">
          Generate an nft challenge
          <Link className="text-blue-500" href={"/create-challenge"}>
            {" "}
            now!
          </Link>
        </h1>
        <h1 className="text-l font-bold my-5">Or</h1>

        <Input
          value={challengeName}
          onChange={(e) => setChallengeName(e.target.value)}
          id="nft-id"
          placeholder="Enter the the challenge id prefix"
        />
        <Link className="mt-5 block" href={`/challenges/${challengeName}`}>
          <Button>
            <h1>View an existing challenge!</h1>
          </Button>
        </Link>
      </div>
    </main>
  );
}
