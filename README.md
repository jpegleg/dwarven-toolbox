# dwarven-toolbox 🧰

The dwarven-toolbox is a collection (monorepo) of 78 small and simple utility programs. Functionality ranges from CLI cryptography to system script optimizations.

```
Priorities for the toolbox:
 UNIX-based compatibility
 simple code
 fast and energy efficient execution
 no main.rs (each tool has a .rs file)
```
Contributing and more information can be found in CONTRIBUTING.md and CODE_OF_CONDUCT.md.


⚠️ Security Warning: Hazmat! This collection contains lower level permutations and abstractions that may not have all of the desirable security properties for a given usage/situation.

Argument tools:
- iron - 32 byte string generate
- silver - 64 byte string generate
- gold - 96 byte string generate
- uidgen - UUIDv4 generate
- armore - base64 encode
- darmore - base64 decode
- shielda - base58 encode
- dshielda - base58 decode
- daggeron - hex to BigUint
- daggeroff - BigUint to hex
- box - gzip compress strings (output as hex)
- unbox - gzip decompress (hex encoded gzip) strings (output as hex)
- zbox - zlib compress strings (output as hex)
- zunbox - zlib decompress (hex encoded zlib) strings (output as hex)
- axor - XOR two integers
- gaxor - XOR two BigInts
- hexor - XOR each byte of hex encoded strings, output as hex
- swordleft - bitshift each byte left by 1, input as hex string
- swordright - bitshift each byte right by 1, input as hex string
- anvil - generate nonce, iv, and key strings (for the encryption utilities, etc)
- aegis - generate hex values like anvil, but only output 256 random bits as 512 bit hex (for encryption utilities, etc)
- hammeron - AES-128 CBC encrypt (encrypt strings 127 bytes or less) 
- hammeroff - AES-128 CBC decrypt (decrypt string 127 bytes or less) 
- warhammeron - AES-256 CBC encrypt (encrypt strings 127 bytes or less) 
- warhammeroff - AES-256 CBC decrypt (decrypt string 127 bytes or less)
- steelhammeron - AES-256 GCM SIV encrypt (encrypt strings 127 bytes or less) with OTK (one time key)
- steelhammeroff - AES-256 GCM SIV decrypt (decrypt strings 127 bytes or less)
- mattockon - ChaCha20Poly1305 encrypt (encrypt strings) 
- mattockoff - ChaCha20Poly1305 decrypt (decrypt strings) 
- halberdon - XChaCha20Poly1305 encrypt (encrypt strings)  
- halberdoff - XChaCha20Poly1305 decrypt (decrypt strings) 
- amuleton - create a one-time-use ed25519 keypair and sign an input and immediately throw away the private key
- amuletoff - validate any ed25519 detached signature with original data, public key, and the detached signature
- magick - hex encode
- antimagick - hex decode
- crown - BLAKE2, BLAKE3, SHA3, SHA2 hashes of text (string/s) input
- saw - f64 log
- axe - f64 division
- greataxe - BigInt division
- catapult - f64 exponentiation
- trebuchet - BigInt exponetiation (u32 for the exponent)
- crossbow - f64 multiplication
- ballista - BigInt multiplication
- stack - f64 addition
- pile - BigInt addition
- smash - f64 subtraction
- crush - BigInt subtraction
- scale - average, mean, median, softmax, lowest value, highest value, range, and sum
- flip - reverse a string
- chop - print a number of bytes from the front of a string
- shave - print a number of bytes from the end of a string
- helmet - HKDF
- greathelmet - PBKDF2
- rip - count bytes in a string
- pop - pop bytes from the end of a string and prepend them back aka shift the bytes to the right by X
- seek - match exact patterns or panic (sort of like grep -o but with an error exit and with arguments only)
- order - data reduction to count the number of different bytes abcccc becomes 1a1b4c

File tools:
- inspect - print each byte of a file, along with the total bytes and bits read
- review - print detailed UNIX file data, including permissions, ownership, hashes, if the file is currently open, and more, as JSON
- chk - the condensed version of "review": report time, bytes, timestamps, and BLAKE3, as JSON
- makesigil - write an ed25519 keypair as binary to a file
- makerune - make a detached ed25519 signature from provided (makesigil) keypair file and target file
- readrune - validate a detached ed25519 signature for a file
- chiselon - XChaCha20Poly1305 encryption + PBKDF2 with user supplied salt, UTF-8 file input, ciphertext hex to STDOUT
- chiseloff - XChaCha20Poly1305 decryption + PBKDF2 with user supplied salt, custom hex file input, plaintext to STDOUT
- forgeon - XChaCha20Poly1305 encryption - file input, ciphertext to file output (binary)
- forgeoff - XChaCha20Poly1305 decryption - file input, plaintext to file output (binary)
- steelforgeon - XChaCha20Poly1305 encryption + Argon2 - file input, protected user password input km, ciphertext to file output (binary)
- steelforgeoff - XChaCha20Poly1305 decryption + Argon2 - file input, protected user password input km, plaintext to file output (binary)
- forge - XChaCha20Poly1305 encryption and decryption + Argon2 - file input, protected user password input km, file is overwritten (binary)
- ore - XChaCha20Poly1305 encryption and decryption + Argon2 - file input, environment variable key input, file is overwritten (binary)
- hexon - hex encode a file (binary) and write to a new hex file
- hexoff - hex decode a hex file and write to a new file (binary)
- clean - remove newlines and returns from file or STDIN, output to file (binary), file overwrite
- toggle - flip bits in a file (binary), file overwrite
- pack - gzip compress or decompress (inflate) a file (binary), file overwrite
- zpack - zlib compress or decompress (inflate) a file (binary), file overwrite
- hex - hex encode and decode file (binary), file overwrite
- shield - base58 encode and decode file (binary), file overwrite
- armor - base64 encode and decode file (binary), file overwrite

 
<b>Some of the included utilities do not ensure privacy, security, or quality. Use for (educational|research) purposes unless you really know what you are doing.</b>

