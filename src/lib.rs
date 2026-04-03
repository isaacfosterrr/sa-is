
use std::vec;

pub const S: u8 = 0;
pub const L: u8 = 1;

pub trait Symbol: Ord + Copy {
    fn as_index(&self) -> usize;
    fn display(&self) -> String;
    fn sentinel() -> Self;
}

impl Symbol for u8 {
    fn as_index(&self) -> usize { *self as usize }
    fn display(&self) -> String {
        if self.is_ascii() { (*self as char).to_string() }
        else { ".".to_string() }
    }
    fn sentinel() -> Self { 0 }
}

impl Symbol for usize {
    fn as_index(&self) -> usize { *self }
    fn display(&self) -> String { self.to_string() }
    fn sentinel() -> Self { 0 }
}
pub  fn  print_aligned<T: Symbol>(bytes: &[T], type_map: &[u8], sa: &[usize]) {
    assert_eq!(bytes.len(), type_map.len(), "Vectors must be the same length");

    // Print indices
    print!("Index: ");
    for i in 0..bytes.len() {
        print!("{:<3} ", i);
    }
    println!();

    // Print types
    print!("Type : ");
    for t in type_map {
        let tipe: char = match t {
            &S => 'S',
            &L => 'L',
            _ => '?',
        };
        print!("{:<3} ", tipe);
    }
    println!();

    // Print characters safely
    print!("Char : ");
    for &b in bytes {
        print!("{:<3} ", b.display());
    }
    println!();
    // Print SA
    print!("rank : ");
    for &b in sa {
        if b != usize::MAX { 
            print!("{:<3} ", b);
        } else { 
            print!("{:<3} ", '.');
        }
        
    }

    println!();
}

pub fn is_lms(type_map: &[u8], i: usize) -> bool {
    if i < 1 || i == usize::MAX {
        return false;
    }
    if type_map[i] == S && type_map[i - 1] == L {
        true
    } else {
        false
    }
}

pub  fn  find_types<T: Symbol>(bytes: &[T]) -> Vec<u8> {
    if bytes.is_empty() { return vec![]; }
    let mut type_map: Vec<u8> = vec![0; bytes.len()];

    for i in (0..bytes.len() - 1).rev(){
       if bytes[i] < bytes[i + 1] {
            type_map[i] = S;
        } else if bytes[i] == bytes[i + 1] {
            type_map[i] = type_map[i + 1];
        } else {
            type_map[i] = L;
        }
    }
    type_map[bytes.len() - 1] = S;
    type_map
}

pub  fn  find_lms(type_map: &[u8]) -> Vec<usize>{
    let mut lms_positions = Vec::new();
    for i in 1..type_map.len() {
        if is_lms(type_map, i) {
            lms_positions.push(i);
        }
    }
    lms_positions
}

pub fn get_frequencies<T: Symbol>(bytes: &[T]) -> Vec<usize> {
    let mut max_symbol = 0;
    for &b in bytes {
        let v = b.as_index();
        if v > max_symbol {
            max_symbol = v;
        }
    }

    let size = (max_symbol + 1).max(256);
    let mut freq = vec![0usize; size];

    for &b in bytes {
        freq[b.as_index()] += 1;
    }

    freq
}
pub fn get_heads_tails(freq: &[usize]) -> (Vec<usize>, Vec<usize>) {
    let mut cur_start = 0;
    let mut heads = vec![0usize; freq.len()];
    let mut tails = vec![0usize; freq.len()];
    for i in 0..freq.len() {
        if freq[i] != 0 {
            heads[i] = cur_start;
            cur_start += freq[i];
            tails[i] = cur_start - 1;
        }
    }
    (heads, tails)
}

pub  fn  place_lms<T: Symbol>(lms: &[usize], mut tails: &mut [usize], bytes: &[T]) -> Vec<usize> {
    //placing lms in end of buckets
    let mut sa = vec![usize::MAX; bytes.len()];
    for i in (0..lms.len()).rev() {
        let ch = bytes[lms[i]].as_index();
        let bucket_end = tails[ch];
        sa[bucket_end] = lms[i];
        if tails[ch] > 0 {
            tails[ch] -= 1;
        }
    }
    sa
}

pub  fn  induce_l<T: Symbol>(bytes: &[T], sa: &mut[usize], type_map: &[u8], heads: &mut [usize])  {
    //left to right scan
    for i in 0..bytes.len() {
        if sa[i] == usize::MAX {
            continue;
        } else {
            let index = sa[i];
            if index > 0 && type_map[index - 1] == L {      // in the heads we have chars as indexes and its bucket start as values 
                let ch = bytes[index - 1];
                let bucket_start = heads[ch.as_index()];
                sa[bucket_start] = index - 1;
                heads[ch.as_index()] += 1;
            }
        }
    }
}

