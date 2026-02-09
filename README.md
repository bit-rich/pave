# PAVE

**Peer-Anonymized VPN Entry**

A protocol that adds a trustless anonymity layer to VPN services by routing traffic through peers before it reaches the VPN server.

## The Problem

When you use a VPN, you're trusting the provider with your IP address, and you're trusting they keep no logs on your activity.

## The Solution

PAVE splits the trust:

```
You → Random Peer → VPN Server → Internet
```

- **The VPN** sees your traffic, but only knows the peer's IP, not yours
- **The peer** sees your IP, but only sees encrypted blobs — not your traffic
- **No single party** can link your identity to your activity

## How It Can Work

1. **Peer Discovery**: All users publish public keys to server or DHT
2. **Anonymous Matching**: Peers use the server or DHT to publish asymmetrically encrypted messages to each other to negotiate connections
3. **Hole Punching**: You and your peer exchange IPs (encrypted), then establish a direct UDP connection through NAT
4. **Relay**: Your VPN traffic flows through the peer to the VPN server

## Key Features

- **No coordinator sees pairings**: The system assigns relays, but IPs are exchanged via end-to-end encrypted messages
- **Credit system**: The goal is to make every participant a relay thereby incentivizing sufficient bandwidth for each user. Earn credits by relaying traffic, and use them to send your own traffic.
- **Single hop**: Unlike Tor (3 hops), PAVE adds only one hop — minimal latency impact
- **Works with existing VPNs**: PAVE is an entry layer, not a replacement

## Threat Model

**PAVE protects against:**
- VPN provider logging your real IP
- VPN provider being compromised or subpoenaed
- Single point of trust failure

**PAVE does NOT protect against:**
- Global traffic analysis (like Tor, timing attacks are possible)
- Malicious peers colluding with VPN (still better than trusting VPN alone)
- Your own OPSEC failures

## Status

🚧 **Early Development** 🚧

This is a proof-of-concept implementation. Not ready for production use.

## Roadmap

- [ ] Basic UDP hole punching between two peers
- [ ] Peer-to-peer connections through hardcoded IP list
- [ ] Peers relaying traffic
- [ ] WireGuard integration
- [ ] Backup relays & reconnection mechanics
- [ ] Encrypted broadcast messaging through DHT for peer discovery
- [ ] Credit/reputation system

## Tech Stack

- **Language**: Rust
- **Networking**: tokio, UDP
- **Crypto**: TBD (likely x25519-dalek or ring)

## License

Apache License 2.0

## Contributing

This is a learning project, but contributions and feedback are welcome! Open an issue to discuss ideas.

---

PAVE is not affiliated with any VPN provider at this time. Use at your own risk.*
