# l-mri
Ever17 resource extractor and repacker

**WARNING**: expect spaghetti code

## Usage
To extract the contents of a `dat` file (such as `script.dat`) to a directory:

```cargo run --bin unpack script.dat script/```

To repack the extracted files:

```cargo run --bin repack script/metadata.csv script.new.dat```
