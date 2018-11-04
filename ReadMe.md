# Structures
### Serialized output/utxo
- version -- *2 bytes*
- value -- *4 bytes*
- Owner -- *32 bytes*

### Serialized input
- utxo hash -- *32 bytes* <-- this is created from Sha256([utxo, blockheader])
- signature -- *67-70 bytes*

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