The encryption tools used directly on the CLI expose the key to the local system, like in history files and process lists.
If that is a concern, we can use them indirectly in some cases, moving the sensitive data to files, RAM, etc.
The hammer tools also can only take 127 bytes of data as input to encrypt at a time.

Use "rage" https://github.com/str4d/rage instead for more normal interactive file encryption operations, etc.  
Or if you feel like using the file encryption tools in the dwarven-toolbox, there are some here as well. The dwarven-toolbox tools `forgeon` and `forgeoff` can be used for normal file encryption operations in an automated way, or `steelforgeon` and `steelforgeoff` for using interactive password prompt for Argon2 key material input. The tools `ore` and `forge` do both encryption and decryption, and overwrites the input file. The overwriting can be useful if that is what you intend on doing anyway, so some of the file tools use that approach to save time.

The "dwarven-toolbox" technique with the hammers, mattocks, and halberds is to layer these tools together within scripts or other programs, designed for not relying on files to perform the operations so that we can process on script variables without writing to disk if we don't want to. We also have tools that work specifically with files. 

All of these utilities enable tweakable and scriptable encryption, hashing, signing, and some maths. While some of the utilities are better with layers, others are more directly useful on the CLI. 

<b>The toolbox is organized into two types of binaries, those that work with files and those that work with args.</b>

The args tools can use subshell concatenation aka "$(cat mything.txt)" and redirects aka "shielda $(gold) > something.dat" can be used effectively. These tools are designed for working with arguments passed into the programs. Combined with "xargs", we can pipe data into the utilities that way as well.

The file tools read files from the disk and write data to files. Arguments to the file tools are paths aka file names, along with other optional args. The tool `clean` also allows STDIN piping, while the tool `ore` works with a file and an environment variable.


## Installation example

This script will compile all of the utilities and copy them to /usr/local/bin/ then clean up.

```
cargo build --release --all
for x in $(grep ^- README.md | grep -v 84 | cut -d'-' -f2); do
    echo "Installing $x"
    sudo cp target/release/$x /usr/local/bin/ 
done
cargo clean
```

Standard compiling for x86 linux of the utilities is currently ~4.2MB per utility.

Standard compiling for x86 darwin (MacOS) of the utilities is ~525KB per utility.

Note that zlib may fail to compile on some systems such as MacOS and others. Zlib is used for the zlib compression utilities `zbox` and `zunbox`. If the zlib compile fails, either install the required deps or remove zbox, zunbox, and the zlib feature of flate2 from the Cargo.toml, if you still want to use the "--all" approach. Included is a file called no-zlib_Cargo.toml which can be copied over the Cargo.toml if you want to quickly exclude the zlib tools and dependencies.

Compile without zlib (and skip the zlib utilities) example:

```
$ cp no-zlib_Cargo.toml Cargo.toml
$ cargo build --release --all
```
Before doing a git pull after copying over the Cargo.toml, move the modificatino out of the way:
```
$ mv Cargo.toml previous.toml
$ git pull
```
If you want to use the for loop from the read me, but exclude some tools, we can edit the README.md first.

```
$ vim README.md # comment out the "- forge" or any tool summary line starting in "-"
$ for x in $(grep ^- README.md | grep -v 84 | cut -d'-' -f2); do
    echo "Installing $x"
    sudo cp target/release/$x /usr/local/bin/ 
done
```
Alternatively, add more exclusions in the grep! Here is an example to exclude the base58 programs:

```
$ for x in $(grep ^- README.md | grep -v "84\|shield" | cut -d'-' -f2); do
    echo "Installing $x"
    sudo cp target/release/$x /usr/local/bin/ 
done
```

If we just want one tool, perhaps a modified version of `forge` for example:

```
$ vim src/forge.rs # Do your stuff, write the change.
$ cargo build --release --bin forge
...
$ sudo cp target/release/forge /usr/local/bin/forge
```

If we want to compile statically linked files, we might instead take an approach with docker. <b>musl builds do not have universally working UNIX timestamps, so `review` and `chk` will panic if compiled this way currently! `thread 'main' panicked at 'Failed to get created timestamp.: Error { kind: Unsupported, message: "creation time is not available on this platform currently" }', src/review.rs:54:68
`</b>

```
$ docker run -v $PWD:/volume --rm -t clux/muslrust:stable cargo build --release --all
...
$ find  target/x86_64-unknown-linux-musl/release/ -type f | grep -v ".d\|.json\|.fingerprint\|.lock" | while read line; do cp $line /usr/local/bin/ ; done
```

This example above doesn't use the README.md to choose which files to install, instead uses find within the release directory.

More installation and packaging options are coming soon! Stay tuned.


## Usage 

We can compile each target: `cargo build --release all` and then install the resulting target/release/ binaries to /usr/local/bin/ 
and then use those within other programs as little utilities. Or compile just the ones we want to use: `cargo build --release --bin iron`, and then perhaps we'll simply copy it to our local bin: `sudo cp target/release/iron /usr/local/bin/`.


Example script leveraging the dwarven-toolbox programs:

```
#!/usr/bin/env bash

iv=$(anvil | grep ^IV | cut -d':' -f2)
key=$(anvil | grep ^KEY | cut -d':' -f2)
mercyv=$(anvil | grep ^IV | cut -d':' -f2)
mercyk=$(anvil | grep ^KEY | cut -d':' -f2)
echo $mercyv | xargs shielda | xargs armore | xargs magick
echo $mercyk | xargs shielda | xargs armore | rev | xargs magick

hammeron $mercyv "$iv $key" $mercyk
hammeron $iv "$(iron)" $key > code.txt.enc
```

The example puts a 32 byte string into a file named code.txt, which it then encrypts. In order to decrypt it, we'll need the iv and key that were used to encrypt it. The example takes the iv and key and encrypts those with yet another iv and key, which are then printed out with a few layers of encoding as a dash of obfuscation.
In order to decrypt, the secondary iv and key must hex decoded, the key reversed, both base64 decoded then base58 decoded, then those are used to decrypt the actual iv and key, which we then finally use to decrypt the code. This is just an example of some of the infinite possibilities of the dwarven-toolbox!

