# Swish :rocket:

A Command Line Tool for [Infomaniak](https://www.swisstransfer.com/)'s SwissTransfer Service. :cloud:

This project is not affiliated with Infomaniak and is not an official tool. :warning:


## Installation

All the binaries are available in the [releases](https://github.com/sctg-development/Swish/releases) section. :inbox_tray:

Download the binary for your platform and add it to your PATH. :arrow_down:

## Usage

```sh
Usage: swish [OPTIONS] <FILE>

Arguments:
  <FILE>  could be a file or a folder or a link

Options:
  -p, --password <password>    Sets the password for the file(s) downloaded / uploaded (not encrypted)
  -a, --aes-password <aes>     Define the password for derivating the AES encryption key for the file(s) uploaded (will be uploaded as a 7z encrypted)
  -m, --message <Hello World>  Define the message for the file(s) uploaded
  -n, --number-download <250>  Define the max number of downloads for the file(s) uploaded
  -d, --duration <30>          Define the number of days the file(s) will be available for download
  -o, --output <output>        Define an output directory for the downloaded files
  -i, --insecure               Use insecure tls connection
  -c, --ca-root                Show the bundled ca root
  -s, --system-ca-bundle       If not set, the bundled cacert.pem from Mozilla (Tue Nov 26 13:58:25 2024 GMT) will be used.
  -v, --verbose                Enable verbose mode
  -w, --curl-verbose           Enable curl verbose mode
  -h, --help                   Print help
  -V, --version                Print version
```

### Examples

Upload a file :rocket::
```sh
 swish /tmo/super-file.pdf
```
Upload a file with a password :closed_lock_with_key:
```sh
swish -p mypassword /tmo/super-file.pdf
```
Upload a encrypted file in a 7z archive :lock::
```sh
swish --aes-password mypassword /tmo/super-file.pdf
```
Download a file :arrow_down::
```sh
swish https://www.swisstransfer.com/d/188be047-5b8c-48bf-9c4a-e70076c0e53c
```
Download a file with a password :closed_lock_with_key::
```sh
swish -p mypassword https://www.swisstransfer.com/d/188be047-5b8c-48bf-9c4a-e70076c0e53c
```

This tool does not provide any encryption feature. :warning:

I strongly recommend you to encrypt your file before uploading it to any cloud service :lock:

## Known Issues
***File Upload Limit***

When uploading many times the ***same*** file, it seems that infomaniaks servers flags you as suspicous traffic and you won't be able to upload for a while. It seems to be linked to your IP address, no extensive testing has been done. Use the debug mode to see the error message. :warning:

***SSL error***

If the local cURL library installation does not have the necessary certificates, the tool will not be able to connect to the server. You can use the insecure flag to bypass the SSL verification. :warning:

## Contributing

If a developer who truly has the skills and doesn't face the same skill issues as me wants to contribute to this project, feel free to do so. PRs and stuff. :handshake:

## License

This project is licensed under the GPL-3.0 License - see the [LICENSE](LICENSE) file for details. :page_facing_up:

## Contact

You can contact [Blutsh](https://github.com/Blutsh) at compote.interroge.0i@icloud.com :email:

## Donate

[Blutsh](https://github.com/Blutsh) drank some during the project, it would be nice if you could help him &nbsp;&nbsp;[![Buy a coffee for Blutsh](https://img.shields.io/badge/Ko--fi-FF5E5B?logo=ko-fi&logoColor=white)](https://ko-fi.com/Y8Y7ZK4DS)



