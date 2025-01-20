use solana_idlgen::idlgen;
idlgen!({
  "address": "ADcaide4vBtKuyZQqdU689YqEGZMCmS4tL35bdTv9wJa",
  "metadata": {
    "name": "Turbin3_prereq",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "complete",
      "discriminator": [0, 77, 224, 147, 136, 25, 88, 76],
      "accounts": [
        {
          "name": "signer",
          "writable": true,
          "isMut": true,
          "signer": true
        },
        {
          "name": "prereq",
          "isMut": true,
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [112, 114, 101, 114, 101, 113]
              },
              {
                "kind": "account",
                "path": "signer"
              }
            ]
          }
        },
        {
          "name": "system_program",
          "isMut": false,
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "github",
          "type": "bytes"
        }
      ]
    },
    {
      "name": "update",
      "discriminator": [219, 200, 88, 176, 158, 63, 253, 127],
      "accounts": [
        {
          "name": "signer",
          "writable": true,
          "isMut": true,
          "signer": true
        },
        {
          "name": "prereq",
          "isMut": true,
          "writable": true
        },
        {
          "name": "system_program",
          "isMut": false,
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "github",
          "type": "bytes"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "SolanaCohort5Account",
      "discriminator": [167, 81, 85, 136, 32, 169, 137, 77]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidGithubAccount",
      "msg": "Invalid Github account"
    }
  ],
  "types": [
    {
      "name": "SolanaCohort5Account",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "github",
            "type": "bytes"
          },
          {
            "name": "key",
            "type": "pubkey"
          }
        ]
      }
    }
  ]
}

);