Here is another example that uses ChaCha20Poly1305 via `mattockon` instead of AES-128 from the `hammeron`. This example also uses `box` to compress data, and leverages rage to perform file encryption on the secret key afterwards.

```
#!/usr/bin/env bash
sesh=$(uidgen)
nonce=$(anvil | grep ^NONCE | cut -d':' -f2)
plaintext=$(box "$(ps auxwww)")
key=$(anvil | grep ^KEY | cut -d':' -f2)
ciphertext=$(mattockon $nonce $plaintext $key)
echo "$nonce $ciphertext" > "$sesh".asc
shielda "$key" > "$sesh".key &&
rage -p -o "$sesh".key.age "$sesh".key &&
rm "$sesh".key
```

The mattock can process much larger chunks per encryption than the hammer, but they both have uses. If the hammer is to be used on larger data, the data can be broken up into smaller pieces and the encryption performed on each chunk, although in those cases the mattock is probably a better choice. And if really large, a file tool such as `forgeon` would be better.

The `halberdon` and `halberdoff` are stronger than the `mattockon` and `mattockoff` because the nonce is longer. The XNONCE output from `anvil` is the larger nonce used by XChaCha20Poly1305.

#### A more simple direct usage

We might use `crown` to quickly generate checksums.

```
$ cat some_file.txt | xargs -0 crown | tee some_checksusms.txt
BLAKE3: 4fe765cc697336191276b8052bf510f9c511bcb024be05ac95bc58e7521963ab
BLAKE2B-512: 4984ef02e17c649c2131ab19af44094b6eaf8ef07445320bf017655071200af48f5d0b01d3cce37677f996d696705c09d0005db3f3c7100f6605f8559291631a
SHA3-256: b7a0085f87df87bfbeb122eb1bafd6bf7387ac10b1673e94558d30a242a7fefc
SHA3-384: 131d5c1116e0bba5e73c48e13d2c61979c416d8a741d6bb2b5def6b51da141d909df9ecdbbd14452774562d3ded52741
SHA2: 67fc5dcc7bf595385699e9a10be5f7a8f2a4b881607632361741a1cfb5d96a8f
```

This style with the cat piped to the xargs -0 will give us matching results to the standard utilities b2sum, sha256sum, etc.


#### Hex magick!

The program `hexor` works well with `magick` and `antimagick` in this example:

```
$ hexor 65595411564354504511415e465443115e57115546504347545f115c5856594510 313131313131313131313131313131313131313131313131313131313131313131 | xargs antimagick
The great power of dwarven might!
```

We can combine XOR and bitshift in interesting ways with `swordleft`, `swordright`, `hexor`, `magick`, and `antimagick`.
We'll add in `gold` to generate a key which we'll XOR against, shift, hex encode, and XOR again, for demonstration purposes.  Example:

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
iseven=$(( rlength / 2 ))

if [[ -z "$iseven" ]]; then
   :
else
   r1=0$r1
fi

morph=$(magick $r1)
r2=$(swordright $morph)
r3=$(hexor "$r2" "$key")

