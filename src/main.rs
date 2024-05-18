use std::mem;
use std::os::fd::AsRawFd;

use libc::{sendto, sockaddr, sockaddr_ll, socket, AF_PACKET, SOCK_RAW};
const EXAMPLE: [u8; 44] = [
    0x00, 0x04, 0x00, 0x01, 0x00, 0x06, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x08, 0x06,
    0x00, 0x01, 0x08, 0x00, 0x06, 0x04, 0x00, 0x01, 0x00, 0xd8, 0x61, 0x57, 0x60, 0x36, 0xc0, 0xa8,
    0x02, 0x08, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xc0, 0xa8, 0x02, 0x01,
];
fn main() {
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
    if ret > 0 {
        println!("No Errors")
    }
}
