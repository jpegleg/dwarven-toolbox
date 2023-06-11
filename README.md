# dwarven-toolbox 🧰

The dwarven-toolbox is a collection of small and simple programs.

- iron - 32 byte string generate
- silver - 64 byte string generate
- gold - 96 byte string generate
- uidgen - UUIDv4 generate
- armore - base64 encode
- darmore - base64 decode
- shielda - base58 encode
- dsheilda - base58 decode
- axor - XOR two integers
- hexor - XOR each byte of hex encoded strings, output as hex
- swordleft - bitshift each byte left by 1, input as hex string
- swordright - bitshift each byte right by 1, input as hex string
- anvil - generate iv and key (for hammeron and hameroff, AES-128, etc)
- hammeron - AES-128 CBC encrypt (encrypt strings 127 bytes or less) ⚠️ Security Warning: Hazmat! 
- hammeroff - AES-128 CBC decrypt (decrypt string 127 bytes or less) ⚠️ Security Warning: Hazmat! 
- amuleton - create a one-time-use ed25519 keypair and sign an input and immediately throw away the private key
- amuletoff - validate any ed25519 detached signature with original data, public key, and the detached signature
- magick - hex encode
- antimagick - hex decode
- crown - BLAKE2, BLAKE3, SHA3, SHA2 hash of an input string

<b>Some of the included utilities do not ensure privacy, security, or quality. Use (hammeron|hammeroff|misc) for (educational|research) purposes unless you really know what you are doing.</b>

The hammer tools (hammeron and hammeroff) used directly on the CLI expose the key to the local system, like in history files and process lists.
If that is a concern, we can use them indirectly in some cases, moving the sensitive data to files, RAM, etc.
The hammer tools also can only take 127 bytes of data as input to encrypt at a time. Also, remeber to always use new IV and KEY since we are in CBC mode for hammeron!

(Use "rage" https://github.com/str4d/rage instead for more normal file encryption operations, or at least another tool designed for file encryption.)

The "dwarven-toolbox" technique with the hammers is to layer these tools together within scripts or other programs, although some of these tools are directly useful. The toolbox is not designed to work with files directly, however subshell concatenation aka "$(cat mything.txt)" can be used effectively in some cases.  The tools are designed for working with arguments passed into the programs. Combined with linux "xargs", we can pipe data into the utilities that way as well.

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

#### Hex magick!

The program `hexor` works well with `magick` and `antimagick` in this example:

```
$ hexor 65595411564354504511415e465443115e57115546504347545f115c5856594510 313131313131313131313131313131313131313131313131313131313131313131 | xargs antimagick
The great power of dwarven might!
```

We can combine XOR and bitshift in interesting ways with `swordleft`, `swordright`, `hexor`, `magick`, and `antimagick`.
We'll add in `gold` to generate a key which we'll XOR against, shift, and XOR again, for demonstration purposes.  Example:

```
#!/usr/bin/env bash

if [[ -z "$1" ]]; then
   echo "Send in a single argument string!"
   exit 1
else
   echo "Permutation result:"
fi

inlen=${#1}

if [[ "$inlen" -gt 192 ]]; then
    echo "Input larger than 192 bytes, output truncated to 192 bytes!"
else
    :
fi

gold > gold.key

key=$(magick "$(cat gold.key)") 
plaintext=$(magick "$1")

r1=$(hexor "$plaintext" "$key")

rlength=${#r1}

iseven=$(( rlength / 2))

if [[ "$iseven" ]]; then
   :
else
   r1=0$r1
fi

r2=$(swordleft $r1)

r3=$(hexor "$r2" "$key")

echo $r3

```

Also note how we check to see if the hex is odd length before performing the bitshift. The hex output we use in dwarven-toolbox is raw, meaning that odd length values can occur. If we try to decode an odd length hex value, we'll get an error. The dwarven-toolbox utilties do not try to compensate for this, it is up to the higher level script or implementation to manage inputs in this way.

To reverse this example permutation, we would perform an XOR against the same key, shift right, then XOR that result against the key, and then hex decode.


#### Forensic and research power!

There are real situations when reverse engineering, researching, and performing ad-hoc operations where we may need to do some quick data butchery. The dwarven-toolbox is built to assist with such tasks. 

Malware might use only base58 encoding or XOR in a weak attempt to obscure information. With our toolbox at hand, we can quickly investigate numerous permutations, either directly or via scripts etc.

```
$ dshielda NZ1USWfjfFYuJ5wetRMLxtv6vjWn8p
Oh this was something!
```
