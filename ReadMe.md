# Structures

*TODO:*
- *utxo changed to only include utxo offset instead of whole hash*
- *Transaction needs only one signature instead of signing each utxo input*
- *networking*
- *change pow algo from sha256 to something more asic resistant. (Maybe Nerva's algo?)*

### Serialized output/utxo
- version -- *2 bytes*
- value -- *4 bytes*
- Owner -- *32 bytes*

### Serialized input
- utxo hash -- *32 bytes* <-- this is created from Sha256([utxo, blockheader, index in utxos of block])
- sig size -- *1 byte*
- signature<sup>sig size</sup> -- *67-70 bytes*

### Serialized transaction
- version -- *2 bytes*
- input count<sup>n</sup> -- *1 byte*
- *serialized inputs*<sup>n</sup> -- *99-102 bytes*
- output count<sup>z</sup> -- *1 byte*
- serialized outputs<sup>z</sup> -- *42 bytes*

### Serialized block
- version -- *2 bytes*
- prev block hash -- *32 bytes*
- hash of all tx -- *32 bytes*
- nonce -- *4 bytes*
- *serialized transactions*<sup>y</sup>
