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
- anvil - generate iv and key (for hammeron and hameroff, AES-128, etc)
- hammeron - AES-128 CBC encrypt (encrypt strings 127 bytes or less)
- hammeroff - AES-128 CBC decrypt (decrypt string 127 bytes or less)
- magick - hex encode
- antimagick - hex decode

The hammer tools (hammeron and hammeroff) used directly on the CLI expose the key to the local system, like in history files and process lists.
If that is a concern, we can use it indirectly, moving the sensitive data to files, RAM, etc.
The hammer tools also can only take 127 bytes of data as input to encrypt at a time.
(Use "rage" https://github.com/str4d/rage instead for more normal file encryption operations.)

While individually these little programs are not very useful, the "dwarven-toolbox" technique is to layer these tools together within scripts or other programs.

We can compile each target: `cargo build --release all` and then install the resulting target/release/ binaries to /usr/local/bin/ 
and then use those within other programs as little utilities. Or compile just the ones we want to use: `cargo build --release --bin iron`, and then perhaps we'll simply copy it to our local bin: `sudo cp target/release/iron /usr/local/bin/`.

Example script leveraging the dwarven-toolbox prorgrams:

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


