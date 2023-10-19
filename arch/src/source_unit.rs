use serde::{Deserialize, Serialize};

use crate::Location;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceUnit {
    name: String,
    data: Vec<u8>,
    lines: Vec<Line>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
struct Line {
    pub start: usize,
    pub end: usize,
}

impl SourceUnit {
    pub fn new(name: String, data: Vec<u8>) -> Self {
        let mut lines = Vec::new();

        {
            let mut idx = 0;
            let mut start = 0;

            loop {
                if idx >= data.len() {
                    lines.push(Line { start, end: idx });

                    break;
                }

                let byte = data[idx];

                if byte == b'\n' {
                    lines.push(Line {
                        start,
                        end: idx + 1,
                    });
                    start = idx + 1;
                } else if byte == b'\r' && matches!(data.get(idx + 1), Some(b'\n')) {
                    lines.push(Line {
                        start,
                        end: idx + 2,
                    });
                    start = idx + 2;
                    idx += 1;
                }

                idx += 1;
            }
        }

        Self { name, data, lines }
    }

    pub fn new_anonymous(data: Vec<u8>) -> Self {
        Self::new(String::from("<anonymous>"), data)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn lines_count(&self) -> usize {
        self.lines.len()
    }

    pub fn find_location(&self, position: usize) -> Option<Location> {
        for (idx, line) in self.lines.iter().enumerate() {
            if position >= line.start && position < line.end {
                return Some(Location {
                    line: idx + 1,
                    column: ((position - line.start) % line.end) + 1,
                    offset: position,
                });
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::SourceUnit;
    use crate::Location;

    #[test]
    fn n_line_endings() {
        let source = "line1\nline2\nline3\n\n";
        let unit = SourceUnit::new(String::from("<anonymous>"), source.as_bytes().to_vec());

        assert_eq!(unit.lines_count(), 5)
    }

    #[test]
    fn rn_line_endings() {
        let source = "line1\r\nline2\r\nline3\r\n\r\n";
        let unit = SourceUnit::new(String::from("<anonymous>"), source.as_bytes().to_vec());

        assert_eq!(unit.lines_count(), 5)
    }

    #[test]
    fn empty() {
        let source = "";
        let unit = SourceUnit::new(String::from("<anonymous>"), source.as_bytes().to_vec());

        assert_eq!(unit.lines_count(), 1)
    }

    #[test]
    fn location() {
        let source = r#"
# A basic program
start:
add $r0 1 0
sub $r5 $r0 0

# Jump
jmp start

# Print stack pointer and program counter
dbg $sp
dbg $pc
        "#;
        let unit = SourceUnit::new(String::from("<anonymous>"), source.as_bytes().to_vec());

        assert_eq!(
            unit.find_location(1),
            Some(Location {
                line: 2,
                column: 1,
                offset: 1
            })
        );
        assert_eq!(
            unit.find_location(9),
            Some(Location {
                line: 2,
                column: 9,
                offset: 9
            })
        );
        assert_eq!(
            unit.find_location(19),
            Some(Location {
                line: 3,
                column: 1,
                offset: 19
            })
        );

        let source = "# Test comment";
        let unit = SourceUnit::new(String::from("<anonymous>"), source.as_bytes().to_vec());

        assert_eq!(
            unit.find_location(0),
            Some(Location {
                line: 1,
                column: 1,
                offset: 0
            })
        )
    }
}