echo $r3
```

Also note how we check to see if the hex is odd length before performing the bitshift. The hex output we use in dwarven-toolbox is raw, meaning that odd length values can occur. If we try to decode an odd length hex value, we'll get an error. The dwarven-toolbox utilties do not try to compensate for this, it is up to the higher level script or implementation to manage inputs in this way.

To reverse this example permutation, we would perform an XOR against the same key, hex decode, shift right, then XOR that result against the key, and then hex decode.

#### Hex to non-UTF8 (binary)

The `hexon` and `hexoff` tools do hex encoding and decoding of files (binary). In this way, `hexoff` can perform the "hex compiler" trick that xxd can do, something most other tools lack as they refuse to write the binary output. The `hexoff` program can safely write the binary output because it writes it to a file, there is no risk of non-UTF8 to STDOUT. 

The `hexon` and `hexoff` tools read and write files in chunks so they can process files larger than the available RAM.

If we don't use `hexon` to create the hex file, we might have extra newlines present in the file. To ensure no newlines or returns added in the file we can `clean` the file first, as `hexoff` will refuse to process newlines or anything that isn't hex.

```
$ vim something.hex
$ clean something.hex
$ hexoff something.hex something.bin
```
There is also `hex`, which overwrites the file with the hex version.

```
$ hex something.bin
Data encoded in hex and written to file: something.bin
$ hex -d something.bin
Hex data decoded and written to file: something.bin
```

Much like `hex`, we also have `shield` for base58 and `armor` for base64 overwrite file encoding.

#### Forensic and research power!

There are real situations when reverse engineering, researching, and performing ad-hoc operations where we may need to do some quick data butchery. The dwarven-toolbox is built to assist with such tasks. 

Malware might use only base58 encoding or XOR in a weak attempt to obscure information. With our toolbox at hand, we can quickly investigate numerous permutations, either directly or via scripts etc.

```
$ dshielda NZ1USWfjfFYuJ5wetRMLxtv6vjWn8p
Oh this was something!
```

One of the interesting properties of the encoding/decoding utilities in the dwarven-toolbox is that they refuse to output non-utf8 to STDOUT. This can be useful when examining malicous data, avoiding various scenarios where binary in STDOUT might do something such as attempt execute an exploit or trigger a bug/crash. While such an exploit is unlikely, another reason this property is useful is that only "successful" string decoding will output from our decode, which is what we want when we are hunting for plaintext. Normal utilities like base64 and xxd will just happily output whatever, while `darmore`,`antimagick`, and `dshielda` will intentionally panic:

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Utf8Error { valid_up_to: 2, error_len: Some(1) }', src/dshielda.rs:7:57
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
This is a huge blessing during brutefoce/batch research tasks attempting to extract plaintext, because success will have a good exit code!

If we do want to output non-UTF8 on the decode, then we can use the file tool hexoff which will write out decoded binary to a file.

There are many manipulation techniques, but some of the most common are XOR, encoding, and flipping bits.

We might find that malware flips bits of files in a weak attempt to hide or obscure. We can flip the bits of a file with the `toggle` tool.

```
$ toggle data.bin
Bits flipped and written to file: data.bin
```

Toggle overwrites the original file with the flipped version. We can flip it back to the way it was by running the program again.

#### Detailed reporting with review and inspect

Examine files closely with `review` and `inspect` file operation tools.

```
$ echo "down in the earth" > story.txt
$ inspect story.txt
Byte vector: [100, 111, 119, 110, 32, 105, 110, 32, 116, 104, 101, 32, 101, 97, 114, 116, 104, 10]
Each byte as hex: 64 6F 77 6E 20 69 6E 20 74 68 65 20 65 61 72 74 68 A 
Number of bytes: 18
Number of bits: 144
$ review story.txt
{
"story.txt": {
  "Report time": "2023-07-18 02:59:21.583803826 UTC",
  "Number of IO blocks": "8",
  "Block size": "4096",
  "Inode": "899901",
  "Number of bytes": "18",
  "Number of kilobytes": "0",
  "Number of megabytes": "0",
  "Number of bits": "144",
  "Byte distribution": "0.6666666666666666",
  "Created timestamp (UTC)": "2023-07-18 02:42:59.993219798 UTC",
  "Modified timestamp (UTC)": "2023-07-18 02:45:58.244601146 UTC",
  "Accessed timestamp (UTC)": "2023-07-18 02:46:16.302310484 UTC",
  "Changed timestamp (UTC)": "2023-07-18 02:45:58.244601146 UTC",
  "Permissions": "100644",
  "Owner": "admin (uid: 1000)",
  "Group": "admin (gid: 1000)",
  "Open": "File is not open by another program.",
  "BLAKE3": "a7fa0cfa9587aabd7cfb6506665c2f2475543b3c60da30d0e4e3c9ceb4e76dd7",
  "BLAKE2B-512": "561c9e2ea545ebdcc065243141382c17c63f07527f648648b63fed4ab6e1dba472cd30cf70f7c4e286ccd5f79303a3b9088b629c5a7eb1b5bcfd607a9e9e78a0",
  "SHA3-256": "3debb73ac91f7292c9d91eb98abf86c289d102654bdbab48ebc4ca1422f6a7c2",
  "SHA3-384": "5e0a873485771f77aba74430820f5f4fd320a332f6f299642f4bbf779826b493a8a2ca208e68dfc97eb911195cf614f9",
  "SHA2": "6b3a3eb380b929fca01fbe6047fec6b4b3a225385dfe4ce9c68233635fdee61c"
  }
}
$ 

```

The byte distribution is a float between 0 and 1 or NaN. The result is 1 if every byte is unique, and the will lower based on the ratio of repeating bytes, lower numbers mean more repeating characters. The value doesn't go below zero except for NaN for no data at all.

The "open" check is to see if the file is currently open by another process. The `review` tool output is JSON for the file object, so it can be utilized by applications more easily.

The tool `chk` is like a condensed version of `review`, which is more a more focused output, leaner computations, and can handle larger files and output faster than `review` can because of that.

```
$  chk story.txt
{
"story.txt": {
  "Report time": "2023-07-23 03:13:51.190472088 UTC",
  "Number of bytes": "18",
  "Created timestamp (UTC)": "2023-07-23 03:13:48.704125247 UTC",
  "Modified timestamp (UTC)": "2023-07-23 03:13:48.704125247 UTC",
  "Accessed timestamp (UTC)": "2023-07-23 03:13:48.704125247 UTC",
  "Changed timestamp (UTC)": "2023-07-23 03:13:48.704125247 UTC",
  "BLAKE3": "a7fa0cfa9587aabd7cfb6506665c2f2475543b3c60da30d0e4e3c9ceb4e76dd7"
  }
}
$
```

#### Maths!

There are a number of basic math operations as individual utilities. The program `scale` takes any number of arguments and reports on several properties.

```
$ scale 234 3 23 43 
Average: 75.75000000
Mean: 75.75000000
Median: 33.00000000
Lowest value: 3.00000000
Highest value: 234.00000000
Range: 231.00000000
Sum of all values: 303.00000000
Softmax: [0.17487770452710943, 0.17487770452710943, 0.17487770452710943, 0.4753668864186717]
```

Note: `scale` softmax normalization currently can't take values greater than 709, larger values can result in a NaN during softmax normalization, which will result in a Panic.
Removing the softmax functionality will lift the max up to the 16 byte length. I've moved the softmax to the last computation so that the others will complete before a normalization panic. If we know we are going to use `scale` with numbers larger than 709, we can tuck away the softmax panic like so:

```
$ scale 1 2 3 999 3 33333333333333 3333333333333333 2>/dev/null
Average: 480952380952524.87500000
Mean: 480952380952524.87500000
Median: 3.00000000
Lowest value: 1.00000000
Highest value: 3333333333333333.00000000
Range: 3333333333333332.00000000
Sum of all values: 3366666666667674.00000000
```

The programs `axe`, `crossbow`, `stack`, `smash`, `catapult`, and `saw` each take two f64 arguments.

<b>Warning: current with core rust maths, after 16 digits, we'll start to get some funny behavior:</b>

```
11111111111111111111111111111.99999 + 1
11111111111111112000000000000
```

Because of this behavior, and not wanting/needing to pull in special handling for that, we have `smash`, `stack`, `scale` and `axe` are set to print an error message if an argument is long enough to trigger this behavior:

```
$ stack 11111111111111111111111111111.99999 1
Error: Argument longer than 16 bytes not supported!
```

Alternately, if we want large numbers and don't need floats, we can use `greataxe`, `ballista`, `pile`, `crush`, and `trebuchet` which take BigInt values.

The BigInt tools can do very large numbers, like up to ~ `3.079 x 10^22212093154093428519` it seems, which is much much larger than the max f64 (64-bit integer) of 18446744073709551615, aka (2^64)-1.
Although the max length of command line arguments in the millions of digits long will get a "command line argument too long" error before we reach the BigInt max.

Note that `saw`, `catapult`, and `crossbow` can input the longer math arguments, however can also run into issues with some output larger than 18446744073709551615 as they use 64 bit integers.

```
$ catapult 3243 33
72652289731892430000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
$
```
That should be 72652289731892424036412098202541310209361726602796135670284698733348436507686376801792853211158279959138140991212843!
 But at least it is the right length and starts correctly haha. This occurs when exponentiation using (`powf`) output exceeds f64 size aka 18446744073709551615 aka 2^64. 

 Our BigInt version `trebuchet` handles that correctly:

```
$ trebuchet 3243 33
72652289731892424036412098202541310209361726602796135670284698733348436507686376801792853211158279959138140991212843
$
```

The `saw` program does not yet have a BigInt version. And the BigInt versions currently only take whole numbers. Implementing huge floating point numbers is a TODO.

#### Pure BASH vs toolbox'd BASH

Here is an example script using some of the dwarven-toolbox math utilties together within BASH:

```
#!/usr/bin/env bash

