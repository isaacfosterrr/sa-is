use sa_linear::*;


use rand::Rng;

fn naive_sa(s: &[u8]) -> Vec<usize> {
    let mut sa: Vec<usize> = (0..s.len()).collect();
    sa.sort_by(|&i, &j| s[i..].cmp(&s[j..]));
    sa
}

#[test]
fn test_sa_is() {
    let mut seed = 123456789u64;

    fn rng(seed: &mut u64) -> u64 {
        *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        *seed
    }

    for test_id in 0..1000 {
        let n = if test_id < 990 {
            (rng(&mut seed) % 2000 + 1) as usize
        } else {
            (rng(&mut seed) % 200_000 + 1) as usize
        };

        let s: Vec<u8> = (0..n)
            .map(|_| (b'a' + (rng(&mut seed) % 26) as u8))
            .collect();

        let text: Vec<u8> = s.iter().map(|&c| (c - b'a' + 1) as u8).collect();

        let sa_fast = build_sa_is(&text);

        // ---- VALIDATION ----
        if sa_fast.len() != n {
            println!("FAIL length mismatch");
            println!("test_id: {}", test_id);
            println!("n: {}", n);
            println!("sa_fast.len(): {}", sa_fast.len());
            println!("string: {:?}", String::from_utf8_lossy(&s));
            panic!();
        }

        if let Some(&bad) = sa_fast.iter().find(|&&x| x >= n) {
            println!("FAIL out-of-bounds index");
            println!("test_id: {}", test_id);
            println!("n: {}", n);
            println!("bad value: {}", bad);
            println!("string: {:?}", String::from_utf8_lossy(&s));
            println!("sa_fast (first 50): {:?}", &sa_fast[..sa_fast.len().min(50)]);
            panic!();
        }

        // check duplicates / missing
        let mut seen = vec![false; n];
        for &x in &sa_fast {
            if seen[x] {
                println!("FAIL duplicate index");
                println!("test_id: {}", test_id);
                println!("duplicate: {}", x);
                println!("string: {:?}", String::from_utf8_lossy(&s));
                panic!();
            }
            seen[x] = true;
        }

        if seen.iter().any(|&x| !x) {
            println!("FAIL missing index");
            println!("test_id: {}", test_id);
            println!("string: {:?}", String::from_utf8_lossy(&s));
            panic!();
        }

        // ---- NAIVE CHECK (small only) ----
        if n <= 2000 {
            let sa_naive = naive_sa(&s);

            if sa_fast != sa_naive {
                println!("FAIL mismatch");
                println!("test_id: {}", test_id);
                println!("n: {}", n);
                println!("string: {:?}", String::from_utf8_lossy(&s));
                println!("fast:  {:?}", sa_fast);
                println!("naive: {:?}", sa_naive);

                // print first differing position
                for i in 0..n {
                    if sa_fast[i] != sa_naive[i] {
                        println!("first diff at position {}", i);
                        println!("fast[{}] = {}", i, sa_fast[i]);
                        println!("naive[{}] = {}", i, sa_naive[i]);
                        break;
                    }
                }

                panic!();
            }
        }

        if test_id % 10 == 0 {
            println!("Passed test {}", test_id);
        }
    }

    println!("All tests passed!");
}

#[test]
fn test_find_lms() {
    let type_map = vec![L, S, L, L, S, L, L, S];
    assert_eq!(find_lms(&type_map), vec![1, 4, 7]);
}

#[test]
fn test_find_lms_complex() {
    let type_map = vec![L, S, L, L, S, L, L, S, L, L, L, S];
    assert_eq!(find_lms(&type_map), vec![1, 4, 7, 11]);
}


#[test]
fn test_frequencies_complex() {
    let freq = get_frequencies(b"mississippi");
    assert_eq!(freq[b'i' as usize], 4);
    assert_eq!(freq[b's' as usize], 4);
    assert_eq!(freq[b'p' as usize], 2);
    assert_eq!(freq[b'm' as usize], 1);

}

#[test]
fn test_heads_tails() {
    let freq = get_frequencies(b"cabbage");
    let (heads, tails) = get_heads_tails(&freq);

    for i in 0..255 {
        if freq[i] != 0 && freq[i+1] != 0 {
            assert!(tails[i] < heads[i+1]);
        }
    }

    for i in 0..256 {
        if freq[i] != 0 {
            assert_eq!(tails[i] - heads[i] + 1, freq[i]);
        }
    }
}

#[test]
fn test_heads_tails_complex() {
    let freq = get_frequencies(b"mississippi");
    let (heads, tails) = get_heads_tails(&freq);

    for i in 0..255 {
        if freq[i] != 0 && freq[i+1] != 0 {
            assert!(tails[i] < heads[i+1]);
        }
    }

    for i in 0..256 {
        if freq[i] != 0 {
            assert_eq!(tails[i] - heads[i] + 1, freq[i]);
        }
    }
}

#[test]
fn test_final_sa() {
    let cases = vec!["cabbage", "banana", "aaaaaa", "abbc"];

    for s in cases {
        let bytes = s.as_bytes();
        let sa = build_sa_is(bytes);

        let mut seen = vec![false; bytes.len()];
        for &i in &sa { seen[i] = true; }
        assert!(seen.iter().all(|&x| x), "not a permutation: {s}");

        for i in 1..sa.len() {
            assert!(bytes[sa[i-1]..] <= bytes[sa[i]..], "not sorted: {s}");
        }
    }
}

#[test]
fn test_final_sa_harder_cases() {
    let cases = vec![
        "mississippi",
        "abababab",
        "abcabcabc",
        "zzzyzzzy",
        "baabaabac",
        "aaabbbbababbbaaaacda"
    ];

    for s in cases {
        let bytes = s.as_bytes();
        let sa = build_sa_is(bytes);
        let naive = naive_sa(bytes);

        assert_eq!(sa, naive, "naive gives different result: {s}");

        let mut seen = vec![false; bytes.len()];
        for &i in &sa { seen[i] = true; }
        assert!(seen.iter().all(|&x| x), "not a permutation: {s}");

        for i in 1..sa.len() {
            assert!(bytes[sa[i-1]..] <= bytes[sa[i]..], "not sorted: {s}");
        }
    }
}