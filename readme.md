# Bitcoin Inscription Demo

This crate demonstrates Bitcoin inscriptions, it generates a random string of characters of around 400kb in length and then commits this inside a Taproot transaction. Once the commit-reveal transactions are processed, we double-check that the correct contents exist on-chain by fetching the transaction data and asserting equality.

The single tapscript has the following structure:
```
OP_FALSE
OP_IF
<Pushes contents of the string into the script>
OP_ENDIF
OP_CHECKSIG
```

The if statement portion of the script acts as an "envelop" allowing us to insert arbitrary data. We push data into the envelope n times, where n is equal the length of the arbitrary data in bytes divided by 520, 520 is the maximum size limit for data pushes.

# Setup

For this I'm using a local regtest node with the following bitcoin.conf:
```
# RPC settings
rpcuser=admin
rpcpassword=admin

fallbackfee=0.0002
```

Which is started by running:
```
bitcoind -regtest -daemon
```