costs=$1
income=$2
account=$3

for months in {1..72}; do
    pay=$(stack $account $income)
    account=$(smash $pay $costs)
    account=$account
    echo "End of month $months total: $account"
done 
```

Although pure BASH is much faster in this case:

```
#!/usr/bin/env bash

costs=$1
income=$2
account=$3

for months in {1..72}; do
    pay=$(( $account + $income ))
    account=$(( $pay - $costs ))
    account=$account
    echo "End of month $months total: $account"
done 
```

While pure BASH functionality could be used instead for this last example, and can be faster for some BASH tasks (what a surprise). But another aspect is that math in BASH has some odd behavior we may want to avoid.

```
$ x=$((11111111111111111111111111 + 1))
$ echo $x
-8480526731661512248
```

Going back to our BASH examples, comparing time and CPU of our cost calculators, the pure BASH version outperforms significantly:

toolbox'd bash cost calc: 0.2 seconds, 0.6 seconds CPU

pure bash cost calc example: less than 0.00 seconds, 0.01 seconds CPU

This is the case because of the forking the bash is doing in the toolbox'd version that the pure version does not need to do.
If we compare single operations, say just some addition, then they are much more comparable and our toolbox shines:

```
$ cat test
echo "$(( 111111111111111 + 111111111111111 ))"
$ time bash test
222222222222222
bash test  0.00s user 0.00s system 75% cpu 0.005 total
$ time stack 111111111111111 111111111111111            
222222222222222
stack 111111111111111 111111111111111  0.00s user 0.00s system 58% cpu 0.005 total
$ time python3 -c 'print(111111111111111 + 111111111111111)'
222222222222222
python3 -c 'print(111111111111111 + 111111111111111)'  0.03s user 0.01s system 85% cpu 0.046 total
```

Now we are seeing a different story, with the toolbox equal or faster and with less CPU. Note that if BASH isn't invoked again and the shell is already BASH `echo "$(( 111111111111111 + 111111111111111 ))"`, the BASH math alone like that is extremely fast and the `time` program won't measure it or measure it as all 0s. I included python in this comparison to further illustrate the inefficiency of python in comparison as well.

BASH math (which is in C) is actually very useful and we can of course write handling for those odd BASH cases if needed and use it just as we are with the argument length property in the dwarven-toolbox. Even so, having the snappy toolbox options to sanity check or escape the constraints and control the behavior more is part of why some of these simple math utilties are included here. While the standard math operations are often going to be superior in pure BASH within BASH, or just in C for that matter, there are yet scenarios that benefit from the toolbox for simple math tasks.

#### compression of strings

We can perform gzip compression on strings with `box` and gzip decompression with `unbox`. The output is hex encoded, which may be much larger than the original. And of course, not all data will be smaller when compressed.

```
$ echo -n "The great forges have made some of the most powerful weapons in the kingdom. The great forges have made some of the most powerful weapons in the kingdom. The great forges have made some of the most powerful weapons in the kingdom. The great forges have made some of the most powerful weapons in the kingdom. The great forges have made some of the most powerful weapons in the kingdom. The great forges have made some of the most powerful weapons in the kingdom. The great forges have made some of the most powerful weapons in the kingdom. The great forges have made some of the most powerful weapons in the kingdom." > t
$ stat t
  File: t
  Size: 615             Blocks: 8          IO Block: 4096   regular file
Device: 0,43    Inode: 446098      Links: 1
Access: (0644/-rw-r--r--)  Uid: ( 1000/adminx)   Gid: ( 1000/adminx)
Access: 2023-06-13 18:40:17.348422483 -0400
Modify: 2023-06-13 18:41:08.606949565 -0400
Change: 2023-06-13 18:41:08.606949565 -0400
 Birth: 2023-06-13 18:40:17.348422483 -0400
$ box "The great forges have made some of the most powerful weapons in the kingdom. The great forges have made some of the most powerful weapons in the kingdom. The great forges have made some of the most powerful weapons in the kingdom. The great forges have made some of the most powerful weapons in the kingdom. The great forges have made some of the most powerful weapons in the kingdom. The great forges have made some of the most powerful weapons in the kingdom. The great forges have made some of the most powerful weapons in the kingdom. The great forges have made some of the most powerful weapons in the kingdom." | xxd -r -p > t.gz
$ stat t.gz
  File: t.gz
  Size: 95              Blocks: 8          IO Block: 4096   regular file
