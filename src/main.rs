pub struct PacketBuffer {
    buf: [u8; 512],
    pos: usize
}

impl PacketBuffer {
    pub fn new() -> PacketBuffer {
        PacketBuffer {
            buf: [0; 512],
            pos: 0
        }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn step(&mut self, steps: usize) -> Result<(), ()> {
        self.pos += steps;

        Ok(())
    }

    pub fn seek(&mut self, pos: usize) -> Result<(), ()> {
        self.pos = pos;

        Ok(())
    }

    pub fn read(&mut self) -> Result<u8, &str> {
        if self.pos >= 512 {
            return Err("End of Buffer");
        }

        let result = self.buf[self.pos];
        self.pos += 1;

        Ok(result)
    }

    pub fn get(&self, pos: usize) -> Result<u8, &str> {
        if pos >= 512 {
            return Err("Position out of bound");
        }

        Ok(self.buf[self.pos])
    }

    pub fn get_range(&self, start: usize, len: usize) -> Result<&[u8], &str> {
        if start + len >= 512 {
            return Err("Position out of bound");
        }

        Ok(&self.buf[start..start + len as usize])
    }

    pub fn read_u16(&mut self) -> Result<u16, &str> {
        // FIXME: We are calling read() twice which uses &mut self
        let ms_byte = self.read()? as u16;
        let ls_byte = self.read()? as u16;
        let result = (ms_byte << 8) | ls_byte;

        Ok(result)
    }
}
