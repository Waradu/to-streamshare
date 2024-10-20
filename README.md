# To Streamshare (to-ss > toss)

Upload files to [streamshare](https://streamshare.wireway.ch) with the terminal.

### Install

```bash
cargo install to-streamshare
```

### Upload

```bash
toss "filepath"
toss --chunk-size 100 "filepath" # set chunk_size to 100
toss --server "streamshare.myserver.com" "filepath" # set server to your server
```

### Delete

```bash
toss --delete "file_identifier/deletion_token"
```

### Download

```bash
toss --download "file_identifier"
toss --download "file_identifier" --path "" # uses current path as default
toss --download "file_identifier" --replace # replace if file already exist
```