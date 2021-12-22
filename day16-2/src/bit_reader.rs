pub struct BitReader<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> BitReader<'a> {
    pub fn new(bytes: &'a [u8]) -> BitReader<'a> {
        BitReader { bytes, pos: 0 }
    }

    pub fn has_next(&self) -> bool {
        self.pos < self.bytes.len()
    }

    pub fn read_bit(&mut self) -> u8 {
        let pos = self.pos;
        self.pos += 1;
        self.bytes[pos]
    }

    pub fn read_bits(&mut self, count: usize) -> &'a [u8] {
        let pos = self.pos..(self.pos + count);
        self.pos += count;
        &self.bytes[pos]
    }

    pub fn read_bits_u8(&mut self, count: usize) -> u8 {
        if count > 8 {
            panic!("cannot read more than 8 bits in a byte");
        }
        let read = self.read_bits(count);
        (0..count).fold(0, |mut accum, curr| {
            let b = read[curr] << (read.len() - curr - 1);
            accum = accum | b;
            accum
        })
    }

    pub fn read_bits_u16(&mut self, count: usize) -> u16 {
        if count > 16 {
            panic!("cannot read more than 16 bits in a word");
        }
        let read = self.read_bits(count);
        (0..count).fold(0, |mut accum, curr| {
            let b = (read[curr] as u16) << (read.len() - curr - 1);
            accum = accum | b;
            accum
        })
    }
}
