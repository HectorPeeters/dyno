use std::io::Write;

pub fn write_elf_file_header<T>(writer: &mut T)
where
    T: Write,
{
    writer.write(&[0x7F, 0x45, 0x4C, 0x46]).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufWriter;

    #[test]
    fn test_write_elf_header() {
        let mut writer = BufWriter::new(Vec::<u8>::new());
        write_elf_file_header(&mut writer);

        assert_eq!(writer.buffer(), &[0x7F, 0x45, 0x4C, 0x46]);
    }
}
