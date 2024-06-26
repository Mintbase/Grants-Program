export type NFTContract = {
  category?: string;
  created_at?: Date;
  icon?: string;
  id: string;
  name: string;
  symbol: string;
  owner_id: string;
  owned?: boolean;
  count: number | undefined;
};

// snake case cause it's pulled directly from blockchain
export type RawNFTChallengeMetaData = {
  challenge_completed: string;
  challenge_nft_ids: ReadonlyArray<string>;
  description: string;
  media_link: string;
  name: string;
  owner_id: string;
  reward_nft_id: string;
  reward_nft_metadata: NFTMetaData;
  expiration_date_in_ns: number;
  winner_limit: number;
  winners_count: number;
};

export type NFTChallengeMetaData = {
  challengeCompleted: string;
  challengeNftIds: ReadonlyArray<string>;
  description: string;
  mediaLink: string;
  name: string;
  ownerId: string;
  rewardNftId: string;
  expirationDateInMs: number | null;
  winnerLimit: number | null;
  winnerCount: number;
};

export type NFTMetaData = {
  /// the Title for this token. ex. "Arch Nemesis: Mail Carrier" or "Parcel 5055"
  title: String;
  /// free-form description of this token.
  description: String;
  /// URL to associated media, preferably to decentralized, content-addressed storage
  media: String;
  /// Base64-encoded sha256 hash of content referenced by the `media` field.
  /// Required if `media` is included. Type may be wrong
  media_hash?: String;
  /// number of copies of this set of metadata in existence when token was minted.
  copies?: number;
  /// ISO 8601 datetime when token expires.
  expires_at?: String;
  /// ISO 8601 datetime when token starts being valid.
  starts_at?: String;
  /// When token was last updated, Unix epoch in milliseconds
  extra?: String;
  /// URL to an off-chain JSON file with more info. The Mintbase Indexer refers
  /// to this field as `thing_id` or sometimes, `meta_id`.
  reference?: String;
  /// Base64-encoded sha256 hash of JSON from reference field. Required if
  /// `reference` is included.
  reference_hash?: String;
};
