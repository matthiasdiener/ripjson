# ripjson
A fast and lean way to grep in JSON files.

## Installation

``` console
$ cargo install ripjson
```

## Usage

    Usage: rj <regex> <files> [options]

    Options:
        -i, --ignore-case   Search case insensitively.
        -s, --sensitive-case
                            Search case sensitively [default].
        -h, --help          Print this help menu.
        -v, --version       Print version.
            --color <WHEN>  Color output.
                            WHEN can be never, always, or auto [default].

Prints all JSON keys and values in `<files>` that match `<regex>`.

`<regex>` specifies for which JSON keys to search for. Separate path elements
with a `/`, e.g. `user/name`, similar to the JSON pointer syntax specfied in
https://tools.ietf.org/html/rfc6901.

## Example

``` console
$ cat test.json
{
    "name": "John Doe",
    "age": 43,
    "address": {
        "street": "10 Downing Street",
        "city": "London"
    },
    "phones": [
        "+44 1234567",
        "+44 2345678"
    ]
}

$ rj '.*es.*' test.json
address/street = "10 Downing Street"
address/city = "London"
phones = "+44 1234567"
phones = "+44 2345678"

$ rj '.*es.*/cit' test.json
address/city = "London"
```
