use crate::app::InputChunkBuffer;

const ZEROES_CHUNK: Chunk = Chunk(['0', '0', '0', '0']);

#[derive(Debug, PartialEq, Eq)]
struct Chunk([char; 4]);

impl Chunk {
    pub fn shorten(&self) -> String {
        // 0100
        let mut short = String::new();
        let mut hidden = true;
        for c in self.0[0..3].iter() {
            if c.eq(&'0') && hidden {
                continue;
            } else {
                hidden = false;
                short.push(*c);
            }
        }
        short.push(self.0[3]);
        short
    }
}

pub fn shorten_ipv6(ipv6: &[InputChunkBuffer; 8]) -> String {
    let mut chunks = [ZEROES_CHUNK; 8];
    for (i, buf) in ipv6.iter().enumerate() {
        if buf.len() != 0 {
            chunks[i] = buf.into()
        }
    }

    let mut max = (0_usize, 0_usize);
    let mut current = (0_usize, 0_usize);
    let mut is_group_beginning = true;
    let mut chunk_iter = chunks.iter().enumerate().peekable();
    while let Some((i, chunk)) = chunk_iter.next() {
        if chunk.eq(&ZEROES_CHUNK) {
            if is_group_beginning {
                current.1 = i;
                is_group_beginning = false;
            }
            current.0 += 1;

            if let Some((_, peek)) = chunk_iter.peek() {
                if peek.ne(&&ZEROES_CHUNK) {
                    is_group_beginning = true;
                    if max.0 < current.0 {
                        max = current;
                    }
                    current = (0_usize, 0_usize);
                }
            }
        }
    }
    if max.0 < current.0 {
        max = current;
    }

    let mut output = String::new();

    if max.0 > 1 {
        let start_idx = max.1;
        let end_idx = max.0 + max.1;

        if start_idx == 0 {
            output.push_str("::");
            if end_idx < 8 {
                for (_, c) in chunks[end_idx..7].iter().enumerate() {
                    output.push_str(&c.shorten());
                    output.push(':');
                }
                output.push_str(&chunks[7].shorten());
            }
        } else {
            let mut chunk_iter = chunks.iter().enumerate().peekable();
            output.push_str(&chunk_iter.next().unwrap().1.shorten());
            output.push(':');
            while let Some((i, chunk)) = chunk_iter.next() {
                if start_idx == i {
                    for _ in start_idx..end_idx-1 {
                        chunk_iter.next();
                    }
                    output.push(':');
                    continue;
                }
                output.push_str(&chunk.shorten());
                output.push(':');
            }
            if end_idx != 8 {
                output.pop();
            }
        }
    } else {
        for (_, c) in chunks[0..7].iter().enumerate() {
            output.push_str(&c.shorten());
            output.push(':');
        }
        output.push_str(&chunks[7].shorten());
    }
    output
}

impl Into<Chunk> for &InputChunkBuffer {
    fn into(self) -> Chunk {
        let mut chars = self.buf.chars();
        Chunk([
            chars.next().unwrap(),
            chars.next().unwrap(),
            chars.next().unwrap(),
            chars.next().unwrap(),
        ])
    }
}
