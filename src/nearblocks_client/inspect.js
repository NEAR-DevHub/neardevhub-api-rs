let nearblocks_response = {
  cursor: "9968066800",
  txns: [
    {
      id: "11369954394",
      receipt_id: "GB6H5dC3neD411G78oA6TzBqXL6ZGoUF7xyNWabmLkF6",
      predecessor_account_id: "megha19.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "8P7n37frTjCK5cZ5itgaUj1NVcTHxaPpJzgam3UyphpK",
        block_height: 135106178,
        block_timestamp: 1734344956043450600,
      },
      receipt_outcome: {
        gas_burnt: 4319833044195,
        tokens_burnt: 431983304419500000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "CPVEWWKn2XwMyR2mWg5y8ftsL2sS6Up9SLMfeHpLTQXu",
      included_in_block_hash: "7Kkjf9n9qyX4kRJQ6pivB5fjyGXGZGWkRpyhc3zxK3WY",
      block_timestamp: "1734344954790034570",
      block: {
        block_height: 135106177,
      },
      receipt_conversion_tokens_burnt: "48687343125000000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "add_proposal",
          deposit: 0,
          fee: 431983304419500000000,
          args: '{"proposal": {"kind": {"ChangePolicy": {"policy": {"roles": [{"kind": {"Group": ["theori.near", "freski.near", "megha19.near", "frol.near", "thomasguntenaar.near", "petersalomonsen.near"]}, "name": "Create Requests", "permissions": ["call:AddProposal", "*:VoteRemove", "transfer:AddProposal", "config:Finalize"], "vote_policy": {"call": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "vote": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "config": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "policy": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "transfer": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "add_bounty": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "bounty_done": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "upgrade_self": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "set_vote_token": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "upgrade_remote": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "add_member_to_role": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "remove_member_from_role": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}}}, {"kind": {"Group": ["freski.near", "petersalomonsen.near", "thomasguntenaar.near", "theori.near", "megha19.near", "frol.near"]}, "name": "Manage Members", "permissions": ["config:*", "policy:*", "add_member_to_role:*", "policy_update_parameters:AddProposal", "policy_add_or_update_role:AddProposal", "remove_member_from_role:*"], "vote_policy": {"call": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "vote": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "config": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "policy": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "transfer": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "add_bounty": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "bounty_done": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "upgrade_self": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "set_vote_token": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "upgrade_remote": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "add_member_to_role": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "remove_member_from_role": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}}}, {"kind": {"Group": ["freski.near", "megha19.near", "meghagoel.near", "petersalomonsen.near", "frol.near"]}, "name": "Vote", "permissions": ["*:VoteReject", "*:VoteApprove", "*:VoteRemove"], "vote_policy": {"call": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "vote": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "config": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "policy": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "transfer": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "add_bounty": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "bounty_done": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "upgrade_self": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "set_vote_token": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "upgrade_remote": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "add_member_to_role": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "remove_member_from_role": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}}}], "bounty_bond": "100000000000000000000000", "proposal_bond": "0", "proposal_period": "864000000000000", "default_vote_policy": {"quorum": "1", "threshold": "1", "weight_kind": "RoleWeight"}, "bounty_forgiveness_period": "604800000000000"}}}, "description": "Update Policy"}}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 480670647544500000000,
      },
    },
    {
      id: "11368042777",
      receipt_id: "78i1QinhhM8eK4FTETRQ4jBhihyhzrYtT6PKFWwvtrGm",
      predecessor_account_id: "testing-astradao.sputnik-dao.near",
      receiver_account_id: "megha19.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "6zYN2GjGof9yiibazpzMQP7sosjcev12vBH4Amua4yBg",
        block_height: 135102877,
        block_timestamp: 1734340801646546200,
      },
      receipt_outcome: {
        gas_burnt: 223182562500,
        tokens_burnt: 22318256250000000000,
        executor_account_id: "megha19.near",
        status: true,
      },
      transaction_hash: "Hzh3LJnDHJFBLRKT2e5e7YfTjrmh66wVpK9GCkR9Db6t",
      included_in_block_hash: "Gx2GGFrbpiJtTL4RctbLzMGFCpy4Q3A78143affug4cg",
      block_timestamp: "1734340799052999087",
      block: {
        block_height: 135102875,
      },
      receipt_conversion_tokens_burnt: "31015758346000000000",
      actions: [
        {
          action: "TRANSFER",
          method: null,
          deposit: 0,
          fee: 22318256250000000000,
          args: null,
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 381145543250700000000,
      },
    },
    {
      id: "11368041967",
      receipt_id: "2JG9P6wTKM3x6rtiKCwc2fJF8TnjdngJuyTvmnz1vEhw",
      predecessor_account_id: "megha19.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "C49xXh6q57ABPg15BRMwT7svt71De8v1QcdHBm19fwbv",
        block_height: 135102876,
        block_timestamp: 1734340800346017500,
      },
      receipt_outcome: {
        gas_burnt: 3278115286547,
        tokens_burnt: 327811528654700000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "Hzh3LJnDHJFBLRKT2e5e7YfTjrmh66wVpK9GCkR9Db6t",
      included_in_block_hash: "Gx2GGFrbpiJtTL4RctbLzMGFCpy4Q3A78143affug4cg",
      block_timestamp: "1734340799052999087",
      block: {
        block_height: 135102875,
      },
      receipt_conversion_tokens_burnt: "31015758346000000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "act_proposal",
          deposit: 0,
          fee: 327811528654700000000,
          args: '{"id": 129, "action": "VoteReject"}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 381145543250700000000,
      },
    },
    {
      id: "11368013253",
      receipt_id: "5Sh4eKAHeDCGnnYY6cPUbu9d12vsg31PcADZ2BHiRvzF",
      predecessor_account_id: "megha19.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "6Sj1to8G8kggjenKxXSJ519bcaYZ5H5yrXTLyPPXytQB",
        block_height: 135102795,
        block_timestamp: 1734340702309651200,
      },
      receipt_outcome: {
        gas_burnt: 2913798985415,
        tokens_burnt: 291379898541500000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "3gbZU7e9c42TS5ZVXbYWBPVbhC8j4aH9gr6ofXqU9Fyg",
      included_in_block_hash: "H5i7vuSi1Zghw13SMpq7QwmjVAwxqQJmvukmDxFbw8Tg",
      block_timestamp: "1734340701219740273",
      block: {
        block_height: 135102794,
      },
      receipt_conversion_tokens_burnt: "31559352697000000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "add_proposal",
          deposit: 0,
          fee: 291379898541500000000,
          args: '{"proposal": {"kind": {"ChangePolicyUpdateParameters": {"parameters": {"proposal_period": "950400000000000"}}}, "description": "Change proposal period"}}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 322939251238500000000,
      },
    },
    {
      id: "11367989314",
      receipt_id: "BmvpCjxt4cbCrqjVheyYPPBTtYE7gX6sRStQgJk43QCf",
      predecessor_account_id: "megha19.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "E4CxtaBZ6gaApuWQRRu9KmTNi4JQhJNXzxwrsyKVxKBQ",
        block_height: 135102730,
        block_timestamp: 1734340624801901300,
      },
      receipt_outcome: {
        gas_burnt: 2713255968758,
        tokens_burnt: 271325596875800000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: false,
      },
      transaction_hash: "7nJpxwPMMm5GioDEQqnFahjdfoFGMQwjBmhLWtqVNrwu",
      included_in_block_hash: "FrCfDWQsPLFCTHcdPiwCmwD1oXVr2tbF8FWZ9JqESouv",
      block_timestamp: "1734340623452087523",
      block: {
        block_height: 135102729,
      },
      receipt_conversion_tokens_burnt: "31015758346000000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "act_proposal",
          deposit: 0,
          fee: 271325596875800000000,
          args: '{"id": 128, "action": "VoteReject"}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: false,
      },
      outcomes_agg: {
        transaction_fee: 302341355221800000000,
      },
    },
    {
      id: "11150945850",
      receipt_id: "C4sUyGYrGYRXnmXfXnLD8HtnsfstYPyDWCTLqeaMPP3Y",
      predecessor_account_id: "testing-astradao.sputnik-dao.near",
      receiver_account_id: "theori.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "GPB6veEAgqyFVcb7tegfkhjbkueKJfKc9Bub9rFCar9D",
        block_height: 134703868,
        block_timestamp: 1733882659613957400,
      },
      receipt_outcome: {
        gas_burnt: 223182562500,
        tokens_burnt: 22318256250000000000,
        executor_account_id: "theori.near",
        status: true,
      },
      transaction_hash: "3jZjrU9LqZLfFP3HeRhEVMJ8foaQghH39kvh51xWfWs9",
      included_in_block_hash: "2pWL8Vo72gDwrvfqHmpF6xxoRhrr3D3m8VqSBJvAg7Ua",
      block_timestamp: "1733882657325144977",
      block: {
        block_height: 134703866,
      },
      receipt_conversion_tokens_burnt: "31020526717500000000",
      actions: [
        {
          action: "TRANSFER",
          method: null,
          deposit: 0,
          fee: 22318256250000000000,
          args: null,
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 538603285960600000000,
      },
    },
    {
      id: "11150945509",
      receipt_id: "AZneFScvsGnRK8JDwh1HdHxNTd3Hf5YFPsNdV5Hrking",
      predecessor_account_id: "theori.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "GEjwNt1dLrcK6GMJVpStTtDdW8W69puKLJhpFPXTtMRB",
        block_height: 134703867,
        block_timestamp: 1733882658333006600,
      },
      receipt_outcome: {
        gas_burnt: 4852645029931,
        tokens_burnt: 485264502993100000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "3jZjrU9LqZLfFP3HeRhEVMJ8foaQghH39kvh51xWfWs9",
      included_in_block_hash: "2pWL8Vo72gDwrvfqHmpF6xxoRhrr3D3m8VqSBJvAg7Ua",
      block_timestamp: "1733882657325144977",
      block: {
        block_height: 134703866,
      },
      receipt_conversion_tokens_burnt: "31020526717500000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "act_proposal",
          deposit: 0,
          fee: 485264502993100000000,
          args: '{"id": 128, "action": "VoteApprove"}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 538603285960600000000,
      },
    },
    {
      id: "11150943995",
      receipt_id: "4pExm3evPc5UNoNQBuhatzw5QKH38SSzAWvPz4wDwdgL",
      predecessor_account_id: "theori.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "5M1QJ3WtawWxbhMo2ws16hBXsc9kywcjTGa9nwzbeY7m",
        block_height: 134703862,
        block_timestamp: 1733882652896874200,
      },
      receipt_outcome: {
        gas_burnt: 4245699043435,
        tokens_burnt: 424569904343500000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "4DprRkqApkp4Yre9vsoopiK6NcApcnCkaLbEo6MvGyTS",
      included_in_block_hash: "ARL6VGdAuAsnwSo6o9eJsvAZCVS9xcYqgTbHTuVmc3C3",
      block_timestamp: "1733882651674783740",
      block: {
        block_height: 134703861,
      },
      receipt_conversion_tokens_burnt: "48687343125000000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "add_proposal",
          deposit: 0,
          fee: 424569904343500000000,
          args: '{"proposal": {"kind": {"ChangePolicy": {"policy": {"roles": [{"kind": {"Group": ["theori.near", "freski.near", "megha19.near", "frol.near", "thomasguntenaar.near", "petersalomonsen.near"]}, "name": "Create Requests", "permissions": ["call:AddProposal", "*:VoteRemove", "transfer:AddProposal", "config:Finalize"], "vote_policy": {"call": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "vote": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "config": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "policy": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "transfer": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "add_bounty": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "bounty_done": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "upgrade_self": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "set_vote_token": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "upgrade_remote": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "add_member_to_role": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "remove_member_from_role": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}}}, {"kind": {"Group": ["freski.near", "petersalomonsen.near", "thomasguntenaar.near", "theori.near", "megha19.near", "frol.near"]}, "name": "Manage Members", "permissions": ["config:*", "policy:*", "add_member_to_role:*", "policy_update_parameters:AddProposal", "policy_add_or_update_role:AddProposal", "remove_member_from_role:*"], "vote_policy": {"call": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "vote": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "config": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "policy": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "transfer": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "add_bounty": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "bounty_done": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "upgrade_self": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "set_vote_token": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "upgrade_remote": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "add_member_to_role": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "remove_member_from_role": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}}}, {"kind": {"Group": ["freski.near", "megha19.near", "meghagoel.near", "petersalomonsen.near", "frol.near"]}, "name": "Vote", "permissions": ["*:VoteReject", "*:VoteApprove", "*:VoteRemove"], "vote_policy": {"call": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "vote": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "config": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "policy": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "transfer": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "add_bounty": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "bounty_done": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "upgrade_self": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "set_vote_token": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "upgrade_remote": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "add_member_to_role": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "remove_member_from_role": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}}}], "bounty_bond": "100000000000000000000000", "proposal_bond": "0", "proposal_period": "864000000000000", "default_vote_policy": {"quorum": "1", "threshold": "1", "weight_kind": "RoleWeight"}, "bounty_forgiveness_period": "604800000000000"}}}, "description": "Change policy"}}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 473257247468500000000,
      },
    },
    {
      id: "11150202109",
      receipt_id: "58h5eQ8d1yJhnx3iqFbbh88Db9GRFpGXkeBMAg8aSnxK",
      predecessor_account_id: "testing-astradao.sputnik-dao.near",
      receiver_account_id: "theori.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "4mkHyvFM7FHNhnsPS3oMF2RDFNFDnHnMgYKPTCU31psk",
        block_height: 134701563,
        block_timestamp: 1733879912444341500,
      },
      receipt_outcome: {
        gas_burnt: 223182562500,
        tokens_burnt: 22318256250000000000,
        executor_account_id: "theori.near",
        status: true,
      },
      transaction_hash: "Ex6tKYc35k22oRXDG5yvPFvRXRd6dDj82oMhCU9UtTqR",
      included_in_block_hash: "FpkLSxP8BCBeHNRmhbZce8fzpP4GukMPxsVr925gRC5h",
      block_timestamp: "1733879910006974460",
      block: {
        block_height: 134701561,
      },
      receipt_conversion_tokens_burnt: "31020526717500000000",
      actions: [
        {
          action: "TRANSFER",
          method: null,
          deposit: 0,
          fee: 22318256250000000000,
          args: null,
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 536437364002100000000,
      },
    },
    {
      id: "11150201675",
      receipt_id: "vdjFgzgBuTBVPRSq5KFgnnwd1iTQAEbFzybABnQnhbK",
      predecessor_account_id: "theori.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "HRYTMdkskTdn2wpaueUj3kJ3C3HnpDnpv4qLucebTvSo",
        block_height: 134701562,
        block_timestamp: 1733879911228003600,
      },
      receipt_outcome: {
        gas_burnt: 4830985810346,
        tokens_burnt: 483098581034600000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "Ex6tKYc35k22oRXDG5yvPFvRXRd6dDj82oMhCU9UtTqR",
      included_in_block_hash: "FpkLSxP8BCBeHNRmhbZce8fzpP4GukMPxsVr925gRC5h",
      block_timestamp: "1733879910006974460",
      block: {
        block_height: 134701561,
      },
      receipt_conversion_tokens_burnt: "31020526717500000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "act_proposal",
          deposit: 0,
          fee: 483098581034600000000,
          args: '{"id": 127, "action": "VoteApprove"}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 536437364002100000000,
      },
    },
    {
      id: "11150200034",
      receipt_id: "8nNvDrhpPeEnpYJEqKs9SwwVFa4CnANn8JXXBoMEvFHA",
      predecessor_account_id: "theori.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "8SDxQdgPy4BBMg3yidLJFyozBQG7jTEZTRnEMMFEhta8",
        block_height: 134701557,
        block_timestamp: 1733879905560422700,
      },
      receipt_outcome: {
        gas_burnt: 4297511363923,
        tokens_burnt: 429751136392300000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "9RdLF5mPt5gJKGUqJApa2n3pWy6VsFg44k4U91fqffTD",
      included_in_block_hash: "8wZhPtUE9YEbK21xZHJ1T6dQJcBnyEYRUvJgiVPmoePo",
      block_timestamp: "1733879904272674011",
      block: {
        block_height: 134701556,
      },
      receipt_conversion_tokens_burnt: "48630122667000000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "add_proposal",
          deposit: 0,
          fee: 429751136392300000000,
          args: '{"proposal": {"kind": {"ChangePolicy": {"policy": {"roles": [{"kind": {"Group": ["theori.near", "freski.near", "megha19.near", "frol.near", "thomasguntenaar.near", "petersalomonsen.near"]}, "name": "Create Requests", "permissions": ["call:AddProposal", "*:VoteRemove", "transfer:AddProposal", "config:Finalize"], "vote_policy": {"call": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "vote": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "config": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "policy": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "transfer": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "add_bounty": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "bounty_done": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "upgrade_self": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "set_vote_token": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "upgrade_remote": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "add_member_to_role": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "remove_member_from_role": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}}}, {"kind": {"Group": ["freski.near", "petersalomonsen.near", "thomasguntenaar.near", "theori.near", "megha19.near"]}, "name": "Manage Members", "permissions": ["config:*", "policy:*", "add_member_to_role:*", "policy_update_parameters:AddProposal", "policy_add_or_update_role:AddProposal", "remove_member_from_role:*"], "vote_policy": {"call": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "vote": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "config": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "policy": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "transfer": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "add_bounty": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "bounty_done": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "upgrade_self": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "set_vote_token": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "upgrade_remote": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "add_member_to_role": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "remove_member_from_role": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}}}, {"kind": {"Group": ["freski.near", "megha19.near", "meghagoel.near", "petersalomonsen.near", "frol.near"]}, "name": "Vote", "permissions": ["*:VoteReject", "*:VoteApprove", "*:VoteRemove"], "vote_policy": {"call": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "vote": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "config": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "policy": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "transfer": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "add_bounty": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "bounty_done": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "upgrade_self": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "set_vote_token": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "upgrade_remote": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "add_member_to_role": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "remove_member_from_role": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}}}], "bounty_bond": "100000000000000000000000", "proposal_bond": "0", "proposal_period": "864000000000000", "default_vote_policy": {"quorum": "1", "threshold": "1", "weight_kind": "RoleWeight"}, "bounty_forgiveness_period": "604800000000000"}}}, "description": "Change policy"}}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 478381259059300000000,
      },
    },
    {
      id: "10720800369",
      receipt_id: "FTyYZVFAav1UEi9VbosYavpoYepyxCgDWp6M6D9Aw939",
      predecessor_account_id: "megha19.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "3Q6Jzu1GYYaX3BuKAkDw7jzCCuUN8fz4rCxcC7d9YAeM",
        block_height: 134214828,
        block_timestamp: 1733324542867720200,
      },
      receipt_outcome: {
        gas_burnt: 2850625428917,
        tokens_burnt: 285062542891700000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "HpbwTiTLtr9UgTEMxUFFZPKeL8P26fWEMSAk3nRhK45b",
      included_in_block_hash: "DP79S6V1pNAwoCCPNZzi4EbU9k4DRVkVxR1wr4FySef8",
      block_timestamp: "1733324541689127587",
      block: {
        block_height: 134214827,
      },
      receipt_conversion_tokens_burnt: "32126788905500000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "add_proposal",
          deposit: 0,
          fee: 285062542891700000000,
          args: '{"proposal": {"kind": {"FunctionCall": {"actions": [{"gas": "200000000000000", "args": "", "deposit": "10000000000000000000000", "method_name": "deposit_and_stake"}], "receiver_id": "astro-stakers.poolv1.near"}}, "description": "* Proposal Action: stake\\\\\\\\n* Notes: Testing notes"}}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 317189331797200000000,
      },
    },
    {
      id: "10720604169",
      receipt_id: "JDXLxzphA1eaZURQho6dhcHQ1DA6e4f2hUVjrN7XJPWK",
      predecessor_account_id: "megha19.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "EE2WTftHiqFS93B5RscG2Qeoo15uqAeUXMDLBsws6Uhs",
        block_height: 134214314,
        block_timestamp: 1733323961563307800,
      },
      receipt_outcome: {
        gas_burnt: 2878391097269,
        tokens_burnt: 287839109726900000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "DEgKuPgFcaWNYCt8LKMfnCTea4FAtj8AcPxALKSRgEED",
      included_in_block_hash: "PGB3uBdPbR1SQRfaT9WZRDDuxLaMQuKnd1kmanTKJxh",
      block_timestamp: "1733323960346437884",
      block: {
        block_height: 134214313,
      },
      receipt_conversion_tokens_burnt: "33557300355500000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "add_proposal",
          deposit: 0,
          fee: 287839109726900000000,
          args: '{"proposal": {"kind": {"Transfer": {"amount": "10000", "token_id": "usdt.tether-token.near", "receiver_id": "thomasguntenaar.near"}}, "description": "* Title: DevHub Developer Contributor report by THOMAS for 11/11/2024 \\\\\\\\u{2013}\\\\\\\\u{a0}11/22/2024\\\\\\\\n* Summary: Replacing the indexer infrastructure which we use at devhub to efficiently query data about proposals and rfp\\\\\\\\\\\\\'s so we can filter order search in the main dashboards of the events- & infrastructure-committee, the templar instance & devhub itself. Also supporting the treasury dashboards.\\\\\\\\n* Notes: testing\\\\\\\\n* Proposal Id: 260"}}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 321396410082400000000,
      },
    },
    {
      id: "10720529640",
      receipt_id: "EikJNrxWrTsFsj46rYVbR75JQEyyNoGV5LrkM5UgjMcM",
      predecessor_account_id: "megha19.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "A6YDZ2RttXmUx4enqAFX6jECLRzCUkygdEwRgmTmL8PR",
        block_height: 134214132,
        block_timestamp: 1733323758364167400,
      },
      receipt_outcome: {
        gas_burnt: 2862659748957,
        tokens_burnt: 286265974895700000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "APMLNvBbSKkcdrT2DCB1o4KKBZySrRe4K9z5DvwYG5Sn",
      included_in_block_hash: "D7t1wpJftHPvtVfc4R3SnUXK8N7atp19mtWSMw3sYcfs",
      block_timestamp: "1733323757308472623",
      block: {
        block_height: 134214131,
      },
      receipt_conversion_tokens_burnt: "32889728345500000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "add_proposal",
          deposit: 0,
          fee: 286265974895700000000,
          args: '{"proposal": {"kind": {"Transfer": {"amount": "10000000000000000000000", "token_id": "", "receiver_id": "megha19.near"}}, "description": "* Title: DevHub Developer Contributor report by Megha for 11/04/2024 - 12/01/2024\\\\\\\\n* Summary: Worked on treasury dashboard, added support for lockup contract, added staking and unstaking, updated dashboard, infinex, helped Thomas with indexer testing.\\\\\\\\n* Notes: testing notes\\\\\\\\n* ProposalId: 266"}}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 319155703241200000000,
      },
    },
    {
      id: "10720513722",
      receipt_id: "AuEPYCJYroemnSNvw9wiJtoUteE6jG9MY7V4JCTdjTJb",
      predecessor_account_id: "megha19.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "GhemU4nwmykrCyi5ZzRX8rEujETt3CvTR8gN4z5mpDbd",
        block_height: 134214092,
        block_timestamp: 1733323714238445300,
      },
      receipt_outcome: {
        gas_burnt: 2868958055157,
        tokens_burnt: 286895805515700000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "FRgrLB1X5LXQRQUUbbU7gAeBUZMUzwqpepteXVGVGG1H",
      included_in_block_hash: "FTe4cwpXn7V96vPbZ1rYH4AtmEaCQpJgdBC1GvbsVh8L",
      block_timestamp: "1733323713336309823",
      block: {
        block_height: 134214091,
      },
      receipt_conversion_tokens_burnt: "33075694834000000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "add_proposal",
          deposit: 0,
          fee: 286895805515700000000,
          args: '{"proposal": {"kind": {"Transfer": {"amount": "1000000000000000000000", "token_id": "", "receiver_id": "megha19.near"}}, "description": "* Title: DevRel & DevHub (01 - 30 November) Contributor Report\\\\\\\\n* Summary: Monthly paid developer relations contributor report. These activities aim to onboard more developers to the ecosystem, and assist existing developers with problems or enrich their experience. Activities for the last month can be seen below.\\\\\\\\n* ProposalId: 267"}}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 319971500349700000000,
      },
    },
    {
      id: "10648074527",
      receipt_id: "JAw6bLmMdJUgsx1hTWYfJKuHbKpveCRYuhwGww1rR4Wf",
      predecessor_account_id: "testing-astradao.sputnik-dao.near",
      receiver_account_id: "megha19.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "6mac8ZXePSf29HxWA3jkWV9kqyhwN1wDTRf6z8mG9UJF",
        block_height: 134053019,
        block_timestamp: 1733140424910559700,
      },
      receipt_outcome: {
        gas_burnt: 223182562500,
        tokens_burnt: 22318256250000000000,
        executor_account_id: "megha19.near",
        status: true,
      },
      transaction_hash: "33CWu4Tavf1FZPFVKeGaVyGWSjiozQePXUtbwcb7VU2X",
      included_in_block_hash: "Eg1mJW2rqkWHQNMoc7eDNWSSsXK1XGvh8ABt75hiAM3n",
      block_timestamp: "1733140420619015943",
      block: {
        block_height: 134053015,
      },
      receipt_conversion_tokens_burnt: "31020526717500000000",
      actions: [
        {
          action: "TRANSFER",
          method: null,
          deposit: 0,
          fee: 22318256250000000000,
          args: null,
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 1.2361571289765e21,
      },
    },
    {
      id: "10648073797",
      receipt_id: "8NZx7B87hPwNA1LKLjPejXz5fB3k7rDbrhVQdgANcUTz",
      predecessor_account_id: "testing-astradao.sputnik-dao.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "4DR41Vw6Y1xhqZbVsgPqT8TwibwydMYHAiFhpaf1hEDH",
        block_height: 134053017,
        block_timestamp: 1733140422933179600,
      },
      receipt_outcome: {
        gas_burnt: 3127924417561,
        tokens_burnt: 312792441756100000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "33CWu4Tavf1FZPFVKeGaVyGWSjiozQePXUtbwcb7VU2X",
      included_in_block_hash: "Eg1mJW2rqkWHQNMoc7eDNWSSsXK1XGvh8ABt75hiAM3n",
      block_timestamp: "1733140420619015943",
      block: {
        block_height: 134053015,
      },
      receipt_conversion_tokens_burnt: "31020526717500000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "on_proposal_callback",
          deposit: 0,
          fee: 312792441756100000000,
          args: '{"proposal_id": 122}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 1.2361571289765e21,
      },
    },
    {
      id: "10648073795",
      receipt_id: "9geJZS8dJJeUipbq74f8UEGCWX3LByG1E6phnSzVscPB",
      predecessor_account_id: "testing-astradao.sputnik-dao.near",
      receiver_account_id: "bisontrails.poolv1.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "4DR41Vw6Y1xhqZbVsgPqT8TwibwydMYHAiFhpaf1hEDH",
        block_height: 134053017,
        block_timestamp: 1733140422933179600,
      },
      receipt_outcome: {
        gas_burnt: 3026095435036,
        tokens_burnt: 302609543503600000000,
        executor_account_id: "bisontrails.poolv1.near",
        status: true,
      },
      transaction_hash: "33CWu4Tavf1FZPFVKeGaVyGWSjiozQePXUtbwcb7VU2X",
      included_in_block_hash: "Eg1mJW2rqkWHQNMoc7eDNWSSsXK1XGvh8ABt75hiAM3n",
      block_timestamp: "1733140420619015943",
      block: {
        block_height: 134053015,
      },
      receipt_conversion_tokens_burnt: "31020526717500000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "unstake",
          deposit: 0,
          fee: 302609543503600000000,
          args: '{"amount": "250000000000000000000000"}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 1.2361571289765e21,
      },
    },
    {
      id: "10648073416",
      receipt_id: "8a4KWaJ9BhCncnxb6Ao7jiQQhbWFr47eofwyqCuk8DCh",
      predecessor_account_id: "megha19.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "DA8RYzaANrJnDs1gJiy8KFSJ42Qh4qNqVsn5p2GsERne",
        block_height: 134053016,
        block_timestamp: 1733140421840036000,
      },
      receipt_outcome: {
        gas_burnt: 3872545268656,
        tokens_burnt: 387254526865600000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "33CWu4Tavf1FZPFVKeGaVyGWSjiozQePXUtbwcb7VU2X",
      included_in_block_hash: "Eg1mJW2rqkWHQNMoc7eDNWSSsXK1XGvh8ABt75hiAM3n",
      block_timestamp: "1733140420619015943",
      block: {
        block_height: 134053015,
      },
      receipt_conversion_tokens_burnt: "31020526717500000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "act_proposal",
          deposit: 0,
          fee: 387254526865600000000,
          args: '{"id": 122, "action": "VoteApprove"}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 1.2361571289765e21,
      },
    },
    {
      id: "10648069284",
      receipt_id: "BNXhwN4sa2jvtQfFtSbLN4tQmndeSrMFYR1tQokaPhsi",
      predecessor_account_id: "megha19.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "BQn8x3TVirjgrxSaY4gL3CorDK7BjMroD4c6ZiCZEW8S",
        block_height: 134053002,
        block_timestamp: 1733140406100546800,
      },
      receipt_outcome: {
        gas_burnt: 2937463757289,
        tokens_burnt: 293746375728900000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "DsK5ZjnGmXQ4qLu9eypZaQGquBRzZLuYRs3j63GmrrUf",
      included_in_block_hash: "EnoCmmFgYf6TASXN3fwcKy8Y1FAZbPtWBSnjyPZ1vLby",
      block_timestamp: "1733140404966551975",
      block: {
        block_height: 134053001,
      },
      receipt_conversion_tokens_burnt: "32174472620500000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "add_proposal",
          deposit: 0,
          fee: 293746375728900000000,
          args: '{"proposal": {"kind": {"FunctionCall": {"actions": [{"gas": "200000000000000", "args": "eyJhbW91bnQiOiIyNTAwMDAwMDAwMDAwMDAwMDAwMDAwMDAifQ==", "deposit": "0", "method_name": "unstake"}], "receiver_id": "bisontrails.poolv1.near"}}, "description": "{\\\\\\\\\\\\\\"isStakeRequest\\\\\\\\\\\\\\":true,\\\\\\\\\\\\\\"notes\\\\\\\\\\\\\\":null}"}}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 325920848349400000000,
      },
    },
    {
      id: "10647933369",
      receipt_id: "ESo2FwdsVTLx7X6eFALMD8B6vQJtvEdkHYiEL7hXScjK",
      predecessor_account_id: "testing-astradao.sputnik-dao.near",
      receiver_account_id: "megha19.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "HatkhgwuQ3d8GJ4EKhTRMEosv6npaLhXy6JBbtJeeCSr",
        block_height: 134052706,
        block_timestamp: 1733140066661023200,
      },
      receipt_outcome: {
        gas_burnt: 223182562500,
        tokens_burnt: 22318256250000000000,
        executor_account_id: "megha19.near",
        status: true,
      },
      transaction_hash: "BBQAmiSngBpyaGYoo8wsuo32aiJEzicQo76nmK8vVqzk",
      included_in_block_hash: "CfzpVWvZTiAMheYpxvebK41ncHc9Z4Gi7qGy1rSHQj7x",
      block_timestamp: "1733140062280899672",
      block: {
        block_height: 134052702,
      },
      receipt_conversion_tokens_burnt: "31020526717500000000",
      actions: [
        {
          action: "TRANSFER",
          method: null,
          deposit: 0,
          fee: 22318256250000000000,
          args: null,
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 1.2976023778136e21,
      },
    },
    {
      id: "10647932407",
      receipt_id: "9C1dgZcP8aJS4xq1Et4e4zHxY5N9eEzZaUqCEmj9SXct",
      predecessor_account_id: "testing-astradao.sputnik-dao.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "ADskNobd8PfxZfQVSHytXLNcqy5Ge9wBWv2dWiBs7hKh",
        block_height: 134052704,
        block_timestamp: 1733140064547454200,
      },
      receipt_outcome: {
        gas_burnt: 3122365808778,
        tokens_burnt: 312236580877800000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "BBQAmiSngBpyaGYoo8wsuo32aiJEzicQo76nmK8vVqzk",
      included_in_block_hash: "CfzpVWvZTiAMheYpxvebK41ncHc9Z4Gi7qGy1rSHQj7x",
      block_timestamp: "1733140062280899672",
      block: {
        block_height: 134052702,
      },
      receipt_conversion_tokens_burnt: "31020526717500000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "on_proposal_callback",
          deposit: 0,
          fee: 312236580877800000000,
          args: '{"proposal_id": 121}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 1.2976023778136e21,
      },
    },
    {
      id: "10647932398",
      receipt_id: "F1spBNpqEVJQjwVHoYVcvV5XH9MnKWvsFG2RpCHSLDrX",
      predecessor_account_id: "testing-astradao.sputnik-dao.near",
      receiver_account_id: "bisontrails.poolv1.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "ADskNobd8PfxZfQVSHytXLNcqy5Ge9wBWv2dWiBs7hKh",
        block_height: 134052704,
        block_timestamp: 1733140064547454200,
      },
      receipt_outcome: {
        gas_burnt: 3653767743965,
        tokens_burnt: 365376774396500000000,
        executor_account_id: "bisontrails.poolv1.near",
        status: true,
      },
      transaction_hash: "BBQAmiSngBpyaGYoo8wsuo32aiJEzicQo76nmK8vVqzk",
      included_in_block_hash: "CfzpVWvZTiAMheYpxvebK41ncHc9Z4Gi7qGy1rSHQj7x",
      block_timestamp: "1733140062280899672",
      block: {
        block_height: 134052702,
      },
      receipt_conversion_tokens_burnt: "31020526717500000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "deposit_and_stake",
          deposit: 5e23,
          fee: 365376774396500000000,
          args: null,
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 1.2976023778136e21,
      },
    },
    {
      id: "10647931699",
      receipt_id: "4AJWHiKW3xdXmKQ6QrPQngxRASL2MjN5JjKhCGcCVeYJ",
      predecessor_account_id: "megha19.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "37fFFLcdwevxmQ66fWjp7qhfT9QUauPYG12VysJ7vSbo",
        block_height: 134052703,
        block_timestamp: 1733140063423976200,
      },
      receipt_outcome: {
        gas_burnt: 3864884056881,
        tokens_burnt: 386488405688100000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "BBQAmiSngBpyaGYoo8wsuo32aiJEzicQo76nmK8vVqzk",
      included_in_block_hash: "CfzpVWvZTiAMheYpxvebK41ncHc9Z4Gi7qGy1rSHQj7x",
      block_timestamp: "1733140062280899672",
      block: {
        block_height: 134052702,
      },
      receipt_conversion_tokens_burnt: "31020526717500000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "act_proposal",
          deposit: 0,
          fee: 386488405688100000000,
          args: '{"id": 121, "action": "VoteApprove"}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 1.2976023778136e21,
      },
    },
    {
      id: "10647922519",
      receipt_id: "8sb9ksPmcHSLH1AuV9DyQfanD1sHtYxpMxFvn6RcUaqm",
      predecessor_account_id: "megha19.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "5defsqWDGsDnkhuMsr1Ngp6XQTiB5Wc35waJBCHZjKKc",
        block_height: 134052682,
        block_timestamp: 1733140040444847400,
      },
      receipt_outcome: {
        gas_burnt: 2935677627509,
        tokens_burnt: 293567762750900000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "CHZPAstbJ2yhKeEzTnxwq3KTHkqwbdhpVTkLYVvmdLRE",
      included_in_block_hash: "7pb3j2iZ1Z14Br3vyV4e98evdVjwwK5s6RKZxG3CnkDe",
      block_timestamp: "1733140038071558701",
      block: {
        block_height: 134052681,
      },
      receipt_conversion_tokens_burnt: "32083873562000000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "add_proposal",
          deposit: 0,
          fee: 293567762750900000000,
          args: '{"proposal": {"kind": {"FunctionCall": {"actions": [{"gas": "200000000000000", "args": "", "deposit": "500000000000000000000000", "method_name": "deposit_and_stake"}], "receiver_id": "bisontrails.poolv1.near"}}, "description": "{\\\\\\\\\\\\\\"isStakeRequest\\\\\\\\\\\\\\":true,\\\\\\\\\\\\\\"notes\\\\\\\\\\\\\\":\\\\\\\\\\\\\\"\\\\\\\\\\\\\\"}"}}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 325651636312900000000,
      },
    },
    {
      id: "10502128620",
      receipt_id: "AdxQe2hUW3XnEt7E4HSjrZtNC9DivmJNmTpLjGrYLTCd",
      predecessor_account_id: "theori.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "J2hCaQDnbX8GeFQmttyxvKPpMPnAGg4qQkNRdCi2tz2Q",
        block_height: 133691943,
        block_timestamp: 1732733071451980500,
      },
      receipt_outcome: {
        gas_burnt: 2896154997999,
        tokens_burnt: 289615499799900000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "GNEUnNkpf9aTE3RRdgoen3zk3R9Hg1TRygR8Fk8XMurY",
      included_in_block_hash: "3D1S5qcv4tQfboZGQHagrYKiCeg4sUfddfaSXfbUfgou",
      block_timestamp: "1732733070349639574",
      block: {
        block_height: 133691942,
      },
      receipt_conversion_tokens_burnt: "31559352697000000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "add_proposal",
          deposit: 0,
          fee: 289615499799900000000,
          args: '{"proposal": {"kind": {"ChangePolicyUpdateParameters": {"parameters": {"proposal_period": "950400000000000"}}}, "description": "Change proposal period"}}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 321174852496900000000,
      },
    },
    {
      id: "10400226934",
      receipt_id: "2XBgiApgBac5uxRW9MSBdXaBNxXyPUn8EFh4UQGgVW3k",
      predecessor_account_id: "testing-astradao.sputnik-dao.near",
      receiver_account_id: "petersalomonsen.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "2FmBRr3Hpy69bamEwrFVnk6Gub5neE9Vaj5jAy7foUAW",
        block_height: 133425552,
        block_timestamp: 1732436112636790300,
      },
      receipt_outcome: {
        gas_burnt: 223182562500,
        tokens_burnt: 22318256250000000000,
        executor_account_id: "petersalomonsen.near",
        status: true,
      },
      transaction_hash: "ASk12bypjh8c4cP17DyjoqAy7RLnEJ55XEmLDF91tVAB",
      included_in_block_hash: "3DC13HQASejwCLjPgzQ5oRtSu9Rvw72TB2Wfff8ZMJzq",
      block_timestamp: "1732436110418787854",
      block: {
        block_height: 133425550,
      },
      receipt_conversion_tokens_burnt: "31020526717500000000",
      actions: [
        {
          action: "TRANSFER",
          method: null,
          deposit: 0,
          fee: 22318256250000000000,
          args: null,
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 442002275082800000000,
      },
    },
    {
      id: "10400226636",
      receipt_id: "BthqH69KiBprL6p9rhqFMrTHPrvnFVRzKd45gaeyb1gf",
      predecessor_account_id: "petersalomonsen.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "28P9hn5HovWQeeoYHxdBmdXPdzM7pxQ7WpgEDNbxupDZ",
        block_height: 133425551,
        block_timestamp: 1732436111586207500,
      },
      receipt_outcome: {
        gas_burnt: 3886634921153,
        tokens_burnt: 388663492115300000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "ASk12bypjh8c4cP17DyjoqAy7RLnEJ55XEmLDF91tVAB",
      included_in_block_hash: "3DC13HQASejwCLjPgzQ5oRtSu9Rvw72TB2Wfff8ZMJzq",
      block_timestamp: "1732436110418787854",
      block: {
        block_height: 133425550,
      },
      receipt_conversion_tokens_burnt: "31020526717500000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "act_proposal",
          deposit: 0,
          fee: 388663492115300000000,
          args: '{"id": 119, "action": "VoteApprove"}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 442002275082800000000,
      },
    },
    {
      id: "10400202465",
      receipt_id: "AKZumKZhapwsrTkp49q4yG8QGeU4YNHiMhK5FW4Cv2Yj",
      predecessor_account_id: "petersalomonsen.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "G7d5eK5sWQbMcRBAgx6wAhR14TiTSbcpQ25iH8TmqK6P",
        block_height: 133425466,
        block_timestamp: 1732436019310463200,
      },
      receipt_outcome: {
        gas_burnt: 2871289854903,
        tokens_burnt: 287128985490300000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "7Y4aeKzxgoPgUsDKSC89VyVGzzap2vWBthQcZtBz8sgx",
      included_in_block_hash: "EJBu1XQKhcZHzkuYVyA8F8sXp4UmXJ95RHbzSY8qUbFB",
      block_timestamp: "1732436018195161109",
      block: {
        block_height: 133425465,
      },
      receipt_conversion_tokens_burnt: "31559352697000000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "add_proposal",
          deposit: 0,
          fee: 287128985490300000000,
          args: '{"proposal": {"kind": {"ChangePolicyUpdateParameters": {"parameters": {"proposal_period": "864000000000000"}}}, "description": "Change proposal period"}}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 318688338187300000000,
      },
    },
    {
      id: "10400110219",
      receipt_id: "Hw8GTxDSsNuxnezDm4shF67mDTrdcdGYiXF9uoeoRmbG",
      predecessor_account_id: "testing-astradao.sputnik-dao.near",
      receiver_account_id: "petersalomonsen.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "44zjrHAgjQboh9vwEw7wkfdEARAQqbkKL5TrMnYDk9Wi",
        block_height: 133425227,
        block_timestamp: 1732435752475275800,
      },
      receipt_outcome: {
        gas_burnt: 223182562500,
        tokens_burnt: 22318256250000000000,
        executor_account_id: "petersalomonsen.near",
        status: true,
      },
      transaction_hash: "87JjNXxgMmrx44W8QD5U2Xq1LRYk5TmrJsWCY8j1C6oK",
      included_in_block_hash: "x8gXvr1YnwTUnk5WqPt636Zhv9JGNiv6ygBx1z1xKxa",
      block_timestamp: "1732435750381267779",
      block: {
        block_height: 133425225,
      },
      receipt_conversion_tokens_burnt: "31020526717500000000",
      actions: [
        {
          action: "TRANSFER",
          method: null,
          deposit: 0,
          fee: 22318256250000000000,
          args: null,
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 442002275082800000000,
      },
    },
    {
      id: "10400110165",
      receipt_id: "AMSFtFbfkJJYvuh71HGykhiMrrrsrUHX4dCbyh4yAcaD",
      predecessor_account_id: "petersalomonsen.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "H6VfySMm3HznCDcxDSwdHT6RuYNkhKv2JAwGLcDcv8tz",
        block_height: 133425226,
        block_timestamp: 1732435751492974600,
      },
      receipt_outcome: {
        gas_burnt: 3886634921153,
        tokens_burnt: 388663492115300000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "87JjNXxgMmrx44W8QD5U2Xq1LRYk5TmrJsWCY8j1C6oK",
      included_in_block_hash: "x8gXvr1YnwTUnk5WqPt636Zhv9JGNiv6ygBx1z1xKxa",
      block_timestamp: "1732435750381267779",
      block: {
        block_height: 133425225,
      },
      receipt_conversion_tokens_burnt: "31020526717500000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "act_proposal",
          deposit: 0,
          fee: 388663492115300000000,
          args: '{"id": 118, "action": "VoteApprove"}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 442002275082800000000,
      },
    },
    {
      id: "10400105937",
      receipt_id: "67RddmTzTBWPaJiCwvLaxenXAHpEqTKXzYM8oSgifMHE",
      predecessor_account_id: "petersalomonsen.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "8gFsDuT986KCQUEPonfBHGQNtNixtMQnKLa2rrjTFW2P",
        block_height: 133425172,
        block_timestamp: 1732435688990069500,
      },
      receipt_outcome: {
        gas_burnt: 2912755722681,
        tokens_burnt: 291275572268100000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "FW7oKnkqcwKgMUXGWo1Q8fiBVuSehy6HsvD6hticcxZL",
      included_in_block_hash: "D5U5cDLqLvtRV3ftKu9RLWSkAgrxpA644cgYMeivGUJL",
      block_timestamp: "1732435687733591498",
      block: {
        block_height: 133425171,
      },
      receipt_conversion_tokens_burnt: "31559352697000000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "add_proposal",
          deposit: 0,
          fee: 291275572268100000000,
          args: '{"proposal": {"kind": {"ChangePolicyUpdateParameters": {"parameters": {"proposal_period": "518400000000000"}}}, "description": "Change proposal period"}}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 322834924965100000000,
      },
    },
    {
      id: "10400071248",
      receipt_id: "8mUN7oRgaDKr71iSpMJ2FvwQm2oHhiuxssNhKqr1WywZ",
      predecessor_account_id: "testing-astradao.sputnik-dao.near",
      receiver_account_id: "petersalomonsen.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "2npYEUnRDSQFto9yuuXciVyMFWgSnkecXaC9bsFRrBEX",
        block_height: 133425048,
        block_timestamp: 1732435550101958000,
      },
      receipt_outcome: {
        gas_burnt: 223182562500,
        tokens_burnt: 22318256250000000000,
        executor_account_id: "petersalomonsen.near",
        status: true,
      },
      transaction_hash: "99eA5kj1RHHSxaNosSqHjAJuFr97CizJjrMXXb7Wp7o7",
      included_in_block_hash: "BgA2aEXibLfs1kB1WJ1Gd1AWcwpLVMoHwBPBKykpNv1w",
      block_timestamp: "1732435547931228591",
      block: {
        block_height: 133425046,
      },
      receipt_conversion_tokens_burnt: "31020526717500000000",
      actions: [
        {
          action: "TRANSFER",
          method: null,
          deposit: 0,
          fee: 22318256250000000000,
          args: null,
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 542684172448000000000,
      },
    },
    {
      id: "10400070662",
      receipt_id: "HcYWx1Kz8MMc8VVf8fgPjmGYRCwJdCw7cTG4MHvzA4AV",
      predecessor_account_id: "petersalomonsen.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "wa2XvwP4e3hqH1jfLPdvzbJn7Qzs9F5iisR9s8w2cPB",
        block_height: 133425047,
        block_timestamp: 1732435549067875800,
      },
      receipt_outcome: {
        gas_burnt: 4893453894805,
        tokens_burnt: 489345389480500000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "99eA5kj1RHHSxaNosSqHjAJuFr97CizJjrMXXb7Wp7o7",
      included_in_block_hash: "BgA2aEXibLfs1kB1WJ1Gd1AWcwpLVMoHwBPBKykpNv1w",
      block_timestamp: "1732435547931228591",
      block: {
        block_height: 133425046,
      },
      receipt_conversion_tokens_burnt: "31020526717500000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "act_proposal",
          deposit: 0,
          fee: 489345389480500000000,
          args: '{"id": 117, "action": "VoteApprove"}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 542684172448000000000,
      },
    },
    {
      id: "10399998757",
      receipt_id: "BQYJaZobs2QE8jkxhYoDUq2AH6NtMzpaAVmvDa2dqufh",
      predecessor_account_id: "petersalomonsen.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "J83W1QfbwsjFMhd7FrFfEBUrNR9ezXP1NRgmE3QSxoQ6",
        block_height: 133424899,
        block_timestamp: 1732435384406369500,
      },
      receipt_outcome: {
        gas_burnt: 4755068398307,
        tokens_burnt: 475506839830700000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "4ucJRwdxL2CsA3akNSvyzT5qhLWgLdAXBtQeyQXSEM2b",
      included_in_block_hash: "63DwVDe3ZNTipHeSrDRC5dmm9bBf1KDhTiVakcqH19r7",
      block_timestamp: "1732435383163199474",
      block: {
        block_height: 133424898,
      },
      receipt_conversion_tokens_burnt: "74355486909500000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "add_proposal",
          deposit: 0,
          fee: 475506839830700000000,
          args: '{"proposal": {"kind": {"ChangePolicy": {"policy": {"roles": [{"kind": {"Group": ["theori.near", "freski.near", "megha19.near", "frol.near", "thomasguntenaar.near", "petersalomonsen.near"]}, "name": "Create Requests", "permissions": ["call:AddProposal", "*:VoteRemove", "transfer:AddProposal", "config:Finalize"], "vote_policy": {"call": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "vote": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "config": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "policy": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "transfer": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "add_bounty": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "bounty_done": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "upgrade_self": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "set_vote_token": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "upgrade_remote": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "add_member_to_role": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "remove_member_from_role": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}}}, {"kind": {"Group": ["freski.near", "petersalomonsen.near", "thomasguntenaar.near", "theori.near", "megha19.near", "frol.near"]}, "name": "Manage Members", "permissions": ["config:*", "policy:*", "policy_update_parameters:AddProposal", "policy_add_or_update_role:AddProposal", "add_member_to_role:*", "remove_member_from_role:*"], "vote_policy": {"call": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "vote": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "config": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "policy": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "transfer": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "add_bounty": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "bounty_done": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "upgrade_self": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "set_vote_token": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "upgrade_remote": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "add_member_to_role": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "remove_member_from_role": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}}}, {"kind": {"Group": ["freski.near", "megha19.near", "meghagoel.near", "petersalomonsen.near"]}, "name": "Vote", "permissions": ["*:VoteReject", "*:VoteApprove", "*:VoteRemove"], "vote_policy": {"call": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "vote": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "config": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "policy": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "transfer": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "add_bounty": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "bounty_done": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "upgrade_self": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "set_vote_token": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "upgrade_remote": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "add_member_to_role": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "remove_member_from_role": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}}}], "bounty_bond": "100000000000000000000000", "proposal_bond": "0", "proposal_period": "604800000000000", "default_vote_policy": {"quorum": "1", "threshold": "1", "weight_kind": "RoleWeight"}, "bounty_forgiveness_period": "604800000000000"}}}, "description": "Add allowed proposal kinds"}}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 549862326740200000000,
      },
    },
    {
      id: "10372781526",
      receipt_id: "HXuUiQAviTCNeUPVibVMvuQfSv63yKh9Lm9x7LPfzfUa",
      predecessor_account_id: "petersalomonsen.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "A4JyJ3unfYJ2czWv9LPa5DFEEpTKAoegeY1YFcuewH14",
        block_height: 133350887,
        block_timestamp: 1732351572012133000,
      },
      receipt_outcome: {
        gas_burnt: 2239359396995,
        tokens_burnt: 223935939699500000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: false,
      },
      transaction_hash: "eseGU9orFSN42g7cXxfsGpHiT22cMw9C6KvMneYs3pt",
      included_in_block_hash: "HDuDDLFWqHxZprxzxmRry3wz6y4gv31Y2xa3Z4B1NnA",
      block_timestamp: "1732351570791569890",
      block: {
        block_height: 133350886,
      },
      receipt_conversion_tokens_burnt: "31554584325500000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "add_proposal",
          deposit: 0,
          fee: 223935939699500000000,
          args: '{"proposal": {"kind": {"ChangePolicyUpdateParameters": {"parameters": {"proposal_period": "86400000000000"}}}, "description": "Change proposal period"}}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: false,
      },
      outcomes_agg: {
        transaction_fee: 255490524025000000000,
      },
    },
    {
      id: "10267189979",
      receipt_id: "4CUUCxG3eXyAFeji563m5ZndpDQxZ1iX5v2HV1i4V5UT",
      predecessor_account_id: "testing-astradao.sputnik-dao.near",
      receiver_account_id: "megha19.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "58JuWMXg4gQCu5SiMv6f6UvjqueBE4AzfEqyhyZVdtVm",
        block_height: 133074700,
        block_timestamp: 1732042994973828600,
      },
      receipt_outcome: {
        gas_burnt: 223182562500,
        tokens_burnt: 22318256250000000000,
        executor_account_id: "megha19.near",
        status: true,
      },
      transaction_hash: "H678TmCirpe3TyGnALjTaXDH7fZDYRH5nJNoY8vBZadq",
      included_in_block_hash: "DxLBNGodABBKzeei4uriybnFHS12xeDDHbXSiS7N51bx",
      block_timestamp: "1732042992756041571",
      block: {
        block_height: 133074698,
      },
      receipt_conversion_tokens_burnt: "31020526717500000000",
      actions: [
        {
          action: "TRANSFER",
          method: null,
          deposit: 0,
          fee: 22318256250000000000,
          args: null,
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 537415322803600000000,
      },
    },
    {
      id: "10267189476",
      receipt_id: "4sZo3jaGm5qysVtaxE7UU2AZTfoviBH3eiAu9kR8JuFc",
      predecessor_account_id: "megha19.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "13CdbhRmDvFcE8eFap7TyeqeNkHcpQ19Bcf9o9c5FtSp",
        block_height: 133074699,
        block_timestamp: 1732042993869794000,
      },
      receipt_outcome: {
        gas_burnt: 4840765398361,
        tokens_burnt: 484076539836100000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "H678TmCirpe3TyGnALjTaXDH7fZDYRH5nJNoY8vBZadq",
      included_in_block_hash: "DxLBNGodABBKzeei4uriybnFHS12xeDDHbXSiS7N51bx",
      block_timestamp: "1732042992756041571",
      block: {
        block_height: 133074698,
      },
      receipt_conversion_tokens_burnt: "31020526717500000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "act_proposal",
          deposit: 0,
          fee: 484076539836100000000,
          args: '{"id": 116, "action": "VoteApprove"}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 537415322803600000000,
      },
    },
    {
      id: "10267188839",
      receipt_id: "HY6g2CLQEjcJr8H2PQ3WggSNuUA1RyhQ1kvyyciB3wpQ",
      predecessor_account_id: "megha19.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "9tmkZMA59Hark9tLrgQLmCJabCYa3GGYZ2TcRUGPVHYa",
        block_height: 133074696,
        block_timestamp: 1732042990840714500,
      },
      receipt_outcome: {
        gas_burnt: 4199218390995,
        tokens_burnt: 419921839099500000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "2VKB4k46UJ2wZwBCBCYXmuyvxoZjXBPyZhXpXmNYtJVV",
      included_in_block_hash: "DBjRMEsddsSjhpG1cbVgv4ntCmw8TsrA9MJnzhRPxouU",
      block_timestamp: "1732042989629242995",
      block: {
        block_height: 133074695,
      },
      receipt_conversion_tokens_burnt: "48253421318500000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "add_proposal",
          deposit: 0,
          fee: 419921839099500000000,
          args: '{"proposal": {"kind": {"ChangePolicy": {"policy": {"roles": [{"kind": {"Group": ["theori.near", "freski.near", "megha19.near", "frol.near", "thomasguntenaar.near", "petersalomonsen.near"]}, "name": "Create Requests", "permissions": ["call:AddProposal", "*:VoteRemove", "transfer:AddProposal", "config:Finalize"], "vote_policy": {"call": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "vote": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "config": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "policy": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "transfer": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "add_bounty": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "bounty_done": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "upgrade_self": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "set_vote_token": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "upgrade_remote": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "add_member_to_role": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "remove_member_from_role": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}}}, {"kind": {"Group": ["freski.near", "petersalomonsen.near", "thomasguntenaar.near", "theori.near", "megha19.near", "frol.near"]}, "name": "Manage Members", "permissions": ["config:*", "policy:*", "add_member_to_role:*", "remove_member_from_role:*"], "vote_policy": {"call": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "vote": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "config": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "policy": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "transfer": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "add_bounty": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "bounty_done": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "upgrade_self": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "set_vote_token": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "upgrade_remote": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "add_member_to_role": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "remove_member_from_role": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}}}, {"kind": {"Group": ["meghagoel.near", "freski.near", "megha19.near", "petersalomonsen.near"]}, "name": "Vote", "permissions": ["*:VoteReject", "*:VoteApprove", "*:VoteRemove"], "vote_policy": {"call": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "vote": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "config": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "policy": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "transfer": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "add_bounty": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "bounty_done": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "upgrade_self": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "set_vote_token": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "upgrade_remote": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "add_member_to_role": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "remove_member_from_role": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}}}], "bounty_bond": "100000000000000000000000", "proposal_bond": "0", "proposal_period": "604800000000000", "default_vote_policy": {"quorum": "1", "threshold": "1", "weight_kind": "RoleWeight"}, "bounty_forgiveness_period": "604800000000000"}}}, "description": "Change policy"}}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 468175260418000000000,
      },
    },
    {
      id: "10223704134",
      receipt_id: "F3A9vkA3K3dDjw9ur8ThsZfUV5juphjD3bEbmRsgoz6d",
      predecessor_account_id: "testing-astradao.sputnik-dao.near",
      receiver_account_id: "megha19.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "B998A7Vjxr2sqNC7xuTsWmEjukGewTM9Ds8z9ZKEh2Mq",
        block_height: 132970409,
        block_timestamp: 1731921923899565800,
      },
      receipt_outcome: {
        gas_burnt: 223182562500,
        tokens_burnt: 22318256250000000000,
        executor_account_id: "megha19.near",
        status: true,
      },
      transaction_hash: "Gg6E3qZwLz1yoD3Urr2dqVrppZjkvGmuNpL52zmJVR7k",
      included_in_block_hash: "HGSxCUWyTLadAxB78VVGJgCehUyJh5CsCyGmqvQRJGz4",
      block_timestamp: "1731921921705748185",
      block: {
        block_height: 132970407,
      },
      receipt_conversion_tokens_burnt: "31020526717500000000",
      actions: [
        {
          action: "TRANSFER",
          method: null,
          deposit: 0,
          fee: 22318256250000000000,
          args: null,
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 529964315392200000000,
      },
    },
    {
      id: "10223703576",
      receipt_id: "8MxXrtJ9KDLMMxpUCrpDzaZ2BbGv3DJeG29vo2YaU4Y9",
      predecessor_account_id: "megha19.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "HHfN7wj9hMepnwmAvY3JRwv85iPN738eGDongd8JDR68",
        block_height: 132970408,
        block_timestamp: 1731921922848291000,
      },
      receipt_outcome: {
        gas_burnt: 4766255324247,
        tokens_burnt: 476625532424700000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "Gg6E3qZwLz1yoD3Urr2dqVrppZjkvGmuNpL52zmJVR7k",
      included_in_block_hash: "HGSxCUWyTLadAxB78VVGJgCehUyJh5CsCyGmqvQRJGz4",
      block_timestamp: "1731921921705748185",
      block: {
        block_height: 132970407,
      },
      receipt_conversion_tokens_burnt: "31020526717500000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "act_proposal",
          deposit: 0,
          fee: 476625532424700000000,
          args: '{"id": 115, "action": "VoteApprove"}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 529964315392200000000,
      },
    },
    {
      id: "10223561156",
      receipt_id: "G3k7EM5Z68R6A4hLEezgz6u91ByxXWq61cZoiahc3vJt",
      predecessor_account_id: "megha19.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "4H77qUvmgWQpY3nGcsKFsocgRtMRcA2E3YWqTw4WgwbQ",
        block_height: 132970137,
        block_timestamp: 1731921611526669300,
      },
      receipt_outcome: {
        gas_burnt: 4181385964947,
        tokens_burnt: 418138596494700000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "CKQ526ayqe9BTvLmrnKx4f85itmCPU5oSeGzCVBHVtXU",
      included_in_block_hash: "FsLSMBYtiRwN5rsfAYyzcYRBNYR3aGr5Rd48hSQPp5uY",
      block_timestamp: "1731921610474316671",
      block: {
        block_height: 132970136,
      },
      receipt_conversion_tokens_burnt: "48143748774000000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "add_proposal",
          deposit: 0,
          fee: 418138596494700000000,
          args: '{"proposal": {"kind": {"ChangePolicy": {"policy": {"roles": [{"kind": {"Group": ["theori.near", "freski.near", "megha19.near", "frol.near", "thomasguntenaar.near", "petersalomonsen.near"]}, "name": "Create Requests", "permissions": ["call:AddProposal", "transfer:AddProposal", "config:Finalize", "*:VoteRemove"], "vote_policy": {"call": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "vote": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "config": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "policy": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "transfer": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "add_bounty": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "bounty_done": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "upgrade_self": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "set_vote_token": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "upgrade_remote": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "add_member_to_role": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "remove_member_from_role": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}}}, {"kind": {"Group": ["freski.near", "petersalomonsen.near", "thomasguntenaar.near", "theori.near", "megha19.near", "frol.near"]}, "name": "Manage Members", "permissions": ["config:*", "policy:*", "add_member_to_role:*", "remove_member_from_role:*"], "vote_policy": {"call": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "vote": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "config": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "policy": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "transfer": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "add_bounty": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "bounty_done": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "upgrade_self": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "set_vote_token": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "upgrade_remote": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "add_member_to_role": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}, "remove_member_from_role": {"quorum": "0", "threshold": "1", "weight_kind": "RoleWeight"}}}, {"kind": {"Group": ["meghagoel.near", "freski.near", "megha19.near"]}, "name": "Vote", "permissions": ["*:VoteReject", "*:VoteApprove", "*:VoteRemove"], "vote_policy": {"call": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "vote": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "config": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "policy": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "transfer": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "add_bounty": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "bounty_done": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "upgrade_self": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "set_vote_token": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "upgrade_remote": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "add_member_to_role": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}, "remove_member_from_role": {"quorum": "0", "threshold": [10, 100], "weight_kind": "RoleWeight"}}}], "bounty_bond": "100000000000000000000000", "proposal_bond": "0", "proposal_period": "604800000000000", "default_vote_policy": {"quorum": "1", "threshold": "1", "weight_kind": "RoleWeight"}, "bounty_forgiveness_period": "604800000000000"}}}, "description": "Update policy"}}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 466282345268700000000,
      },
    },
    {
      id: "10223016065",
      receipt_id: "EVwUEVZT2yYXrQjK1MRN5ug6S4EJTFVpik6uFMTPsEto",
      predecessor_account_id: "megha19.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "93Xg5YiduUx7tSKNTJgMhRUDx5BZZHndKCYVU92i7J11",
        block_height: 132969003,
        block_timestamp: 1731920292943543800,
      },
      receipt_outcome: {
        gas_burnt: 2881363929322,
        tokens_burnt: 288136392932200000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "CXZyck2aZRLcmNgvWAqewBiYDSwSmqbbjAq63k6yYvpn",
      included_in_block_hash: "3sXd1Ef78AckkZE53Pm8KGLRPTVNawNND3tjh19mdrqW",
      block_timestamp: "1731920291635012734",
      block: {
        block_height: 132969002,
      },
      receipt_conversion_tokens_burnt: "31015758346000000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "act_proposal",
          deposit: 0,
          fee: 288136392932200000000,
          args: '{"id": 114, "action": "VoteRemove"}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 319152151278200000000,
      },
    },
    {
      id: "10222766407",
      receipt_id: "DnQ5Fesd7dCqkcBRwwadBJoTSiKsTRaDimhzL6fboaqJ",
      predecessor_account_id: "megha19.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "As9zRjFbbs3k9UG4pxoy2o3mEtgQ6putT1w2a8AGR5Uh",
        block_height: 132968706,
        block_timestamp: 1731919916394594600,
      },
      receipt_outcome: {
        gas_burnt: 2824900147771,
        tokens_burnt: 282490014777100000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "AKienJ6xV7SfkH3crtsbAMRvBbenm2TiwLsrXVcEZtUG",
      included_in_block_hash: "EjSApLjTnjXjQuyf8Z8tdXDfbATJCB9sgYPByXRTBxyB",
      block_timestamp: "1731919915119717031",
      block: {
        block_height: 132968705,
      },
      receipt_conversion_tokens_burnt: "32040958218500000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "add_proposal",
          deposit: 0,
          fee: 282490014777100000000,
          args: '{"proposal": {"kind": {"FunctionCall": {"actions": [{"gas": "200000000000000", "args": "", "deposit": "0", "method_name": "deposit_and_stake"}], "receiver_id": "bisontrails.poolv1.near"}}, "description": "{\\\\\\\\\\\\\\"isStakeRequest\\\\\\\\\\\\\\":true,\\\\\\\\\\\\\\"notes\\\\\\\\\\\\\\":\\\\\\\\\\\\\\"Testing delete\\\\\\\\\\\\\\"}"}}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 314530972995600000000,
      },
    },
    {
      id: "10221376958",
      receipt_id: "7j6EAFfVdLtQYSFiV4wkHNAnzWcvcBP1uWy3XwJrSNf1",
      predecessor_account_id: "megha19.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "GDAXThHruF5rbbhRipxwJDuaQVQPHBoK8cVvbAqcHRcm",
        block_height: 132966569,
        block_timestamp: 1731917364951132000,
      },
      receipt_outcome: {
        gas_burnt: 2889821732970,
        tokens_burnt: 288982173297000000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "7UyHZZi8tGMf5PDeuoWvuPq4tRYwXaS9DbpEK7B19kmv",
      included_in_block_hash: "CCMQSBLLBo7jQxC2SiwPqY7dAgDZZBvt6wawpsuEX4MV",
      block_timestamp: "1731917363761757699",
      block: {
        block_height: 132966568,
      },
      receipt_conversion_tokens_burnt: "31015758346000000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "act_proposal",
          deposit: 0,
          fee: 288982173297000000000,
          args: '{"id": 113, "action": "VoteRemove"}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 319997931643000000000,
      },
    },
    {
      id: "10221362690",
      receipt_id: "mGYnuc4Pd5DYhCmqH8KWT54GHtZGT2hmoiU2htGQrMK",
      predecessor_account_id: "megha19.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "EwzCZoBHvZJCR6dv8MYnLWNYeCYCqa2JQ2kUizRooEgP",
        block_height: 132966550,
        block_timestamp: 1731917343777140500,
      },
      receipt_outcome: {
        gas_burnt: 2750542007327,
        tokens_burnt: 275054200732700000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "6g3W25rEZEDxx3ZEhQyzzYM5B34ochT37W1HeUppYDnQ",
      included_in_block_hash: "6kMVzU4Tkh1UYnEEGCL4W5cw5ZwmAYzRmn3UKuUhe7Bj",
      block_timestamp: "1731917342650042995",
      block: {
        block_height: 132966549,
      },
      receipt_conversion_tokens_burnt: "32517795368500000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "add_proposal",
          deposit: 0,
          fee: 275054200732700000000,
          args: '{"proposal": {"kind": {"Transfer": {"amount": "200000", "token_id": "usdt.tether-token.near", "receiver_id": "maguila.near"}}, "description": "{\\\\\\\\\\\\\\"title\\\\\\\\\\\\\\":\\\\\\\\\\\\\\"Fellowship Contributor report by Matias Benary for 2024-10-14 2024-11-10\\\\\\\\\\\\\\",\\\\\\\\\\\\\\"summary\\\\\\\\\\\\\\":\\\\\\\\\\\\\\"Fellowship Contributor report by Matias Benary for 2024-10-14 2024-11-10\\\\\\\\\\\\\\",\\\\\\\\\\\\\\"notes\\\\\\\\\\\\\\":null,\\\\\\\\\\\\\\"proposalId\\\\\\\\\\\\\\":252}"}}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 307571996101200000000,
      },
    },
    {
      id: "10141373741",
      receipt_id: "Dw4fe7zYAzKbPxBNkMGmjo4XWvmFxLdLzhcjqnvz9oB6",
      predecessor_account_id: "megha19.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "2vyS7dZZvHECXbBkg15VJY5q4oBHn54o47YziWtaepRk",
        block_height: 132751163,
        block_timestamp: 1731674552392744000,
      },
      receipt_outcome: {
        gas_burnt: 2225363426046,
        tokens_burnt: 222536342604600000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: false,
      },
      transaction_hash: "HNxt6pzkK24umVX7kfK8HvrJ8pwcnRE3fPp7fgT38hRL",
      included_in_block_hash: "8P1Xi7tDdyjHD9f82hZMuhjyo3CMT43y3bKxhNazjZ1u",
      block_timestamp: "1731674551161124853",
      block: {
        block_height: 132751162,
      },
      receipt_conversion_tokens_burnt: "31010989974500000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "act_proposal",
          deposit: 0,
          fee: 222536342604600000000,
          args: '{"id": 77, "action": "VoteRemove"}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: false,
      },
      outcomes_agg: {
        transaction_fee: 253547332579100000000,
      },
    },
    {
      id: "9968072188",
      receipt_id: "F3bws6Mttth9jSG6bKQyEinvohwP8GM3QHLiLSvRufYL",
      predecessor_account_id: "testing-astradao.sputnik-dao.near",
      receiver_account_id: "freski.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "H55krvFqfDqmbFRjmq4gKTdz7cqcyu1DjweZnRhwgzZX",
        block_height: 132311080,
        block_timestamp: 1731166803333462500,
      },
      receipt_outcome: {
        gas_burnt: 223182562500,
        tokens_burnt: 22318256250000000000,
        executor_account_id: "freski.near",
        status: true,
      },
      transaction_hash: "pGhAU8Wo2CDhde9ti6aD9jbuxCnvovyfyV7q1dYd3pk",
      included_in_block_hash: "9WG42Y4viQ3L7ojELy9W9v8yw9YU98e8PDDifZ6fthyz",
      block_timestamp: "1731166801116539698",
      block: {
        block_height: 132311078,
      },
      receipt_conversion_tokens_burnt: "31015758346000000000",
      actions: [
        {
          action: "TRANSFER",
          method: null,
          deposit: 0,
          fee: 22318256250000000000,
          args: null,
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 375824409035600000000,
      },
    },
    {
      id: "9968071866",
      receipt_id: "58wmhctee5VWqGyrkYJQQk351nbZ6H11GHhdbjoEFtPt",
      predecessor_account_id: "freski.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "5nCNja6xtpDUw58zrdN3Q7hovrAPLhz8rsJq2HUHg16o",
        block_height: 132311079,
        block_timestamp: 1731166802311923700,
      },
      receipt_outcome: {
        gas_burnt: 3224903944396,
        tokens_burnt: 322490394439600000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "pGhAU8Wo2CDhde9ti6aD9jbuxCnvovyfyV7q1dYd3pk",
      included_in_block_hash: "9WG42Y4viQ3L7ojELy9W9v8yw9YU98e8PDDifZ6fthyz",
      block_timestamp: "1731166801116539698",
      block: {
        block_height: 132311078,
      },
      receipt_conversion_tokens_burnt: "31015758346000000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "act_proposal",
          deposit: 0,
          fee: 322490394439600000000,
          args: '{"id": 112, "action": "VoteReject"}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 375824409035600000000,
      },
    },
    {
      id: "9968066800",
      receipt_id: "41xWTUqX7aeAZjotGWfawMBdkVtvCzx2FGTsKthXhwau",
      predecessor_account_id: "freski.near",
      receiver_account_id: "testing-astradao.sputnik-dao.near",
      receipt_kind: "ACTION",
      receipt_block: {
        block_hash: "9NzGQNY6JVPzBKaYo1JHneHyMiKvxAmBHZHYvNUGWFSM",
        block_height: 132311056,
        block_timestamp: 1731166776491221000,
      },
      receipt_outcome: {
        gas_burnt: 2810879954867,
        tokens_burnt: 281087995486700000000,
        executor_account_id: "testing-astradao.sputnik-dao.near",
        status: true,
      },
      transaction_hash: "76tcfj4jMnWpwqZ3TvPCkjVbpb3mGah9s16ESnvWWnUZ",
      included_in_block_hash: "Ct696NZsrNztEQMfzrN1AvrRG1C5UTu6t8BJDAdavbJz",
      block_timestamp: "1731166774973657538",
      block: {
        block_height: 132311055,
      },
      receipt_conversion_tokens_burnt: "32031421475500000000",
      actions: [
        {
          action: "FUNCTION_CALL",
          method: "add_proposal",
          deposit: 0,
          fee: 281087995486700000000,
          args: '{"proposal": {"kind": {"FunctionCall": {"actions": [{"gas": "200000000000000", "args": "eyJhbW91bnQiOiIwIn0=", "deposit": "0", "method_name": "unstake"}], "receiver_id": "astro-stakers.poolv1.near"}}, "description": "{\\\\\\\\\\\\\\"isStakeRequest\\\\\\\\\\\\\\":true,\\\\\\\\\\\\\\"notes\\\\\\\\\\\\\\":\\\\\\\\\\\\\\"\\\\\\\\\\\\\\"}"}}',
        },
      ],
      actions_agg: {
        deposit: 0,
      },
      outcomes: {
        status: true,
      },
      outcomes_agg: {
        transaction_fee: 313119416962200000000,
      },
    },
  ],
};

let txns = nearblocks_response.txns;

txns.forEach((txn) => {
  console.log(txn.actions[0].method);
});
