# To Streamshare (to-ss > toss)

Upload files to [streamshare](https://streamshare.wireway.ch) with the terminal.

### Install

```bash
cargo install to-streamshare
```

### Upload

```bash
toss "filepath"
toss "filepath" --chunk-size 100 # set chunk_size to 100
toss "filepath" --server "streamshare.myserver.com" # set server to your server
```

### Delete

```bash
toss --delete file_identifier/deletion_token
```
