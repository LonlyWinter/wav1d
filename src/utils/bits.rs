
use crate::utils::math::floor_log2;

pub struct BitsReader<'a> {
    data: &'a [u8],
    index: usize,
    index_max: usize,
}

impl<'a> From<&'a [u8]> for BitsReader<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self { data: value, index: 0, index_max: value.len() * 8 }
    }
}

impl<'a> BitsReader<'a> {
    pub fn get_bit_bool(data: u8, index: usize) -> bool {
        (data & (1 << (7 - index))) != 0
    }

    fn read_check(&mut self, count: usize) -> usize {
        let index = self.index;
        self.index += count;
        if self.index > self.index_max {
            panic!("data len not enought: {} {}", self.index, self.index_max);
        }
        index
    }

    pub fn get_bit(&self, index: usize) -> bool {
        let n = index >> 3;
        let index = index & 0b111;
        Self::get_bit_bool(self.data[n], index)
    }

    pub fn get_1byte(&self, start: usize, count: usize) -> u8 {
        assert!(count <= 8, "count too long");
        // 计算起始和结束的字节索引和比特偏移
        let start_byte = start >> 3;
        let start_bit = start & 0b111; // 在字节内的比特偏移（0 是最高位？还是最低位？）
        let end = start + count - 1;
        let end_byte = end >> 3;
        let end_bit = end & 0b111;

        if start_byte == end_byte {
            // 同一字节内
            let byte = self.data[start_byte];
            // 构造掩码：从 start_bit 到 end_bit（共 count 位）
            // 掩码：从 bit0 开始数，要取 [start_bit, end_bit] 这些位
            let mask = ((1u8 << count) - 1) << (7 - end_bit); // 对齐到高位
            (byte & mask) >> (7 - end_bit)
        } else {
            // 跨字节：最多跨两个字节（因为 e-s+1 < 8）
            // 第一个字节：从 start_bit 到字节末尾（bit7）
            let first_byte = self.data[start_byte];
            let bits_from_first = 8 - start_bit; // 从第一个字节取多少位
            let mask_first = (1u8 << bits_from_first) - 1; // 低 bits_from_first 位置 1
            let mut result = (first_byte & mask_first) << (count - bits_from_first);

            // 第二个字节：取高位部分（从 bit0 到 bit_{remaining-1}）
            let second_byte = self.data[end_byte];
            let bits_from_second = count - bits_from_first;
            let mask_second = (1u8 << bits_from_second) - 1; // 低 bits_from_second 位置 1
            result |= (second_byte >> (8 - bits_from_second)) & mask_second;
            result
        }
    }

    pub fn get_nbyte(&self, start: usize, count: usize, res: &mut [u8]) {
        assert!(res.len() * 8 >= count, "get bits res length not enouth");
        let mut src_byte_idx = start >> 3;           // 当前读取的源字节索引
        let mut src_bit_offset = 7 - (start & 0b111);   // 源字节内偏移（0 = MSB）
        let mut bits_left = count;         // 还需要复制的比特数
        let mut dst_byte_idx = 0;               // 当前写入的目标字节
        let mut pending: u16 = 0;               // 待写入的比特（最多 16 位）
        let mut pending_bits = 0;               // pending 中的比特数

        while bits_left > 0 && src_byte_idx < self.data.len() {
            let src_byte = self.data[src_byte_idx];
            let available_bits = src_bit_offset + 1; // 可用比特数（从当前bit到字节末尾）
            let take_bits = available_bits.min(bits_left);

            // 提取当前字节中的有效比特
            let shift = src_bit_offset + 1 - take_bits;
            let mask = if take_bits == 8 {
                0xFF
            } else {
                ((1u16 << take_bits) - 1) << shift
            } as u8;
            let bits = (src_byte & mask) >> shift;

            // 将提取的比特加入 pending（左对齐到高位）
            pending |= (bits as u16) << (16 - pending_bits - take_bits);
            pending_bits += take_bits;

            bits_left -= take_bits;

            // 更新 src_bit_offset：如果当前字节已读完
            if take_bits > src_bit_offset {
                src_byte_idx += 1;
                src_bit_offset = 7; // 下一个字节从 MSB 开始
            } else {
                src_bit_offset -= take_bits;
            }


            // 将 pending 中的完整字节写入 b
            while pending_bits >= 8 {
                res[dst_byte_idx] = (pending >> 8) as u8; // 取出最高字节
                dst_byte_idx += 1;
                pending <<= 8;
                pending_bits -= 8;
            }
        }

        // 写入最后不足一个字节的部分
        if pending_bits > 0 {
            res[dst_byte_idx] |= (pending >> (16 - pending_bits)) as u8;
        }
    }