Device: 0,43    Inode: 446096      Links: 1
Access: (0644/-rw-r--r--)  Uid: ( 1000/adminx)   Gid: ( 1000/adminx)
Access: 2023-06-13 18:39:59.337129516 -0400
Modify: 2023-06-13 18:44:08.936967784 -0400
Change: 2023-06-13 18:44:08.936967784 -0400
 Birth: 2023-06-13 18:39:56.443171214 -0400
```

Here we observed the compression reducing the file size. Note how we used xxd to decode the hex produced by `box` to binary. Our `antimagick` hex decode is not for this purpose, as it will refuse to decode non-utf8. Even if we don't decode the hex of the gzip string like that, we still have size reduction in this case.

```
$  box "The great forges have made some of the most powerful weapons in the kingdom. The great forges have made some of the most powerful weapons in the kingdom. The great forges have made some of the most powerful weapons in the kingdom. The great forges have made some of the most powerful weapons in the kingdom. The great forges have made some of the most powerful weapons in the kingdom. The great forges have made some of the most powerful weapons in the kingdom. The great forges have made some of the most powerful weapons in the kingdom. The great forges have made some of the most powerful weapons in the kingdom." > t2.gz.hex
# stat t2.gz.hex
  File: t2.gz.hex
  Size: 191             Blocks: 8          IO Block: 4096   regular file
Device: 0,43    Inode: 446142      Links: 1
Access: (0644/-rw-r--r--)  Uid: ( 1000/unseenwork)   Gid: ( 1000/unseenwork)
Access: 2023-06-13 18:47:53.587370312 -0400
Modify: 2023-06-13 18:47:53.591367265 -0400
Change: 2023-06-13 18:47:53.591367265 -0400
 Birth: 2023-06-13 18:47:53.587370312 -0400
```

We can use `box` and `unbox` within our other programs to shrink some large variables in RAM.

```
bigpsout="$(ps auxwww)"
gpsout=$(box "$BIGSTUFFS")
unset bigpsout # empty the original
# some time later when we need to inflate the data again
obigpsout=$(unbox $COMPRESSEDSTUFFS | xargs antimagick)

```

We might also find forensic/IR use for `unbox`, such as when malware uses gzip compression or when we need to reverse engineer something and have to deal with ad-hoc gzip data. 

There are also the `zbox` and `zunbox` tools, which are the same style but use zlib instead of gzip.

### Compression and decompression of files

The `pack` tool does both gzip compression and decompression on files. It works in chunks so it can process large files, including binary. Pack will write out to a temporary file and then move the temporary file over the original.

```
$ pack mydata.txt
Data compressed and written to file: mydata.txt
```

Note how `pack` does not change the file extension at all and overwrites the original target.

To decompress (inflate), we'll pass the -u argument:

```
$ pack -u mydata.txt
Data decompressed and written to file: mydata.txt
```
The `zpack` tool is the same as `pack` except that it uses zlib instead of gzip compression library.

# Encryption script example

This example bash script leverages the dwarven-toolbox utilities to perform XChaCha20Poly1305, with the symmetric password generated from 21,000 round PBKDF2.

The salt to the PBKDF2 is sent in as an arg and not preserved in the file, so it must be remembered or recorded separately. Note that the salt is also leaked
to history/process data because it is used as an argument for the process.

The `gold` program from the dwarven-toolbox is used to generate an on-disk password seed, which is transformed with `daggeron`, `daggeroff`, and `greataxe` before
used along with the salt in the PBKDF2. The division from greataxe technically weakens the strength of the input key material, but a calculated risk in this 
case to increase the obfuscation of the input key material. This obfuscation can of course be removed or adjusted, one of the benefits of using dwarven-toolbox
is that we are able to use "tweakable" and "scriptable" encryption.

The input file is removed with the GNU/linux "shred" program, and the ciphertext file is created from the original file name + a_$(date +%Y%m%d%H%M%S%N), so 
the ciphertext files have a timestamp built into the file name.

The file can't be large as all of the data is read as an argument by halberdon, so max commandline argument length will stop larger files from being encrypted.
<b>This will result in data loss in the example, as files too large will get shred and a ciphertext file with only a nonce in it will be created!</b>

Use with caution, and enjoy!

```
#!/usr/bin/env bash
#

alength=$(rip "$(cat ~/.bound)" 2>/dev/null)
if [ -z "$alength" ]; then 
    echo "Mining new gold..."
    gold | xargs -0 magick > ~/.bound
fi

alength=$(rip "$(cat ~/.bound)" 2>/dev/null)
if [ "$alength" -ne "194" ]; then
    echo "Mining new gold..."
    gold | xargs -0 magick > ~/.bound
else
    echo "Bound in gold!"
fi

gold=$(cat ~/.bound)
intup=$(daggeron $gold)
divider=$(greataxe $intup 127)
inthex=$(daggeroff $divider)
kdfm=$(greathelmet $inthex "$3" 21000)
contents="$(cat $2)"
nonce=$(anvil | grep ^XNONCE | xargs -0 flip | xargs -0 chop 24)

if [ "$1" == "enc" ]; then
    echo -n "$nonce " | tee "$2".a
    halberdon $nonce "$contents" $kdfm | tee -a "$2".a &&
    shred -n 25 -u -z "$2"
    mv "$2".a "$2".a_$(date +%Y%m%d%H%M%S%N)
else
    echo -n "$contents" | cut -d' ' -f2 > "$2".2a
    ciphertext="$(cat "$2".2a | tr -d '\n')"
    enonce=$(chop 24 "$(cat $2 | tr -d '\n')")
    halberdoff $enonce "$ciphertext" $kdfm
    shred -n 25 -u -z "$2".2a
