export type OnchainGmmContracts = {
    "version": "0.1.0",
    "name": "onchain_gmm_contracts",
    "instructions": [
      {
        "name": "createSolPool",
        "accounts": [
          {
            "name": "user",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "poolState",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "poolTokenWallet",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "position",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "wallet",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "userWalletToken",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "tokenMint",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "tokenProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "systemProgram",
            "isMut": false,
            "isSigner": false
          }
        ],
        "args": [
          {
            "name": "solAmount",
            "type": "u64"
          },
          {
            "name": "tokenAmount",
            "type": "u64"
          }
        ]
      },
      {
        "name": "createPool",
        "accounts": [
          {
            "name": "user",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "poolState",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "poolWalletToken0",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "poolWalletToken1",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "position",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "stakersList",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "userWalletToken0",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "userWalletToken1",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "token0Mint",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "token1Mint",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "tokenProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "systemProgram",
            "isMut": false,
            "isSigner": false
          }
        ],
        "args": [
          {
            "name": "tokenAAmount",
            "type": "u64"
          },
          {
            "name": "tokenBAmount",
            "type": "u64"
          },
          {
            "name": "pubkeyInvoker",
            "type": "publicKey"
          },
          {
            "name": "isSolPool",
            "type": "bool"
          }
        ]
      },
      {
        "name": "swap",
        "accounts": [
          {
            "name": "user",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "pool",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "rewardPool0For2",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "rewardPool1For2",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "poolWalletToken0",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "poolWalletToken1",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "rewardPoolWalletToken",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "stakersList",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "userWalletToken0",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "userWalletToken1",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "token0Mint",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "token1Mint",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "tokenProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "systemProgram",
            "isMut": false,
            "isSigner": false
          }
        ],
        "args": [
          {
            "name": "inputAmount",
            "type": "u64"
          },
          {
            "name": "aToB",
            "type": "bool"
          }
        ]
      }
    ],
    "accounts": [
      {
        "name": "pool",
        "type": {
          "kind": "struct",
          "fields": [
            {
              "name": "token0",
              "type": "publicKey"
            },
            {
              "name": "token1",
              "type": "publicKey"
            },
            {
              "name": "kConstant",
              "type": "u64"
            },
            {
              "name": "currentTotalEmissions",
              "type": "f64"
            },
            {
              "name": "totalStakedToken0",
              "type": "f64"
            },
            {
              "name": "totalStakedToken1",
              "type": "f64"
            }
          ]
        }
      },
      {
        "name": "position",
        "type": {
          "kind": "struct",
          "fields": [
            {
              "name": "amount",
              "type": "u16"
            },
            {
              "name": "timestamp",
              "type": "i64"
            },
            {
              "name": "currentTotalEmissions",
              "type": "f64"
            }
          ]
        }
      },
      {
        "name": "validatorList",
        "docs": [
          "Storage list for all validator stake accounts in the pool."
        ],
        "type": {
          "kind": "struct",
          "fields": [
            {
              "name": "validators",
              "docs": [
                "List of stake info for each validator in the pool"
              ],
              "type": {
                "vec": {
                  "defined": "ValidatorStakeInfo"
                }
              }
            }
          ]
        }
      }
    ],
    "types": [
      {
        "name": "ValidatorStakeInfo",
        "type": {
          "kind": "struct",
          "fields": [
            {
              "name": "token0Amount",
              "type": "i64"
            },
            {
              "name": "token1Amount",
              "type": "i64"
            },
            {
              "name": "token0Reward",
              "type": "f64"
            },
            {
              "name": "token1Reward",
              "type": "f64"
            },
            {
              "name": "owner",
              "type": "publicKey"
            },
            {
              "name": "timestamp",
              "type": "i64"
            }
          ]
        }
      }
    ]
  };
  
  export const IDL: OnchainGmmContracts = {
    "version": "0.1.0",
    "name": "onchain_gmm_contracts",
    "instructions": [
      {
        "name": "createSolPool",
        "accounts": [
          {
            "name": "user",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "poolState",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "poolTokenWallet",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "position",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "wallet",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "userWalletToken",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "tokenMint",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "tokenProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "systemProgram",
            "isMut": false,
            "isSigner": false
          }
        ],
        "args": [
          {
            "name": "solAmount",
            "type": "u64"
          },
          {
            "name": "tokenAmount",
            "type": "u64"
          }
        ]
      },
      {
        "name": "createPool",
        "accounts": [
          {
            "name": "user",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "poolState",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "poolWalletToken0",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "poolWalletToken1",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "position",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "stakersList",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "userWalletToken0",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "userWalletToken1",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "token0Mint",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "token1Mint",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "tokenProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "systemProgram",
            "isMut": false,
            "isSigner": false
          }
        ],
        "args": [
          {
            "name": "tokenAAmount",
            "type": "u64"
          },
          {
            "name": "tokenBAmount",
            "type": "u64"
          },
          {
            "name": "pubkeyInvoker",
            "type": "publicKey"
          },
          {
            "name": "isSolPool",
            "type": "bool"
          }
        ]
      },
      {
        "name": "swap",
        "accounts": [
          {
            "name": "user",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "pool",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "rewardPool0For2",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "rewardPool1For2",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "poolWalletToken0",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "poolWalletToken1",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "rewardPoolWalletToken",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "stakersList",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "userWalletToken0",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "userWalletToken1",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "token0Mint",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "token1Mint",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "tokenProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "systemProgram",
            "isMut": false,
            "isSigner": false
          }
        ],
        "args": [
          {
            "name": "inputAmount",
            "type": "u64"
          },
          {
            "name": "aToB",
            "type": "bool"
          }
        ]
      }
    ],
    "accounts": [
      {
        "name": "pool",
        "type": {
          "kind": "struct",
          "fields": [
            {
              "name": "token0",
              "type": "publicKey"
            },
            {
              "name": "token1",
              "type": "publicKey"
            },
            {
              "name": "kConstant",
              "type": "u64"
            },
            {
              "name": "currentTotalEmissions",
              "type": "f64"
            },
            {
              "name": "totalStakedToken0",
              "type": "f64"
            },
            {
              "name": "totalStakedToken1",
              "type": "f64"
            }
          ]
        }
      },
      {
        "name": "position",
        "type": {
          "kind": "struct",
          "fields": [
            {
              "name": "amount",
              "type": "u16"
            },
            {
              "name": "timestamp",
              "type": "i64"
            },
            {
              "name": "currentTotalEmissions",
              "type": "f64"
            }
          ]
        }
      },
      {
        "name": "validatorList",
        "docs": [
          "Storage list for all validator stake accounts in the pool."
        ],
        "type": {
          "kind": "struct",
          "fields": [
            {
              "name": "validators",
              "docs": [
                "List of stake info for each validator in the pool"
              ],
              "type": {
                "vec": {
                  "defined": "ValidatorStakeInfo"
                }
              }
            }
          ]
        }
      }
    ],
    "types": [
      {
        "name": "ValidatorStakeInfo",
        "type": {
          "kind": "struct",
          "fields": [
            {
              "name": "token0Amount",
              "type": "i64"
            },
            {
              "name": "token1Amount",
              "type": "i64"
            },
            {
              "name": "token0Reward",
              "type": "f64"
            },
            {
              "name": "token1Reward",
              "type": "f64"
            },
            {
              "name": "owner",
              "type": "publicKey"
            },
            {
              "name": "timestamp",
              "type": "i64"
            }
          ]
        }
      }
    ]
  };
  