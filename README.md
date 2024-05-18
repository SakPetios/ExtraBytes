# Extra Bytes Added to Packet When Sending via Raw Socket (Rust + libc)

## Description

I'm currently tring to send packets via a raw socket in rust. I'm currently using libc to manage the socket.
I've had success with sending the packet but the packet sent is never the same as the original one. Here's what's happening,
the packet I'm sending is this:
```rust 
const EXAMPLE: [u8; 44] = [
    0x00, 0x04, 0x00, 0x01, 0x00, 0x06, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00,/* Pay attention here*/ 0x08, 0x06, 
    0x00, 0x01, 0x08, 0x00, 0x06, 0x04, 0x00, 0x01, 0x00, 0xd8, 0x61, 0x57, 0x60, 0x36, 0xc0, 0xa8,
    0x02, 0x08, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xc0, 0xa8, 0x02, 0x01,
];
```
This is essentially an arp request about 192.168.2.1

The packet being sent is this (according to wireshark)
```
00 04 00 01 00 06 ff ff ff ff ff ff 00 00 11 00 // Notice the extra 0x0011 here?
08 06 00 01 08 00 06 04 00 01 00 d8 61 57 60 36 
c0 a8 02 08 ff ff ff ff ff ff c0 a8 02 01        
```
I know what is adding the exta bytes. 
Here's what I'm doing:
```rust
    let sock = unsafe { socket(AF_PACKET, SOCK_RAW, 0x3) }; // Allow all protocols
    let addr = sockaddr_ll {
        sll_family: 0x3,
        sll_protocol: 0x11,
        sll_ifindex: 2,
        sll_addr: [0; 8],
        sll_halen: 0,
        sll_hatype: 0,
        sll_pkttype: 0,
    };

    let ret = unsafe { sendto(
        sock.as_raw_fd(),
        EXAMPLE.as_ptr().cast(),
        EXAMPLE.len(),
        0,
        &addr as *const _ as *const sockaddr,
        mem::size_of::<sockaddr_ll>() as u32,
    ) };
    if ret < 0 {
        unreachable!()
    }
```


After some investigating I've come to the conclusion that the extra bytes belong to `sll_protocol`.

## Possible Cause

I'm not 100% sure but I noticed that `sockaddr_ll` is 20 bytes long while `sockaddr` is 16. This leaves
4 bytes. This is extactly the size of out extra bytes. The reason why the extra bytes appear is because I'm casting 
the pointer type from `sockaddr_ll` to `sockaddr` as a result for some reason the 4 extra bytes are inserted in the 
packet. If this is the case then what's the purpose to `mem::size_of::<sockaddr_ll>()` it should've been handled correctlu
since I provide the size of the address. In addition If my assumtion is correct how do I fix my error?