fi
```

Tis example shows that processing files can be done even with the argument tools.
Since we can hit a maximum length of CLI arguments, this could be further adjusted to chunk larger files so that they can be processed. Or instead, just rewritten
in Rust, referencing the dwarven-toolbox, and then expanding the new impelementation to read and write files, etc. Rapid prototyping is another use for the dwarven-toolbox!

This script in BASH using the utilities was the prototype for `chiselon` and `chiseloff` file utilties. Those utilities don't have the adjustable nature
and the integer math tricks, but does do the XChaCha20Poly1305 via PBKDF2, working with files instead of arguments, and very quickly considering it does 2100 rounds in the PBKDF2. 


### Aren't CLI args a bad place for secrets? Yes, args can leak! Let's talk about it.

As mentioned several times, we do have potential process argument leakage, as process arguments are not designed to be hidden from the (GNU/Linux) default operating system. But for that matter, not much truly evades the "root trace view" on the standard linux local system. Even rage (age) has different types of leakage, such as from the password in the write syscall. And gpg can leak passphrases to the local system as well from arguments or read syscalls. But process args are less protected than syscall trace exposures in default GNU/linux because non-root users can theoretically detect the process args more often than process syscall tracing. In short, trying to hide from root-level tracing in linux is a lost game, but we can take measures against process argument leakage. We see many times that "EDR" and security systems will trace all executions and forward trace data off to a centralized logging system, which may contain exposed data!

Another good reminder is that xargs does not hide process arguments either, they are still there. But changing the program to read from STDIN does not leak to args, although still has the read syscall leakage.

All that said, we should use caution about what is used in process arguments and how that is designed. The dwarven-toolbox intentionally uses process arguments, and so we should expect local leakage if we use these tools and put secrets in the arguments. Encapsulating the args within child processes that run quickly and also don't expose to history files is a start, which the above example does for the plaintext and the input key material, while exposing the salt to the history/parent args. But the child process args still expose data.

Another thing to note is that most of the time process args are truncated in tracing, so only the first 32 bytes or so are actually read. We can exercise this trickery but unfortnately tracing theoretically can capture the entire thing, and also leaking the first 32 bytes of a secret is, in some cases, fatal to the security of that secret.

Most threat models don't expect the attacker to have local access and be tracing things, however when we do model those scenarios, we should consider carefully which data can be collected this way too, not just history file leaks. Using "strace -f" is a great way to hunt in this regard, and we can also use those same EDR tools which largely use kernel modules and eBPF to perform the call tracing. 

Be aware of your CLI history files and the history/logging data usage, clean up after yourself when appropriate, and consider the risks. The dwarven-toolkit is scriptable and tweakable and does not guarantee security of implementation. In fact, not many tools, software, or systems can really claim that! Because people are involved, all systems can fail or be used incorrectly.

#### So, the file tools solve these arg leak problems, right?

There are tools in the toolbox that use files instead of args. These tools also make syscalls that can leak information, however the more likely attack vector here is access to the disk itself.

The file reads also leak data in syscalls:

```
lseek(3, 0, SEEK_CUR)                   = 0
read(3, "dang\n", 8)                    = 5
read(3, "", 3)    
```

That said, file processing is useful for some things, so we can use the file tool instead of args if leaking args is an issue, or if we just want to work on files!

Using files is slower performance, and creates more IO activity that can be avoided by working entirely with arguments.

#### But what about STDIN piping?

As of now, the dwarven-toolbox does not allow STDIN piping normally, without the aid of xargs etc. The only exception to this currenlty is `clean`, which accepts STDIN piping and
will write out to the file passed as the first argument.

```
$ cat somefile.txt | clean somefile.flat
Newlines and returns removed from piped data and written to file: somefile.flat
$
$ clean anotherfile.txt
Newlines and returns removed from file: anotherfile.txt
$ silver | clean new.key
Newlines and returns removed from piped data and written to file: new.key
```
Back to the subject, so does data piping help security? Most coreutils allow data piping, and this is the UNIX way to pipe commands around...

But STDIN piping doesn't save us from syscall leakage either:

```
read(3, "dang\n", 131072)               = 5
write(1, "dang\n", 5)                   = 5
read(3, "", 131072)                     = 0
```
STDIN piping is the GNU coreutils standard approach it seems, so we'll do something different with dwarven-toolbox and embrace arguments or use files instead.
STDIN piping has less leakage than arguments,  because "ps" process data is more accessible than syscall trace data. Lower privileged users can access process data by default in GNU/linux, including the arguments.

We have x2 the risk with STDIN on syscall leakage because we have two different syscalls that leak the data.

Leaking data at some level is more than likely going to happen. It is about how we deal with it, what our overall controls are, and how the overall security of the system is designed.

So how can we optimize user supplied input without args, without piping between commands, and without expoosing plaintext to the disk? Well, there is still a read syscall, but we can use rpassword crate approach to interactively read a password from the user. This is a useful technique when we want to avoid the disk and avoid arg leakage. See the `forge`, `steelforgeon`, and `steelforgeoff` tools for an example which takes the user supplied password, then instead of using it directly, takes those bytes along with a salt and puts them through Argon2, to generate the final key material for the XChaCha20.

But as I mentioned, that still leaks a read syscall (which is hard to avoid). Most software leaks at least a read. Age and gpg have at least as much leakage.

```
read(4, "dang\n", 8192)                 = 5
```

But the interactive read does prevent arg leak, so it is useful for that reason. It also theoretically avoids storing the secret on disk fully,
although one could pipe into an "expect" and to automate the interactive prompt too.

So don't let unauthorized people on the system in the first place, and none of these leaks are an issue :)
And then further, if unauthorized people/things get in the system, args are potentially leaked, but they will typically need some more permissions to get a trace for syscall leakage.
Most of the time that means root or the same user as the process at least. So if you do let unauthorized people in, don't let them be root.

### What about environment variables?

Environment variables are another place some people will stick secrets. Environment variables also leak in differnt ways. The most obvious is that the user which has the ENV var can print their ENV vars. The less obvious is that systemd also reads all the environment variables. Many different software configurations or debug setups may dump all the environment variables, which then exposes any secrets set there.

In the dwarven-toolbox we have the `ore` tool which uses input key material in an environment variable, so we have this as an option. While there are obvious risks depending on the local system configuration, Environment variables have the advantage of being able to avoid the disk and process arguments. If we do choose to put secrets into environment variables, then we likely want to ensure that the operating system is configured to not expose it unexpectedly, or at least we should be aware of how it can get exposed. In a cloud native context inside a minimized container, environment variables are stronger: there is no systemd, there is not even a shell or debug mechanism, so leakage is small... until a tracer or debugger is attached at least. We might have eBPF or kernel based tracing within the containers of a cluster. We might also have debugging tools that may attach to processes and additionally leak information from environment variables. Any type of debug shell will expose the environment variables of at least the given user.

So things leak, local access is local access, which means a lot in a default GNU/Linux system. Systems that properly implement MAC (mandatory access controls) may be able to further limit leakage by blocking access at the kernel level.  


## chisel file tools

<b>These chisel tools work with text (UTF-8) files only! Binary files will encrypt, but refuse to decrypt as the default decryption parses plaintext to UTF-8.</b>

When using the `chiselon` and `chiseloff` tools, newlines will break the decryption because the entire file is read. So when writing the ciphertext files, take care to not let newlines in. While we could use `tr -d '\n' > text.enc` here, we also have a tool for that in the toolbox called `clean`. Clean takes either piped data or processes files in place, writing out to the path specified.

```
$ chiselon text.txt ~/.tmp/weekly.select 000000000000tank.985G.b | clean text.enc
```

The ciphertext has mixed capitalization in the hex! This is because the NONCE (first 24 bytes) is upper hex, and the rest of the text is lower hex.
When `chiseloff` reads these values, they are processed independently within the functionality.

PBKDF2 is not ideal in some ways, however we are not using it for password hash storage, which is the context where PBKDF2 is good but not as good as Argon2.
But in `chiselon` and `chiseloff`, PBKDF2 is used for permutation of key material at time of encryption and decryption and is not involved in storage at all.
PBKDF2 used in this way is more like the HKDF, approach for ephemeral optimization of byte distribution in key material.

The user supplied salt is an interesting choice in these tools. Having the salt in an argument like that is like adding a leaky password multiplier that still adds overall benefit, even if leaked. The salt in this case is not used in the encryption itself, but the PBKDF2 of the key material. So it's security isn't important to the overall security. But the fact that it does exist increases the security. Just don't forget your salt values! You need them to decrypt, and they are not in the output or in any file by default.

When scripting with the chisel tools, some fun can be had with the salts being results of other permutations, etc.

I would not recommend the chisel tools for large files because the file is loaded into memory entirely with the chisels, however small and medium files work great. 

Whatever supplies the input to the chisels will need to judge how much to put in at once. Something like 48MB may be a reasonable max size for the `chiselon` and `chiseloff` files. I tested with a 1GB file and it got the emulator OOM killed. The intent is for encrypting UTF-8 (text) config files and secrets on the disk, etc. If binary or larger files are needed, use the forge tools instead.

## forge file tools

If you want to use an interactive password, then the either `forge` or the `steelforgeon` and `steelforgeoff` tools are for that. Combining XChaCha20Poly1305 with Argon2, reading
a user supplied password which is sent in to Argon2 along with a changing salt to create the key input material to XChaCha20. 

Here is an example of the `steelforgeon` encrypting a key file and then `steelforgeoff` decrypting.

```
$ steelforgeon ~/.keys/forge ~/.keys/forge.e # encrypt key to forge.e 
Password: 
Encryption completed successfully.
$ steelforgeoff ~/.keys/forge.e ~/.keys/forge # decrypt key from forge.e
Password:
Decryption completed successfully.
```

And here is an example `forge` for both encryption and decryption, overwriting the original file.

```
$ forge data.txt
Password: 
Data encrypted and written to file: data.txt
$ forge -d data.txt
Password: 
Data decrypted and written to file: data.txt
$ 
```

This is a strong approach that keeps the key input in the head of the user and away from the system. Encrypting user files this way can make sense.

If you want a fully automated approach with a secret input on the disk, then `forgeon` and `forgeoff` are the tools to use instead.

The `forgeon` and `forgeoff` file tools do not use the PBKDF2 or Argon2 rounds on the key material and instead use a BLAKE3 hash of the input key material to create the secret key used.

```
$ forgeon data.txt ~/.keys/forge data.e # encrypt to data.e 
Encryption completed successfully.
$ forgeoff data.e ~/.keys/forge data.txt # decrypt to data.txt
Decryption completed successfully.
$

