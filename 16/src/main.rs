use std::cmp::{max, min};
use std::fs;

extern crate hex;

struct FileReader<'a> {
    data: &'a [u8],
    curr_bit: usize,
    read_bytes: usize,
}

fn read_bits(curr_value: u8, curr_bit: usize, num_bits: usize) -> u32 {
    let mask = ((1u32 << num_bits) - 1) << (8 - curr_bit - num_bits);
    ((curr_value as u32 & mask) >> (8 - curr_bit - num_bits))
}

impl<'a> FileReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, curr_bit: 0, read_bytes: 0 }
    }

    fn next_char(&mut self) {
        self.data = &self.data[1..];
        self.curr_bit = 0;
        self.read_bytes += 1;
    }

    fn can_read(&self, num_bits: usize) -> bool {
        (self.data.len() * 8 - self.curr_bit) >= num_bits
    }

    fn bits_read(&self) -> usize {
        self.read_bytes * 8 + self.curr_bit
    }

    fn read(&mut self, mut num_bits: usize) -> u32 {
        let remaining_bits = 8 - self.curr_bit;
        // Read the remaining bits from current char
        let mut result = read_bits(self.data[0], self.curr_bit, min(num_bits, remaining_bits));

        if num_bits >= remaining_bits {
            num_bits -= remaining_bits;

            self.next_char();
            if num_bits > 0 {
                return (result << num_bits) | self.read(num_bits);
            }
        } else {
            self.curr_bit += num_bits;
        }

        result
    }
}


fn read_packets(reader: &mut FileReader, version_sum: &mut u32, stop_at_n_packets: Option<usize>, stop_after_n_bits: Option<usize>) {
    let start_bits_read = reader.bits_read();
    let mut packets_read = 0;

    while reader.can_read(6 + 5) {
        match stop_after_n_bits {
            Some(bit_count) => if reader.bits_read() >= bit_count + start_bits_read { break; }
            _ => ()
        }
        match stop_at_n_packets {
            Some(packet_count) => if packets_read >= packet_count { break; }
            _ => ()
        }

        let packet_version = reader.read(3);
        let packet_type = reader.read(3);
        *version_sum += packet_version;
        packets_read += 1;

        if packet_type == 4 { // Literal value
            let mut value = 0;
            loop {
                let flag = reader.read(1);
                value |= reader.read(4);
                if flag == 1 {
                    value <<= 4;
                } else {
                    break;
                }
            }
            println!("{}", value);
        } else {
            match reader.read(1) {
                1 => {
                    let packet_count = reader.read(11);
                    read_packets(reader, version_sum, Some(packet_count as usize), None)
                }
                0 => {
                    let bit_count = reader.read(15);
                    read_packets(reader, version_sum, None, Some(bit_count as usize))
                }
                _ => panic!("unexpected length type ID")
            };
        }
    }
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").unwrap();
    let data = hex::decode(file_contents).unwrap();
    let mut reader = FileReader::new(data.as_slice());

    let mut version_sum = 0;
    read_packets(&mut reader, &mut version_sum, None, None);

    println!("Version sum: {}", version_sum);
}
