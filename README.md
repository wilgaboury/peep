# peeps

This is a dead simple library/infra for creating small short-term fully connected p2p networks.

## IPV6

One of the most severe limitations it that this lib does not support ipv4 traffic. The main reason being that I don't want to be bothered with all the complexities of NAT traversal

## How the Protocol Works

### Bootstrap Step

There is one central bootstrapping server which is used for peer discovery.

1. One client creates a session, and recieves a session id
