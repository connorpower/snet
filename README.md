# snet4

A simple CLI tool for calculating subnet addresses.

## Examples

### Basic Subnet Info

When invoked without arguments, `snet4` lists basic information about the
number of hosts and subnets in a network.

```
$ snet4 192.168.13.160/28

class C network
Subnets:      14
Hosts/subnet: 14
```

### List Subnets

When subnets cross the octet boundary, it becomes much less obvious where each
subnet starts and ends. `snet4` can be used to list all subnets within a
network.

```
$ snet4 --list-snets 192.168.13.160/28

192.168.13.176
192.168.13.160
192.168.13.176
192.168.13.224
...
...
```

### List All

`snet4` can also list every address in a network, along with the address
classification (host, broadcast, etc.). This can be useful to
troubleshooting issues - for example, determining whether a host might have
been incorrectly configured with a subnet broadcast or network address.

```
$ snet4 --list-all --binary --decimal 192.168.13.160/28

11000000101010000000110110100000 - 192.168.13.160 (class C network)
11000000101010000000110110110000 - 192.168.13.176 (subnet)
11000000101010000000110110110001 - 192.168.13.177 (host)
11000000101010000000110110110010 - 192.168.13.178 (host)
11000000101010000000110110110011 - 192.168.13.179 (host)
11000000101010000000110110110100 - 192.168.13.180 (host)
11000000101010000000110110110101 - 192.168.13.181 (host)
11000000101010000000110110110110 - 192.168.13.182 (host)
11000000101010000000110110110111 - 192.168.13.183 (host)
11000000101010000000110110111000 - 192.168.13.184 (host)
11000000101010000000110110111001 - 192.168.13.185 (host)
11000000101010000000110110111010 - 192.168.13.186 (host)
11000000101010000000110110111011 - 192.168.13.187 (host)
11000000101010000000110110111100 - 192.168.13.188 (host)
11000000101010000000110110111101 - 192.168.13.189 (host)
11000000101010000000110110111110 - 192.168.13.190 (host)
11000000101010000000110110111111 - 192.168.13.191 (subnet broadcast)
11000000101010000000110111000000 - 192.168.13.192 (subnet)
11000000101010000000110111000001 - 192.168.13.193 (host)
11000000101010000000110111000010 - 192.168.13.194 (host)
...
...
```

## Help

```
$ snet4 --help

snet4 provides subnet information about an IPv4 network, and optionally
lists all hosts, subnet addresses and broadcast addresses.

In many cases, IPv4 network information is fairly obvious when presented
with dotted decimal notation, but subnets which cross the octet boundary
are much less obvious. This is where snet4 can help.

When invoked without arguments, snet4 will determine the class of the
network, number of subnets and hosts per subnet.

USAGE:
    snet4 [FLAGS] <network>

FLAGS:
    -b, --binary        Format output addresses in binary
    -d, --decimal       Format output addresses in dotted decimal
    -h, --help          Prints help information
    -a, --list-all      Lists all network address, network broadcast
                        addresses, subnet addresses, subnet broadcast
                        addresses and host addresses within each
                        subnet.
    -s, --list-snets    List all base subnet network addresses
    -V, --version       Prints version information

ARGS:
    <network>    Network address in CIDR notation (e.g. 192.168.13.160/28)
```

### Why Did I Build This?

While working through the fairly excellent
Cisco [Routing TCP/IP](https://www.ciscopress.com/store/routing-tcp-ip-volume-1-9781587052026)
text, there are several exercises which involve troubleshooting subnets which
cross an octet boundary (precisely because of lack of "eyeball-a-bility"
inherent to such networks). The text's intention is that would-be CCIE's become
familiar with this process by hand, on paper, and in binary through rote
application. This small tool was born to test application of that same process,
but in program form.
