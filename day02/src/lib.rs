use std::fs;

pub fn read_input() -> String {
    fs::read_to_string("./input.txt").expect("Should be able to read input file")
}

pub struct Range {
    low: i64,
    high: i64,
}

#[derive(Clone, PartialEq)]
pub enum PatternKind {
    Twice,
    Any,
}

impl Range {
    pub fn new(low: i64, high: i64) -> Range {
        Range { low, high }
    }

    pub fn invalid_ids(&self, pk: PatternKind) -> Vec<i64> {
        let mut invalids: Vec<i64> = Vec::new();
        for id in self.low..=self.high {
            if Self::is_invalid(id, pk.clone()) {
                invalids.push(id);
            }
        }
        invalids
    }

    fn is_invalid(id: i64, pk: PatternKind) -> bool {
        let digits = id.ilog10() + 1;
        if pk == PatternKind::Twice && digits % 2 != 0 {
            return false;
        }
        let pattern_sizes = match pk {
            PatternKind::Any => 1..=(digits / 2),
            PatternKind::Twice => (digits / 2)..=(digits / 2),
        };
        for pattern_size in pattern_sizes {
            if digits % pattern_size != 0 {
                continue;
            }
            let pattern_divisor = 10_i64.pow(pattern_size);
            let pattern = id % 10_i64.pow(pattern_size);
            let mut pattern_holds = true;
            for pattern_instance in (1..digits / pattern_size).map(|n| n * pattern_size) {
                if (id / 10_i64.pow(pattern_instance)) % pattern_divisor != pattern {
                    pattern_holds = false;
                    break;
                }
            }
            if pattern_holds {
                // println!("Invalid id: {id}");
                return true;
            }
        }
        false
    }
}
