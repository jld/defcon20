fn main() {
    let in = io::stdin();
    let out = io::stdout();

    loop {
        let b : int = in.read_byte();
        if b == -1 { break }
        alt b as u8 {
          16 { out.write_str("\x1b[2J\x1b[H"); }
          13 { out.write_str("\r\n"); }
          2 { 
            let xy = in.read_bytes(2);
            if xy.len() != 2 { break }
            let x = (xy[0] as uint) + 1;
            let y = (xy[1] as uint) + 1;
            out.write_str(#fmt("\x1b[%u;%uH", y, x));
          }
          b { out.write(~[b]) }
        }
    }
}
