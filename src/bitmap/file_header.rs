use super::util;

pub struct FileHeader
{
    // Must always be set to 'BM' to declare that this is a .bmp-file
    bf_type: [char; 2],
    // specifies the size of the file in bytes
    bf_size: u32,
    bf_reserved1: u16,
    bf_reserved2: u16,
    // Specifies the offset from the beginning of the file to the bitmap data.
    bf_off_bits: u32
}

impl FileHeader
{
    /**
     * Create a new Bit Map File Header
     * 
     * @param {u32} size of file in bytes
     * @param {u32} offset from beginning of file to bitmap data
     * @return {FileHeader}
     */
    // pub fn new(size: u32, offset: u32) -> FileHeader
    // {
    //     FileHeader {
    //         bf_type: ['B', 'M'],
    //         bf_size: size,
    //         bf_reserved1: 0,
    //         bf_reserved2: 0,
    //         bf_off_bits: offset,
    //     }
    // }

    /**
     * Create a new Bit Map File Header from stream of bytes
     * 
     * @param {&[u8; 14]} 14 byte long slice
     * @return {FileHeader}
     */
    pub fn stream(bit_stream: &[u8]) -> FileHeader
    {
        let bf_type = [bit_stream[0] as char, bit_stream[1] as char];
        let mut i = 2;
        FileHeader {
            bf_type,
            bf_size: util::byte_slice_to_u32(bit_stream, &mut i),
            bf_reserved1: util::byte_slice_to_u16(bit_stream, &mut i),
            bf_reserved2: util::byte_slice_to_u16(bit_stream, &mut i),
            bf_off_bits: util::byte_slice_to_u32(bit_stream, &mut i)
        }
    }

    pub fn as_bytes(&self) -> Vec<u8>
    {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&[self.bf_type[0] as u8, self.bf_type[1] as u8]);
        bytes.extend_from_slice(&util::byte_slice_from_u32(self.bf_size));
        bytes.extend_from_slice(&util::byte_slice_from_u16(self.bf_reserved1));
        bytes.extend_from_slice(&util::byte_slice_from_u16(self.bf_reserved2));
        bytes.extend_from_slice(&util::byte_slice_from_u32(self.bf_off_bits));
        bytes
    }

    pub fn get_file_header_byte_size(&self) -> usize
    {
        14
    }

    // pub fn get_size(&self) -> u32
    // {
    //     self.bf_size
    // }

    pub fn get_off_bits(&self) -> u32
    {
        self.bf_off_bits
    }

}

impl std::fmt::Display for FileHeader
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "Type: {}{}, Size: {}, res1: {}, res2: {}, offset: {}",
            self.bf_type[0], self.bf_type[1], self.bf_size, self.bf_reserved1,
            self.bf_reserved2, self.bf_off_bits)
    }
}