    pub fn read_bit(&mut self) -> bool {
        let index = self.read_check(1);
        self.get_bit(index)
    }

    pub fn read_nbyte(&mut self, count: usize, res: &mut [u8]) {
        let index = self.read_check(1);
        Self::get_nbyte(self, index, count, res);
    }

    pub fn read_skip(&mut self, count: u8) {
        self.index += count as usize;
    }

    // pub fn read_u128(&mut self, count: usize) -> u128 {
    //     let mut res = [0u8; 16];
    //     self.read_nbyte(count, &mut res);
    //     u128::from_le_bytes(res)
    // }

    // pub fn read_u64(&mut self, count: u8) -> u64 {
    //     let mut res = [0u8; 8];
    //     self.read_nbyte(count as usize, &mut res);
    //     u64::from_le_bytes(res)
    // }

    pub fn read_u32(&mut self, count: u8) -> u32 {
        let mut res = [0u8; 4];
        self.read_nbyte(count as usize, &mut res);
        u32::from_le_bytes(res)
    }

    pub fn read_u16(&mut self, count: u8) -> u16 {
        let mut res = [0u8; 2];
        self.read_nbyte(count as usize, &mut res);
        u16::from_le_bytes(res)
    }

    pub fn read_u8(&mut self, count: u8) -> u8 {
        let count = count as usize;
        let index = self.read_check(count);
        Self::get_1byte(self, index, count)
    }

    pub fn read_uvlc(&mut self) -> u32 {
        let mut leading_zeros = 0;
        for _ in 0..32 {
            if self.read_bit() {
                break;
            }
            leading_zeros += 1;
        }
        if leading_zeros >= 32 {
            return u32::MAX
        }
        let mut res = [0u8; 4];
        self.read_nbyte(leading_zeros, &mut res);
        u32::from_le_bytes(res) + ( 1 << leading_zeros ) - 1
    }

    pub fn read_leb128(&mut self) -> usize {
        let mut res = 0;
        let mut shift = 0;
        const CONTINUATION_BIT: u8 = 1 << 7;

        loop {
            let v = self.read_u8(8);
            let low_bits = (v & (!CONTINUATION_BIT)) as u64;
            res |= low_bits << shift;

            if !Self::get_bit_bool(v, 7) {
                break;
            }

            shift += 7;
        }
        res as usize
    }

    pub fn read_su(&mut self, count: u8) -> i32 {
        let mut value = self.read_u32(count) as i32;
        let sign_mask = 1 << (count - 1);
        if value & sign_mask > 0 {
            value -= 2 * sign_mask
        }
        value
    }

    pub fn read_ns(&mut self, n: u32) -> u32 {
        let w = floor_log2(n);
        let m = (1 << w) - n;
        let v = self.read_u32(m as u8);
        if v < m {
            return v;
        }
        let extra_bit = self.read_u32(1);
        (v << 1) - m + extra_bit
    }

    pub fn read_alignment(&mut self) {
        let n = self.index & 0b111;
        if n > 0 {
            self.read_skip(8 - n as u8);
        }
    }

    pub fn read_position(&self) -> usize {
        self.index
    }
}


pub trait FromBitsReader {
    fn from_bits_reader(reader: &mut BitsReader) -> Self;
}