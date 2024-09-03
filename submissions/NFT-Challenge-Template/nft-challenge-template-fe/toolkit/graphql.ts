import { NFTContract } from "@/types/nft";
import { fetchGraphQl } from "@mintbase-js/data";
import { Network } from "@mintbase-js/sdk";
import _ from "lodash";

const query = `
query NftContract($_eq: String = "") {
  nft_contracts(where: {id: {_eq: $_eq}}) {
    category
    created_at
    id
    icon
    name
    owner_id
    symbol
  }
}
`;

export async function fetchNftContract(id: string, network: Network): Promise<NFTContract | undefined> {
  const res: {
    data?: {
      nft_contracts: Array<NFTContract>;
    };
  } = await fetchGraphQl({ query, variables: { _eq: id }, network });
  return res.data?.nft_contracts[0];
}

export async function fetchNftContracts(nftIds: ReadonlyArray<string>, network: Network): Promise<Array<NFTContract>> {
  let frequencies = nftIds.reduce(function (value, value2) {
    return value[value2] ? ++value[value2] : (value[value2] = 1), value;
  }, {} as Record<string, number>);
  const res: {
    data?: {
      nft_contracts: Array<NFTContract>;
    };
  } = await fetchGraphQl({
    query: `
    query NFTContractsQuery($_in: [String!] = "") {
      nft_contracts(where: {id: {_in: $_in}}) {
        category
        created_at
        id
        icon
        name
        owner_id
        symbol
      }
    }`,
    variables: { _in: nftIds },
    network,
  });
  if (res.data == null) return [];

  let challengeNftIds: Array<NFTContract> = [];
  for (const nft of res.data.nft_contracts) {
    for (let i = 0; i < frequencies[nft.id]; i++) {
      challengeNftIds = [...challengeNftIds, nft];
    }
  }
  return challengeNftIds;
}

// record how much gas is
