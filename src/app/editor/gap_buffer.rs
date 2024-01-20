use std::{cmp::max, fmt, str};

pub struct GapBuffer {
    pub buffer: Vec<char>,
    pub gap_start: usize,
    pub gap_length: usize,
}

impl GapBuffer {
    pub fn with_data(data: &str) -> Self {
        let mut buffer: Vec<char> = data.chars().collect();
        let target_capacity = max(
            if buffer.len().is_power_of_two() {
                (buffer.len() + 1).next_power_of_two()
            } else {
                buffer.len().next_power_of_two()
            },
            16,
        );

        let gap_size = target_capacity - buffer.len();
        let gap_start = buffer.len();

        buffer.append(&mut vec!['_'; gap_size]);

        let gap_length = buffer.len() - data.len();

        let mut gb = Self {
            buffer,
            gap_start,
            gap_length,
        };
        gb.move_gap(0);
        gb
    }

    pub fn get_text_as_chars(&self) -> Vec<char> {
        let left = &self.buffer[..self.gap_start];
        let right = &self.buffer[self.tail_start()..];
        [left, right].concat()
    }

    pub fn get_text_as_bytes(&self) -> Vec<u8> {
        let left = &self.buffer[..self.gap_start];
        let right = &self.buffer[self.tail_start()..];
        [left, right]
            .concat()
            .into_iter()
            .collect::<String>()
            .into_bytes()
    }

    pub fn get_text_as_string(&self) -> String {
        let left = &self.buffer[..self.gap_start];
        let right = &self.buffer[self.tail_start()..];
        [left, right].concat().into_iter().collect::<String>()
    }

    pub fn insert_at(&mut self, data: &str, at: usize) {
        self.move_gap(at);
        self.insert(data)
    }

    pub fn delete_at(&mut self, num_to_delete: usize, at: usize) {
        // If delete would mean tail is out of bounds we need to panic
        let new_tail_end = self.gap_length + num_to_delete + at;
        assert!(new_tail_end <= self.buffer.len());

        self.move_gap(at);
        self.gap_length += num_to_delete;
    }

    pub fn get_at(&self, at: usize) -> char {
        if at < self.gap_start {
            self.buffer[at]
        } else {
            self.buffer[self.tail_start() + at - self.gap_start]
        }
    }

    fn move_gap(&mut self, pos: usize) {
        // Assert that we are only moving the gap within the data
        // of our buffer excluding the gap.
        assert!(pos <= self.data_length());

        let position_delta: isize = pos as isize - self.gap_start as isize;
        match position_delta {
            // Moving the "cursor" (gap) right
            num if num > 0 => {
                let chunk_to_bump =
                    self.tail_start()..(self.tail_start() + position_delta.unsigned_abs());

                self.buffer.copy_within(chunk_to_bump, self.gap_start);

                self.gap_start = pos;
            }
            // Moving the "cursor" (gap) left
            num if num < 0 => {
                let chunk_to_bump = pos..self.gap_start;
                let new_tail_start = self.tail_start() - position_delta.unsigned_abs();
                self.buffer.copy_within(chunk_to_bump, new_tail_start);
                self.gap_start = pos;
            }
            // Moving the "cursor" to where it already is (noop)
            _ => {}
        }
    }

    fn insert(&mut self, data: &str) {
        let slice_to_insert: Vec<char> = data.chars().collect();

        if slice_to_insert.len() > self.gap_length {
            self.grow(slice_to_insert.len() + 1024);
        }

        let body_slice: &mut [char] =
            &mut self.buffer[self.gap_start..(self.gap_start + slice_to_insert.len())];
        body_slice.copy_from_slice(&slice_to_insert);
        self.gap_start += slice_to_insert.len();
        self.gap_length -= slice_to_insert.len();
    }

    fn grow(&mut self, amount: usize) {
        let tail_range = self.tail_start()..self.buffer.len();
        let tail_size = tail_range.len();
        self.buffer.resize(self.buffer.len() + amount, '_');
        let new_tail_start = self.buffer.len() - tail_size;
        self.buffer.copy_within(tail_range, new_tail_start);
        self.gap_length += amount;
    }

    fn tail_start(&self) -> usize {
        self.gap_start + self.gap_length
    }

    pub fn data_length(&self) -> usize {
        self.buffer.len() - self.gap_length
    }
}

impl fmt::Debug for GapBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "    ")?;
        writeln!(f, "    Gap Start:   {}", self.gap_start)?;
        writeln!(f, "    Gap Length:  {}", self.gap_length)?;
        writeln!(f, "    Tail Start:  {}", self.tail_start())?;
        writeln!(f, "    Data Length:  {}", self.data_length())?;
        writeln!(f, "    Buffer Length:  {}", self.buffer.len())?;
        let display_buf: Vec<char> = self
            .buffer
            .clone()
            .into_iter()
            .enumerate()
            .filter_map(|(idx, el)| {
                if idx < self.gap_start || idx > self.tail_start() - 1 {
                    return Some(el);
                } else if idx == self.gap_start {
                    return Some('[');
                } else if idx == self.tail_start() - 1 {
                    return Some(']');
                } else if idx == self.tail_start() - 2 || idx == self.gap_start + 1 {
                    return Some('~');
                }
                None
            })
            .collect();
        writeln!(
            f,
            "    Data:        {}",
            display_buf.iter().collect::<String>()
        )
    }
}

