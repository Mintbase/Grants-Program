import NFTForm from "@/components/forms/NFTForm";
import { NFTContract, NFTMetaData } from "@/types/nft";
import { useContext, useEffect, useState } from "react";
import { VictoryConditionsForm } from "./forms/VictoryConditionsForm";
import { TerminationRulesForm } from "./forms/TerminationRulesForm";
import { Button } from "./ui/button";
import { NearWalletConnector } from "./NearWalletSelector";
import { useMbWallet } from "@mintbase-js/react";
import RewardNFTForm from "./forms/RewardNFTForm";
import { NetworkContext } from "@/toolkit/blockchain";

export enum Progress {
  ChallengeDetails,
  RewardNFTDetails,
  SetChallenges,
  SetTerminationRules,
  CreateContract,
  ChallengeCreated,
}

export const MAX_INT = "18000000000000000000";

export default function ChallengeCreator() {
  const [progress, setProgress] = useState<Progress>(Progress.ChallengeDetails);
  // Reward Info
  const [rewardNft, setRewardNft] = useState<NFTContract | undefined>(undefined);
  const [rewardTitle, setRewardTitle] = useState<string | undefined>(undefined);
  const [rewardDesc, setRewardDesc] = useState<string | undefined>(undefined);
  const [rewardMediaLink, setRewardMediaLink] = useState<string | undefined>(undefined);

  const [name, setName] = useState<string | undefined>(undefined);
  const [desc, setDesc] = useState<string | undefined>(undefined);
  const [idPrefix, setIdPrefix] = useState<string | undefined>(undefined);
  const [mediaLink, setMediaLink] = useState<string | undefined>(undefined);
  const [challengeNftIds, setChallengeNftIds] = useState<Array<string>>([""]);
  const [burnChallengeNftId, setBurnChallengeNftId] = useState<Array<boolean>>([false]);
  const [terminationDate, setTerminationDate] = useState<Date | undefined>(undefined);
  const [creatorCanEndChallenge, setCreatorCanEndChallenge] = useState(false);
  const [winnerCount, setWinnerCount] = useState<undefined | number>(undefined);
  const [maxProgress, setMaxProgress] = useState(Progress.ChallengeDetails);
  const { isConnected, selector } = useMbWallet();
  const { challengeFactoryContractId } = useContext(NetworkContext)!;

  useEffect(() => {
    if (rewardNft != null) {
      setProgress(Progress.SetChallenges);
    }
  }, [rewardNft]);

  useEffect(() => {
    if (progress > maxProgress) {
      setMaxProgress(progress);
    }
  }, [progress]);
  useEffect(() => {
    if (!isConnected) {
      setProgress(Progress.ChallengeDetails);
    }
  });

  const onSubmit = async () => {
    const wallet = await selector.wallet();
    if (!isConnected) return false;

    let terminationDateStr = terminationDate?.getTime().toString();
    if (terminationDate == null) {
      terminationDateStr = MAX_INT.toString();
    } else {
      terminationDateStr = terminationDateStr + "000000";
    }

    const args = {
      id_prefix: idPrefix,
      name,
      description: desc,
      media_link: mediaLink,
      reward_nft_id: rewardNft!.id,
      challenge_nft_ids: challengeNftIds,
      _expiration_date_in_ns: terminationDateStr,
      _winner_limit: winnerCount?.toString() || MAX_INT,
      creator_can_update: creatorCanEndChallenge,
      burn_challenge_piece_on_claim: burnChallengeNftId,
      reward_nft_metadata: {
        title: rewardTitle!,
        description: rewardDesc!,
        media: rewardMediaLink!,
      } as NFTMetaData,
    };

    const res = await wallet.signAndSendTransaction({
      receiverId: challengeFactoryContractId,
      actions: [
        {
          type: "FunctionCall",
          params: {
            methodName: "create_challenge",
            args,
            gas: "90000000000000",
            deposit: "1000000000000000000000000",
          },
        },
      ],
      callbackUrl: `${window.location.origin}/challenges/${idPrefix}`,
    });

    if (res != null) {
      setProgress(Progress.ChallengeCreated);
    }
  };

  const prefix =
    progress > Progress.ChallengeDetails ? (
      <div className="flex items-start space-x-4 my-5 ">
        {mediaLink != null ? (
          <img
            alt="NFT Icon"
            className="rounded-md"
            height="120"
            src={mediaLink}
            style={{
              aspectRatio: "80/80",
              objectFit: "cover",
            }}
            width="120"
          />
        ) : (
          <img
            alt="NFT Icon"
            className="rounded-md"
            height="120"
            src={rewardNft?.icon}
            style={{
              aspectRatio: "80/80",
              objectFit: "cover",
            }}
            width="120"
          />
        )}
        <div className={`grid-1 gap-1 ${rewardNft == null ? "hidden" : ""}`}>
          <div className="text-lg font-medium">{name}</div>

          <div className="flex items-center gap-10 text-sm text-gray-500 dark:text-gray-400">
            {/* <PuzzleIcon className="w-4 h-4" /> */}
            <span>Reward: {rewardNft?.name}</span>
          </div>
          {maxProgress >= Progress.SetChallenges && (
            <div className="flex items-center gap-10 text-sm text-gray-500 dark:text-gray-400">
              <span>
                {winnerCount == null ? "Unlimited" : winnerCount} winner{winnerCount && winnerCount > 1 ? "s" : ""}
              </span>
            </div>
          )}
          {challengeNftIds.length > 0 && (
            <div className="flex items-center gap-10 text-sm text-gray-500 dark:text-gray-400">
              <span>
                {challengeNftIds.length} Challenge{challengeNftIds.length > 1 ? "s" : ""}
              </span>
            </div>
          )}
          {maxProgress >= Progress.SetTerminationRules && (
            <div className="flex items-center gap-10 text-sm text-gray-500 dark:text-gray-400">
              <span>{terminationDate != null ? `Ends on ${terminationDate.toLocaleString()}` : `Never ends`}</span>
            </div>
          )}
          {maxProgress >= Progress.SetTerminationRules && (
            <div className="flex items-center gap-10 text-sm text-gray-500 dark:text-gray-400">
              <span>Creator can{!creatorCanEndChallenge && "not"} end challenge</span>
            </div>
          )}
        </div>
      </div>
    ) : null;

  switch (progress) {
    case Progress.ChallengeDetails:
      return !isConnected ? (
        <div className="flex flex-col items-center justify-center">
          <div className="mb-6">You&apos;ll need to connect your NEAR wallet to create a challenge.</div>
          <NearWalletConnector />
        </div>
      ) : (
        <div>
          {prefix}
          <NFTForm
            name={name}
            setName={setName}
            desc={desc}
            setDesc={setDesc}
            idPrefix={idPrefix}
            setIdPrefix={setIdPrefix}
            mediaLink={mediaLink}
            setMediaLink={setMediaLink}
            setProgress={setProgress}
          />
        </div>
      );
    case Progress.RewardNFTDetails:
      return (
        <div>
          {prefix}
          <RewardNFTForm
            rewardNft={rewardNft}
            setRewardNft={setRewardNft}
            rewardTitle={rewardTitle}
            setRewardTitle={setRewardTitle}
            rewardDescription={rewardDesc}
            setRewardDescription={setRewardDesc}
            rewardMediaLink={rewardMediaLink}
            setRewardMediaLink={setRewardMediaLink}
            setProgress={setProgress}
          />
        </div>
      );

    case Progress.SetChallenges:
      return (
        <div>
          {prefix}
          <VictoryConditionsForm
            challengeNftIds={challengeNftIds}
            setChallengeNftIds={setChallengeNftIds}
            setProgress={setProgress}
            winnerCount={winnerCount}
            setWinnerCount={setWinnerCount}
            burnChallengeNftId={burnChallengeNftId}
            setBurnChallengeNftId={setBurnChallengeNftId}
          />
        </div>
      );
    case Progress.SetTerminationRules:
      return (
        <div>
          {prefix}
          <TerminationRulesForm
            setProgress={setProgress}
            terminationDate={terminationDate}
            setTerminationDate={setTerminationDate}
            creatorCanEndChallenge={creatorCanEndChallenge}
            setCreatorCanEndChallenge={setCreatorCanEndChallenge}
          />
        </div>
      );
    case Progress.CreateContract:
      return (
        <div>
          {prefix}
          <div className="mt-6 flex justify-between">
            <Button onClick={() => setProgress(Progress.SetTerminationRules)} variant="outline">
              Previous
            </Button>
            <Button className="ml-4" onClick={onSubmit}>
              Confirm
            </Button>
          </div>
        </div>
      );
    case Progress.ChallengeCreated:
      return (
        <div>
          {prefix}
          <div className="mt-6 flex justify-between">Congrats! You can find your challenge at</div>
        </div>
      );
  }
}