pub  fn  induce_s<T: Symbol>(bytes: &[T], sa: &mut[usize], type_map: &[u8], tails: &mut [usize])  {
    // right to left scan
    for i in (0..bytes.len()).rev() {
        if sa[i] == usize::MAX {
            continue;
        } else {
            let index = sa[i];
            if index > 0 && type_map[index - 1] == S {      // in the tails we have chars as indexes and its bucket end as values 
                let ch = bytes[index - 1];
                let bucket_end = tails[ch.as_index()];
                sa[bucket_end] = index - 1;
                if tails[ch.as_index()] > 0 { tails[ch.as_index()] -= 1 }
            }
        }
    }
}

pub fn is_lms_equal<T: Symbol>(bytes: &[T], type_map: &[u8], mut lms_one: usize, mut lms_two: usize) -> bool {
    if bytes[lms_one] == bytes[lms_two] {
        lms_one += 1;
        lms_two += 1;
    } else {
        return false;
    }
    while lms_one < bytes.len() && lms_two < type_map.len() {
        if !is_lms(type_map, lms_one) && !is_lms(type_map, lms_two) {
            if bytes[lms_one] != bytes[lms_two] || type_map[lms_one] != type_map[lms_two] {
                return false;
            }
            lms_one += 1;
            lms_two += 1;
        } else if is_lms(type_map, lms_one) && is_lms(type_map, lms_two) {
            return true;
        } else {
            return false; // one ended before the other
        }
    }
    true
}

pub fn summarize<T: Symbol>(guessed_sa: &[usize], bytes: &[T], type_map: &[u8]) -> (Vec<usize>, Vec<usize>, usize) {
    let mut intermidiate_summary: Vec<usize> = vec![usize::MAX; bytes.len()];
    let mut summary: Vec<usize> = Vec::new();
    let mut suffix_offsets: Vec<usize> = Vec::new();
    let mut last_lms: Option<usize> = None;
    let mut current_name = 1;

    for i in guessed_sa {
        if is_lms(type_map, *i) {   // here lms' got placed at index, which is same as their original index in bytes
            if let Some(last) = last_lms {
                if !is_lms_equal(bytes, type_map, *i, last) {
                    current_name += 1;
                }
            }
            intermidiate_summary[*i] = current_name;
            last_lms = Some(*i);
        }

    }
    for (index, &name) in intermidiate_summary.iter().enumerate() {
        if name != usize::MAX {
            summary.push(name);
            suffix_offsets.push(index);
        }
    }

    (summary, suffix_offsets, current_name + 1)
}
pub fn  build_sa_is<T: Symbol>(bytes: &[T]) -> Vec<usize> {
    let mut bytes_with_sentinel = bytes.to_vec();
    bytes_with_sentinel.push(T::sentinel());
    let bytes = &bytes_with_sentinel;

    let freq = get_frequencies(bytes);
    
    let (heads, tails) = get_heads_tails(&freq);
    let type_map = &find_types(bytes);
    let lms = find_lms(type_map);
    //println!("{:?}", lms);


    let mut guessed_sa = place_lms(&lms, &mut tails.clone(), bytes);
    //println!("{:?}", guessed_sa);
    //println!();
    induce_l(bytes, &mut guessed_sa, type_map, &mut heads.clone());
    //print_aligned(bytes, type_map, &guessed_sa);
    //println!();
    induce_s(bytes, &mut guessed_sa, type_map, &mut tails.clone());
    //print_aligned(bytes, type_map, &guessed_sa);
    let (summary, suffix_offsets, name_count) = summarize(&guessed_sa, bytes, &type_map);
    let sorted_lms: Vec<usize> = if name_count - 1 == summary.len() {
        guessed_sa.iter().copied()
            .filter(|&i| i != usize::MAX && is_lms(type_map, i))
            .collect()
    } else {
        // recurse
        let recursive_sa = build_sa_is(&summary);
        recursive_sa.iter().map(|&i| suffix_offsets[i]).collect()
    };

    //place lms in right positions
    //println!("sorted_lms: {:?}", sorted_lms);
    let (heads, tails) = get_heads_tails(&freq);
    let mut final_sa = place_lms(&sorted_lms, &mut tails.clone(), bytes);
    induce_l(&bytes, &mut final_sa, type_map, &mut heads.clone());
    induce_s(&bytes, &mut final_sa, type_map, &mut tails.clone());

    //println!("summary: {:?}", summary);
    //println!("suffix_offsets: {:?}", suffix_offsets);
    final_sa[1..].to_vec()
}
