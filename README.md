# dwarven-toolbox 🧰

The dwarven-toolbox is a collection of small and simple programs.

⚠️ Security Warning: Hazmat! This collection contains lower level permutations and abstractions that may not have all of the desirable security properties for a given usage/situation.

- iron - 32 byte string generate
- silver - 64 byte string generate
- gold - 96 byte string generate
- uidgen - UUIDv4 generate
- armore - base64 encode
- darmore - base64 decode
- shielda - base58 encode
- dshielda - base58 decode
- daggeron - hex to u64
- daggeroff - u64 to hex
- box - gzip compress strings (output as hex)
- unbox - gzip decompress (hex encoded gzip) strings (output as hex)
- zbox - zlib compress strings (output as hex)
- zunbox - zlib decompress (hex encoded zlib) strings (output as hex)
- axor - XOR two integers
- hexor - XOR each byte of hex encoded strings, output as hex
- swordleft - bitshift each byte left by 1, input as hex string
- swordright - bitshift each byte right by 1, input as hex string
- anvil - generate nonce, iv, and key strings (for the encryption utilities, etc)
- hammeron - AES-128 CBC encrypt (encrypt strings 127 bytes or less) 
- hammeroff - AES-128 CBC decrypt (decrypt string 127 bytes or less) 
- warhammeron - AES-256 CBC encrypt (encrypt strings 127 bytes or less) 
- warhammeroff - AES-256 CBC decrypt (decrypt string 127 bytes or less)
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
- catapult - f64 exponentiation
- crossbow - f64 multiplication
- stack - f64 addition
- smash - f64 subtraction
- scale - average, mean, median, softmax, lowest value, highest value, range, and sum

<b>Some of the included utilities do not ensure privacy, security, or quality. Use for (educational|research) purposes unless you really know what you are doing.</b>

The encryption tools used directly on the CLI expose the key to the local system, like in history files and process lists.
If that is a concern, we can use them indirectly in some cases, moving the sensitive data to files, RAM, etc.
The hammer tools also can only take 127 bytes of data as input to encrypt at a time. Also, remeber to always use new IV and KEY since we are in CBC mode for the hammers!

Use "rage" https://github.com/str4d/rage instead for more normal interactive file encryption operations, etc.  

The "dwarven-toolbox" technique with the hammers, mattocks, and halberds is to layer these tools together within scripts or other programs, designed for not relying on files to perform the operations. While some of the utilities are better with layers, others are more directly useful on the CLI. While the toolbox is not designed to work with files directly, subshell concatenation aka "$(cat mything.txt)" and redirects aka "shielda $(gold) > something.dat" can be used effectively. The tools are designed for working with arguments passed into the programs. Combined with linux "xargs", we can pipe data into the utilities that way as well.


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

The full collection of utilities compiled for x86 GNU/Linux is roughly 137.6MB. Each of those compiled utilities is currently ~4.2MB.

Using `scale` for some stats on the MB of each utility on x86 linux:
```
Average: 4.17575758
Mean: 4.17575758
Median: 4.20000000
Lowest value: 4.10000000
Highest value: 4.30000000
Range: 0.20000000
Sum of all values: 137.80000000
```
The collection of tools (not including the zlib tools `zbox` and `zunbox`) compiled for x86 Darwin (MacOS) is roughly 19.5MB. Each those compiled utilties is ~525KB.

Using `scale` for some stats on the KB of each utility on x86 darwin (MacOS Ventura):
```
Average: 525.72972973
Mean: 525.72972973
Median: 520.00000000
Lowest value: 496.00000000
Highest value: 628.00000000
Range: 132.00000000
Sum of all values: 19452.00000000
```

Note that zlib may fail to compile on some systems such as MacOS and others. Zlib is used for the zlib compression utilities `zbox` and `zunbox`. If the zlib compile fails, either install the required deps or remove zbox, zunbox, and the zlib feature of flate2 from the Cargo.toml, if you still want to use the "--all" approach. Included is a file called no-zlib_Cargo.toml which can be copied over the Cargo.toml if you want to quickly exclude the zlib tools and dependencies.

Compile without zlib (and skip the zlib utilities) example:

```
$ cp no-zlib_Cargo.toml Cargo.toml
$ cargo build --release --all
```

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

The mattock can process much larger chunks per encryption than the hammer, but they both have uses. If the hammer is to be used on larger data, the data can be broken up into smaller pieces and the encryption performed on each chunk, although in those cases the mattock is probably a better choice.

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

The programs `axe`, `crossbow`, `stack`, `smash`, `catapult`, and `saw` each take two arguments.

<b>Warning: current with regular rust maths, after 16 digits, we'll start to get some funny behavior:</b>

```
11111111111111111111111111111.99999 + 1
11111111111111112000000000000
```

Because of this behavior, and not wanting/needing to pull in special handling for that, we have `smash`, `stack`, `scale` and `axe` are set to print an error message if an argument is long enough to trigger this behavior:

```
$ stack 11111111111111111111111111111.99999 1
Error: Argument longer than 16 bytes not supported!
```

Note that `saw`, `catapult`, and `crossbow` can handle the longer math arguments, however can still run into issues with some output larger than 18446744073709551615.

```
$ catapult 3243 33
72652289731892430000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
$
```
That should be 72652289731892424036412098202541310209361726602796135670284698733348436507686376801792853211158279959138140991212843!
 But at least it is the right length and starts correctly haha. This occurs when exponentiation using (`powf`) output exceeds f64 size aka 18446744073709551615 aka 2^64. 

Large numbers can be handled using additional crate if you feel the need to implement support for that! See https://docs.rs/num-bigint/latest/num_bigint/ for more information on that subject.

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
