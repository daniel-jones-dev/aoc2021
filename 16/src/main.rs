use std::cmp::min;
use std::fs;

extern crate hex;

struct FileReader<'a> {
    data: &'a [u8],
    curr_bit: usize,
    read_bytes: usize,
}

fn read_bits(curr_value: u8, curr_bit: usize, num_bits: usize) -> u64 {
    let mask = ((1u64 << num_bits) - 1) << (8 - curr_bit - num_bits);
    (curr_value as u64 & mask) >> (8 - curr_bit - num_bits)
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

    fn read(&mut self, mut num_bits: usize) -> u64 {
        let remaining_bits = 8 - self.curr_bit;
        // Read the remaining bits from current char
        let result = read_bits(self.data[0], self.curr_bit, min(num_bits, remaining_bits));

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

fn read_n_packets(reader: &mut FileReader, version_sum: &mut u64, num_packets_to_read: usize) -> Vec<u64> {
    let mut result = Vec::new();
    for _ in 0..num_packets_to_read {
        result.push(read_packet(reader, version_sum));
    }
    return result;
}

fn read_packets_from_n_bits(reader: &mut FileReader, version_sum: &mut u64, num_bits_to_read: usize) -> Vec<u64> {
    let start_bits_read = reader.bits_read();
    let mut result = Vec::new();
    while reader.bits_read() < num_bits_to_read + start_bits_read{
        result.push(read_packet(reader, version_sum));
    }
    return result;
}

fn read_packet(reader: &mut FileReader, version_sum: &mut u64) -> u64 {
    if ! reader.can_read(6 + 5) { panic!("Not enough bits remaining"); }

    let packet_version = reader.read(3);
    let packet_type = reader.read(3);
    *version_sum += packet_version;

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
        value
    } else {
        let subpackets = match reader.read(1) {
            1 => {
                let packet_count = reader.read(11) as usize;
                read_n_packets(reader, version_sum, packet_count)
            }
            0 => {
                let bit_count = reader.read(15) as usize;
                read_packets_from_n_bits(reader, version_sum, bit_count)
            }
            _ => panic!("unexpected length type ID")
        };

        return match packet_type {
            0 => subpackets.iter().fold(0, |acc, x| acc + x),
            1 => subpackets.iter().fold(1, |acc, x| acc * x),
            2 => *subpackets.iter().min().unwrap(),
            3 => *subpackets.iter().max().unwrap(),
            5 => if subpackets[0] > subpackets[1] {1} else {0},
            6 => if subpackets[0] < subpackets[1] {1} else {0},
            7 => if subpackets[0] == subpackets[1] {1} else {0},
            _ => panic!("Unknown operator type")
        }
    }
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").unwrap();
    // let file_contents = "9C0141080250320F1802104A08";
    let data = hex::decode(file_contents).unwrap();
    let mut reader = FileReader::new(data.as_slice());

    let mut version_sum = 0;
    let result = read_packet(&mut reader, &mut version_sum);

    println!("Version sum: {}", version_sum);
    println!("Result: {}", result);
}