```

The forge tools can also handle non-UTF-8 data, which the chisel tools cannot on decrypt by default to protect STDOUT.
The forge tools have file output, so they can safely encrypt and decrypt any type of file, including binaries, etc.
The forge tools are designed to process larger files. They are able to do this more effectively because of in-place AEAD processing, which does not need alloc or std.

The choice to use the sigil files from `makesigil` is for interoperability with the dwarven-toolbox file tools. The key input can be any 512 bit file that is desired.

The files encrypted by the forge tools are binary, not hex encoded etc. This reduces disk usage compared to encoded, and they can always be encoded later if needed etc.

Here is an example base64 encoding the ciphertext file.

```
$ armor data.e
Data encoded in Base64 and written to file: data.e
$
```
## ore file tool

Much like `forge`, `ore` leverages XChaCha20Poly1305 and Aron2, however `ore` is not interactive, nor does it read the input key material from disk. This tool reads an environment variable from "DWARF" in the OS, taking the value along with a salt as the input key material to Argon2. This tool is great for automation approaches. We might choose to expose our input material to the disk or a secret management system so we don't lose it, if that is important to us.

Here we'll set the "DWARF" value to a UUIDv4.

```
$ export DWARF=$(uidgen)
$ echo "$DWARF" | clean loader.dwarf
Newlines and returns removed from piped data and written to file: loader.dwarf
$
```

Then to load that key material to the system later:

```
$ export DWARF=$(cat loader.dwarf)
```

Using `ore` example:

```
$ ore somedata.csv
Data encrypted and written to file: somedata.csv
$
```
