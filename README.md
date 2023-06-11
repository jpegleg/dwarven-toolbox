# dwarven-toolbox

The dwarven-toolbox is a collection of small and simple CLI programs written in Rust.

- iron - 32 byte string generate
- silver - 64 byte string generate
- gold - 96 byte string generate
- uidgen - UUIDv4 generate
- armore - base64 encode
- darmore - base64 decode
- shielda - base58 encode
- dsheilda - base58 decode
- axor - XOR two integers
- hexor - XOR each byte of two strings, output as hex
- anvil - generate iv and key (for hammeron and hameroff, AES-128, etc)
- hammeron - AES-128 CBC encrypt (encrypt strings 127 bytes or less) ⚠️ Security Warning: Hazmat Hammers! 
- hammeroff - AES-128 CBC decrypt (decrypt string 127 bytes or less) ⚠️ Security Warning: Hazmat Hammers! 
- amuleton - create a one-time-use ed25519 keypair and sign an input and immediately throw away the private key
- amuletoff - validate any ed25519 detached signature with original data, public key, and the detached signature
- magick - hex encode
- antimagick - hex decode
- crown - BLAKE2, BLAKE3, SHA3, SHA2 hash of an input string

<b>Some of the included utilities do not ensure privacy, security, or quality. Use (hammeron|hammeroff|misc) for (educational|research) purposes unless you really know what you are doing.</b>

The hammer tools (hammeron and hammeroff) used directly on the CLI expose the key to the local system, like in history files and process lists.
If that is a concern, we can use them indirectly in some cases, moving the sensitive data to files, RAM, etc.
The hammer tools also can only take 127 bytes of data as input to encrypt at a time.

(Use "rage" https://github.com/str4d/rage instead for more normal file encryption operations, or at least another tool designed for file encryption.)

The "dwarven-toolbox" technique with the hammers is to layer these tools together within scripts or other programs, although some of these tools are directly useful. 

## Usage 

We can compile each target: `cargo build --release all` and then install the resulting target/release/ binaries to /usr/local/bin/ 
and then use those within other programs as little utilities. Or compile just the ones we want to use: `cargo build --release --bin iron`, and then perhaps we'll simply copy it to our local bin: `sudo cp target/release/iron /usr/local/bin/`.


Example script leveraging the dwarven-toolbox programs:

```
#!/usr/bin/env bash

iv=$(anvil | grep IV | cut -d':' -f2)
key=$(anvil | grep KEY | cut -d':' -f2)
mercyv=$(anvil | grep IV | cut -d':' -f2)
mercyk=$(anvil | grep KEY | cut -d':' -f2)
echo $mercyv | xargs shielda | xargs armore | xargs magick
echo $mercyk | xargs shielda | xargs armore | rev | xargs magick

hammeron $mercyv "$iv $key" $mercyk
iron > code.txt
hammeron $iv "$(cat code.txt)" $key > code.txt.enc && cp /dev/null code.txt
```

The example puts a 32 byte string into a file named code.txt, which it then encrypts. In order to decrypt it, we'll need the iv and key that were used to encrypt it. The example takes the iv and key and encrypts those with yet another iv and key, which are then printed out with a few layers of encoding as a dash of obfuscation.
In order to decrypt, the secondary iv and key must hex decoded, the key reversed, both base64 decoded then base58 decoded, then those are used to decrypt the actual iv and key, which we then finally use to decrypt the code. This is just an example of some of the infinite possibilities of the dwarven-toolbox!

#### A more normal example

We might use `crown` to quickly generate checksums.

```
$ cat some_file.txt | xargs -0 crown | tee some_checksusms.txt
BLAKE3: 4fe765cc697336191276b8052bf510f9c511bcb024be05ac95bc58e7521963ab
BLAKE2B-512: 4984ef02e17c649c2131ab19af44094b6eaf8ef07445320bf017655071200af48f5d0b01d3cce37677f996d696705c09d0005db3f3c7100f6605f8559291631a
SHA3-256: b7a0085f87df87bfbeb122eb1bafd6bf7387ac10b1673e94558d30a242a7fefc
SHA3-384: 131d5c1116e0bba5e73c48e13d2c61979c416d8a741d6bb2b5def6b51da141d909df9ecdbbd14452774562d3ded52741
SHA2: 67fc5dcc7bf595385699e9a10be5f7a8f2a4b881607632361741a1cfb5d96a8f
```

#### Forensic and research power!

There are real situations when reverse engineering, researching, and performing ad-hoc operations where we may need to do some quick data butchery. The dwarven-toolbox is built to assist with such tasks. 

Malware might use only base58 encoding or XOR in a weak attempt to obscure information. With our toolbox at hand, we can quickly investigate numerous permutations, either directly or via scripts etc.

```
$ dshielda NZ1USWfjfFYuJ5wetRMLxtv6vjWn8p
Oh this was something!
```