#[cfg(test)]
mod miv_gap_buffer_tests {
    use super::*;

    // More or less a smoke test to make sure we can create
    #[test]
    fn creation() {
        let gb = GapBuffer::with_data("Hello World");
        assert_eq!(&gb.get_text_as_string(), "Hello World");
        assert_eq!(gb.gap_length, 5);
        assert_eq!(gb.gap_start, 0);
    }

    #[test]
    fn creation_multiline() {
        let multiline = r"1
2
3";
        let gb = GapBuffer::with_data(multiline);
        assert_eq!(&gb.get_text_as_string(), "1\n2\n3");
        assert_eq!(gb.gap_length, 11);
        assert_eq!(gb.gap_start, 0);
    }

    // Check happy path movement logic
    // I.e. moving to a valid position
    #[test]
    fn move_gap_within_bounds() {
        let mut gb = GapBuffer::with_data("Hello World");
        assert_eq!(&gb.get_text_as_string(), "Hello World");
        assert_eq!(gb.gap_length, 5);
        assert_eq!(gb.gap_start, 0);
        dbg!(&gb.tail_start());
        gb.move_gap(11);
        assert_eq!(&gb.get_text_as_string(), "Hello World");
        assert_eq!(gb.gap_length, 5);
        assert_eq!(gb.gap_start, 11);
        gb.move_gap(5);
        assert_eq!(&gb.get_text_as_string(), "Hello World");
        assert_eq!(gb.gap_length, 5);
        assert_eq!(gb.gap_start, 5);
    }

    // Check unhappy path movement logic
    // I.e. moving to a position outside the data
    #[test]
    #[should_panic]
    fn move_gap_out_of_bounds() {
        let mut gb = GapBuffer::with_data("Hello World");
        gb.move_gap(12);
    }

    // Check insertion
    #[test]
    fn insert_multi_character_string_smaller_than_gap_at_end() {
        let mut gb = GapBuffer::with_data("Hello World");
        let original_buffer_length = gb.buffer.len();
        gb.move_gap(gb.data_length());
        gb.insert("!!!");
        assert_eq!(&gb.get_text_as_string(), "Hello World!!!");
        // Assert gap changed as expected, length of addition was 3
        assert_eq!(gb.gap_length, 2);
        assert_eq!(gb.gap_start, 14);
        // Make sure buffer stays the same length
        assert_eq!(gb.buffer.len(), original_buffer_length);
    }

    #[test]
    fn insert_multi_character_string_smaller_than_gap_at_start() {
        let mut gb = GapBuffer::with_data("Hello World");
        let original_buffer_length = gb.buffer.len();
        dbg!(&gb);
        gb.move_gap(0);
        gb.insert("Hi, ");
        assert_eq!(&gb.get_text_as_string(), "Hi, Hello World");
        // Assert gap changed as expected, length of addition was 4
        assert_eq!(gb.gap_length, 1);
        assert_eq!(gb.gap_start, 4);
        // Make sure buffer stays the same length
        assert_eq!(gb.buffer.len(), original_buffer_length);
    }

    #[test]
    fn insert_multi_character_string_larger_than_gap_at_start() {
        let mut gb = GapBuffer::with_data("Hello World");
        let original_buffer_length = gb.buffer.len();
        let str_to_insert = "Hi there, ";
        gb.move_gap(0);
        gb.insert(str_to_insert);
        assert_eq!(&gb.get_text_as_string(), "Hi there, Hello World");
        assert_eq!(gb.gap_length, 1029);
        assert_eq!(gb.gap_start, 10);
        // Make sure buffer stays the same length
        assert_eq!(
            gb.buffer.len(),
            original_buffer_length + str_to_insert.len() + 1024
        );
    }

    #[test]
    fn delete_5_at_start() {
        let mut gb = GapBuffer::with_data("Hello World");
        gb.delete_at(5, 0);
        assert_eq!(&gb.get_text_as_string(), " World");
        assert_eq!(gb.gap_length, 10);
        assert_eq!(gb.gap_start, 0);
    }

    #[test]
    #[should_panic]
    fn attempt_delete_5_after_data() {
        let mut gb = GapBuffer::with_data("Hello World");
        gb.delete_at(5, 11);
        assert_eq!(&gb.get_text_as_string(), " World");
        assert_eq!(gb.gap_length, 10);
        assert_eq!(gb.gap_start, 0);
    }
}
