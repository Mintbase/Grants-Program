// TODO: Put these in env vars
import { Network } from "@mintbase-js/sdk";
import React from "react";

export const CONTRACT_ID = process.env.NEXT_PUBLIC_CHALLENGE_FACTORY_CONTRACT_ID;
export const NETWORK = process.env.NEXT_PUBLIC_NETWORK;

export const CONNECTION_CONFIG = {
  networkId: NETWORK!,
  nodeUrl: process.env.NEXT_PUBLIC_NODE_URL!,
  walletUrl: process.env.NEXT_PUBLIC_WALLET_URL!,
  helperUrl: process.env.NEXT_PUBLIC_HELPER_URL!,
  explorerUrl: process.env.NEXT_PUBLIC_EXPLORER_URL!,
};

export const NetworkContext = React.createContext<
  | {
      network: Network;
      callbackUrl: string;
      failureUrl: string;
      challengeFactoryContractId: string;
      networkId: string;
      nodeUrl: string;
      walletUrl: string;
      helperUrl: string;
      explorerUrl: string;
    }
  | undefined
>(undefined);
