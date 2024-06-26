"use server";

import { MongoClient, ServerApiVersion } from "mongodb";

export async function create(account_id: String) {
  try {
    const uri =
      "mongodb+srv://shakiran:uEc1J7Zx8plaiHdT@scavenger-hunt.mpwg8ad.mongodb.net/?retryWrites=true&w=majority&appName=scavenger-hunt";
    const options = {
      serverApi: {
        version: ServerApiVersion.v1,
        strict: true,
        deprecationErrors: true,
      },
    };
    const client = await new MongoClient(uri, options).connect();
    const db = client.db("scavenger-hunt");
    const movies = await db.collection("winners").insertOne({
      account_id: account_id,
      timeCreated: new Date().toString(),
    });
  } catch (e) {
    console.error(e);
  }
}

export async function getNumOfWinners() {
  const uri =
    "mongodb+srv://shakiran:uEc1J7Zx8plaiHdT@scavenger-hunt.mpwg8ad.mongodb.net/?retryWrites=true&w=majority&appName=scavenger-hunt";

  const client = await new MongoClient(uri).connect();
  const db = client.db("scavenger-hunt");
  const count = await db.collection("winners").countDocuments();
  return count;
}

export async function checkIfAccountIsWinner(account_id: String) {
  const uri =
    "mongodb+srv://shakiran:uEc1J7Zx8plaiHdT@scavenger-hunt.mpwg8ad.mongodb.net/?retryWrites=true&w=majority&appName=scavenger-hunt";

  const client = await new MongoClient(uri).connect();
  const db = client.db("scavenger-hunt");
  const res = await db.collection("winners").findOne({
    account_id,
  });

  return res != null;
}
