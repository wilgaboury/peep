# peeps

Dead simple library/infra for creating small short-term fully connected p2p networks.

## IPV6

One of the most severe limitations is that this lib is ipv6 only. The main reason being that I don't want to be bothered with all the complexities of NAT traversal.

## How it works

There is a central bootstrapping server which is used for peer discovery. Clients create/update a p2p session and the server stores the list of peers, identified by their (ip, port) tuple. Clients joining the session read the list and then connect directly to the peers.