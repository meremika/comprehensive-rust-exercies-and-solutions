use std::io::Read;

pub struct RotDecoder<R> {
    pub input: R,
    pub rot: u8,
}

// Implement the `Read` trait for `RotDecoder`.
impl<R> Read for RotDecoder<R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.input.read(buf)?;
        for c in &mut buf[..n] {
            if c.is_ascii_alphabetic() {
                let offset = if c.is_ascii_uppercase() { 'A' } else { 'a' } as u8;
                *c = offset + (*c - offset + self.rot) % 26;
            }
        }
        Ok(n)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn joke() {
        let mut rot = RotDecoder {
            input: "Gb trg gb gur bgure fvqr!".as_bytes(),
            rot: 13,
        };
        let mut result = String::new();
        rot.read_to_string(&mut result).unwrap();
        assert_eq!(&result, "To get to the other side!");
    }

    #[test]
    fn binary() {
        let input: Vec<u8> = (0..=255u8).collect();
        let mut rot = RotDecoder::<&[u8]> {
            input: input.as_slice(),
            rot: 13,
        };
        let mut buf = [0u8; 256];
        assert_eq!(rot.read(&mut buf).unwrap(), 256);
        for i in 0..=255 {
            if input[i] != buf[i] {
                assert!(input[i].is_ascii_alphabetic());
                assert!(buf[i].is_ascii_alphabetic());
            }
        }
    }
}
