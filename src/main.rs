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
    if self.pos >= 512 {
      return Err("Position out of bounds");
    }
    
    let mut result: u16 = 0;
    
    result = result | (self.buf[self.pos] as u16) << 8;
    self.pos += 1;
    
    result = result | self.buf[self.pos] as u16;
    self.pos += 1;

    Ok(result)
  }
  
  pub fn read_u32(&mut self) -> Result<u32, &str> {
    if self.pos >= 512 {
      return Err("Position out of bounds");
    }
    
    let mut result: u32 = 0;
    
    result = result | (self.buf[self.pos] as u32) << 24;
    self.pos += 1;
    
    result = result | (self.buf[self.pos] as u32) << 16;
    self.pos += 1;
    
    result = result | (self.buf[self.pos] as u32) << 8;
    self.pos += 1;
    
    result = result | self.buf[self.pos] as u32;
    self.pos += 1;
    
    Ok(result)
  }
}

pub fn main() {
  println!("hello");
}
