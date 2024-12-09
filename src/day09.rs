#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct FSEntryInner {
    file_id: u64,
    length: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum FSEntry {
    FreeSpace(usize),
    File(FSEntryInner),
}

#[derive(Debug, Clone)]
struct FSMap {
    fs: Vec<FSEntry>,
    left_most_free_space: usize,
}

impl FSMap {
    fn checksum(&self) -> u64 {
        let mut current_index: usize = 0;
        self.fs
            .iter()
            .map(|entry| match entry {
                FSEntry::File(entry) => {
                    let sum = ((current_index as u64)..(current_index + entry.length) as u64)
                        .sum::<u64>()
                        * entry.file_id;
                    current_index += entry.length;
                    sum
                }
                FSEntry::FreeSpace(length) => {
                    current_index += length;
                    0
                }
            })
            .sum()
    }

    fn shift_into_free_space_whole(&mut self) {
        // println!("{}", self);
        let mut current_file = self.fs.len() - 1;
        while current_file > self.left_most_free_space {
            let FSEntry::File(entry) = self.fs[current_file] else {
                current_file -= 1;
                continue;
            };

            self.insert_left_most_whole(entry, &mut current_file);
            current_file -= 1;
            // println!(
            // "{} (left most: {}, current: {})",
            // self, self.left_most_free_space, current_file
            // );
        }
    }

    fn shift_into_free_space(&mut self) {
        // println!("{}", self);
        while self.fs.len() > self.left_most_free_space {
            let FSEntry::File(last_entry) = self
                .fs
                .pop()
                .expect("self.fs is empty despite self.fs.len() > self.left_most_free_space")
            else {
                continue;
            };

            self.insert_left_most(last_entry);
            // println!("{} (left most: {})", self, self.left_most_free_space);
        }
    }

    fn insert_left_most_whole(&mut self, entry: FSEntryInner, entry_index: &mut usize) {
        let first_fitting = self.fs[self.left_most_free_space..*entry_index]
            .iter()
            .enumerate()
            .find_map(|(i, e)| {
                if let FSEntry::FreeSpace(free_space) = e {
                    (*free_space >= entry.length).then_some(i + self.left_most_free_space)
                } else {
                    None
                }
            });

        let first_fitting = match first_fitting {
            Some(i) => i,
            None => return,
        };

        self.fs[*entry_index] = FSEntry::FreeSpace(entry.length);
        let FSEntry::FreeSpace(ref mut free_space) = self.fs[first_fitting] else {
            panic!(
                "first_fitting ({first_fitting}) was not a free space ({:?})",
                self.fs[first_fitting]
            );
        };

        *free_space -= entry.length;

        if *free_space == 0 {
            self.fs[first_fitting] = FSEntry::File(entry);
        } else {
            self.fs.insert(first_fitting, FSEntry::File(entry));
            *entry_index += 1;
        }

        self.left_most_free_space = self
            .fs
            .iter()
            .enumerate()
            .find(|(_, &entry)| matches!(entry, FSEntry::FreeSpace(_)))
            .map(|(i, _)| i)
            .unwrap_or(self.fs.len());
    }

    fn insert_left_most(&mut self, mut entry: FSEntryInner) {
        while entry.length > 0 {
            if self.fs.len() <= self.left_most_free_space {
                self.fs.push(FSEntry::File(entry));
                return;
            }

            let FSEntry::FreeSpace(last_free_space) = &mut self.fs[self.left_most_free_space]
            else {
                panic!(
                    "self.left_most_free_space ({}) was not a free space ({:?})",
                    self.left_most_free_space, self.fs[self.left_most_free_space]
                );
            };

            if *last_free_space <= entry.length {
                let length = *last_free_space;

                self.fs[self.left_most_free_space] = FSEntry::File(FSEntryInner {
                    file_id: entry.file_id,
                    length,
                });
                entry.length -= length;

                self.left_most_free_space += self.fs[self.left_most_free_space..]
                    .iter()
                    .enumerate()
                    .find(|(_, &entry)| matches!(entry, FSEntry::FreeSpace(_)))
                    .map(|(i, _)| i)
                    .unwrap_or(self.fs.len());
            } else {
                *last_free_space -= entry.length;

                self.fs
                    .insert(self.left_most_free_space, FSEntry::File(entry));

                self.left_most_free_space += 1;
                return;
            }
        }
    }
}

impl std::fmt::Display for FSMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in self.fs.iter() {
            match entry {
                FSEntry::FreeSpace(length) => write!(f, "{}", ".".repeat(*length))?,
                FSEntry::File(entry) => {
                    write!(f, "{}", entry.file_id.to_string().repeat(entry.length))?
                }
            }
        }

        Ok(())
    }
}

fn parse(input: &[u8]) -> FSMap {
    let fs = input
        .iter()
        .enumerate()
        .map(|(index, &length)| {
            if index % 2 == 0 {
                FSEntry::File(FSEntryInner {
                    file_id: (index / 2) as u64,
                    length: (length - b'0') as usize,
                })
            } else {
                FSEntry::FreeSpace((length - b'0') as usize)
            }
        })
        .collect();

    FSMap {
        fs,
        left_most_free_space: 1,
    }
}

#[aoc(day09, part1)]
fn part1(input: &str) -> u64 {
    let mut fs = parse(input.trim().as_bytes());
    fs.shift_into_free_space();
    fs.checksum()
}

#[aoc(day09, part2)]
fn part2(input: &str) -> u64 {
    let mut fs = parse(input.trim().as_bytes());
    fs.shift_into_free_space_whole();
    fs.checksum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2333133121414131402";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 1928);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 2858);
    }
}
