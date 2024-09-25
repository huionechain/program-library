// To connect to a public cluster, set `export LIVE=1` in your
// environment. By default, `LIVE=1` will connect to the devnet cluster.

import {clusterApiUrl, Cluster} from '@xoneorg/web3.js';
import dotenv from 'dotenv';

function chooseCluster(): Cluster | undefined {
  dotenv.config();
  if (!process.env.LIVE) return;
  switch (process.env.CLUSTER) {
    case 'devnet':
    case 'testnet':
    case 'mainnet-beta': {
      return process.env.CLUSTER;
    }
  }
  throw 'Unknown cluster "' + process.env.CLUSTER + '", check the .env file';
}

export const cluster = chooseCluster();

export const url =
  process.env.RPC_URL ||
  (process.env.LIVE ? clusterApiUrl(cluster, false) : 'https://rpc.huione.org');

export const urlTls =
  process.env.RPC_URL ||
  (process.env.LIVE ? clusterApiUrl(cluster, true) : 'https://rpc.huione.org');

export let walletUrl =
  process.env.WALLET_URL || 'https://rpc.huione.org';
