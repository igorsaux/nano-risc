use nano_risc_arch::Limits;

use crate::{RuntimeError, RuntimeErrorKind};

#[derive(Debug, Clone, PartialEq)]
pub struct Ram {
    limits: Limits,
    data: Vec<u8>,
}

impl Ram {
    pub fn new(limits: Limits) -> Self {
        Self {
            limits,
            data: Vec::new(),
        }
    }

    pub fn write_slice(&mut self, offset: usize, src: &[u8]) -> Result<(), RuntimeError> {
        if src.len() > self.limits.ram_length {
            return Err(RuntimeError::new(
                format!("Can't fit a memory block with size {} into RAM", src.len()),
                RuntimeErrorKind::OutOfMemory,
            ));
        }

        let needed = offset + src.len();

        if offset >= self.limits.ram_length || needed > self.limits.ram_length {
            return Err(RuntimeError::new(
                format!("Offset {} with size {} is out of bounds", offset, src.len()),
                RuntimeErrorKind::InvalidAddress { address: offset },
            ));
        }

        self.grow(needed - self.data.capacity());

        if offset == 0 {
            self.data.copy_from_slice(src);
        } else {
            self.data.split_at_mut(offset).1.copy_from_slice(src);
        }

        Ok(())
    }

    pub fn write(&mut self, offset: usize, src: u8) -> Result<(), RuntimeError> {
        if offset >= self.limits.ram_length {
            return Err(RuntimeError::new(
                format!("Address {} is out of bounds", offset),
                RuntimeErrorKind::InvalidAddress { address: offset },
            ));
        }

        if offset >= self.data.len() {
            self.grow(offset + 1);
        }

        self.data[offset] = src;

        Ok(())
    }

    pub fn read(&self, offset: usize) -> Result<u8, RuntimeError> {
        if offset >= self.limits.ram_length {
            return Err(RuntimeError::new(
                format!("Address {} is out of bounds", offset),
                RuntimeErrorKind::InvalidAddress { address: offset },
            ));
        }

        if offset >= self.data.len() {
            return Ok(0);
        }

        Ok(self.data[offset])
    }

    fn grow(&mut self, length: usize) {
        self.data.reserve_exact(length);

        for i in self.data.spare_capacity_mut() {
            i.write(0);
        }

        unsafe { self.data.set_len(self.data.capacity()) }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.data.as_slice()
    }
}

#[cfg(test)]
mod tests {
    use nano_risc_arch::Limits;

    use crate::{Ram, RuntimeErrorKind};

    #[test]
    fn write_slice() {
        let mut ram = Ram::new(Limits {
            ram_length: 10,
            ..Default::default()
        });
        let data = [5, 5, 5, 5, 5];

        ram.write_slice(5, &data).unwrap();
        assert_eq!(ram.as_bytes().len(), 10);
        assert!(matches!(ram.as_bytes(), [0, 0, 0, 0, 0, 5, 5, 5, 5, 5]))
    }

    #[test]
    fn write_slice_fail() {
        let mut ram = Ram::new(Limits {
            ram_length: 5,
            ..Default::default()
        });
        let data = [5, 5, 5, 5, 5];

        assert_eq!(
            ram.write_slice(5, &data).map_err(|err| err.kind().clone()),
            Err(RuntimeErrorKind::InvalidAddress { address: 5 })
        );

        let data = [5, 5, 5, 5, 5, 5, 5, 5, 5];

        assert_eq!(
            ram.write_slice(0, &data).map_err(|err| err.kind().clone()),
            Err(RuntimeErrorKind::OutOfMemory)
        );
    }

    #[test]
    fn write() {
        let mut ram = Ram::new(Limits {
            ram_length: 5,
            ..Default::default()
        });

        ram.write(3, 5).unwrap();
        assert_eq!(ram.as_bytes().len(), 4)
    }

    #[test]
    fn write_fail() {
        let mut ram = Ram::new(Limits {
            ram_length: 5,
            ..Default::default()
        });

        assert_eq!(
            ram.write(5, 5).map_err(|err| err.kind().clone()),
            Err(RuntimeErrorKind::InvalidAddress { address: 5 })
        );
    }

    #[test]
    fn read() {
        let mut ram = Ram::new(Limits {
            ram_length: 5,
            ..Default::default()
        });

        ram.write(3, 12).unwrap();

        assert_eq!(ram.read(3), Ok(12));
    }

    #[test]
    fn read_zero() {
        let ram = Ram::new(Limits {
            ram_length: 5,
            ..Default::default()
        });

        assert_eq!(ram.read(4), Ok(0))
    }

    #[test]
    fn read_fail() {
        let ram = Ram::new(Limits {
            ram_length: 5,
            ..Default::default()
        });

        assert_eq!(
            ram.read(7).map_err(|err| err.kind().clone()),
            Err(RuntimeErrorKind::InvalidAddress { address: 7 })
        )
    }
}
