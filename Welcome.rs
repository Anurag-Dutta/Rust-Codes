#![allow(unknown_lints)]
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::io;
use std::io::prelude::*;

#[macro_export]
#[allow(unused_macros)]
macro_rules! read_vec {
    ($a:expr, $t:ty, $n:expr) => {{
        let mut temp_vec = Vec::<$t>::new();

        for _ in 0..$n {
            let x: $t = $a.token();
            temp_vec.push(x);
        }

        temp_vec
    }};
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! print_vec {
    ($w:expr, $a:expr) => {{
        for x in $a {
            let _ = write!($w, "{} ", x);
        }
        let _ = write!($w, "{}", "\n");
    }};
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! read_matrix {
    ($a:expr, $t:ty, $n:expr, $m:expr) => {{
        let mut temp_vec = Vec::<Vec<$t>>::new();

        for _ in 0..$n {
            let mut row_vec = Vec::<$t>::new();
            for _ in 0..$m {
                let x: $t = $a.token();
                row_vec.push(x);
            }
            temp_vec.push(row_vec);
        }

        temp_vec
    }};
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! read_vec2 {
    ($a:expr, $t1:ty, $t2:ty, $n:expr) => {{
        let mut temp_vec1 = Vec::<$t1>::new();
        let mut temp_vec2 = Vec::<$t2>::new();

        for _ in 0..$n {
            let x: $t1 = $a.token();
            temp_vec1.push(x);
            let x: $t2 = $a.token();
            temp_vec2.push(x);
        }

        (temp_vec1, temp_vec2)
    }};
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! read_vec3 {
    ($a:expr, $t1:ty, $t2:ty, $t3:ty, $n:expr) => {{
        let mut temp_vec1 = Vec::<$t1>::new();
        let mut temp_vec2 = Vec::<$t2>::new();
        let mut temp_vec3 = Vec::<$t3>::new();

        for _ in 0..$n {
            let x: $t1 = $a.token();
            temp_vec1.push(x);
            let x: $t2 = $a.token();
            temp_vec2.push(x);
            let x: $t3 = $a.token();
            temp_vec3.push(x);
        }

        (temp_vec1, temp_vec2, temp_vec3)
    }};
}


trait TokenStream<T> {
    fn token(&mut self) -> T;
}

struct FastInput<R> {
    stdin: R,
    pos: usize,
}

impl<R: BufRead> From<R> for FastInput<R> {
    fn from(r: R) -> Self {
        FastInput { stdin: r, pos: 0 }
    }
}

impl<R: BufRead> TokenStream<u8> for FastInput<R> {
    fn token(&mut self) -> u8 {
        loop {
            if let Ok(buf) = self.stdin.fill_buf() {
                while self.pos < buf.len() {
                    self.pos += 1;
                    if buf[self.pos - 1] > 32 {
                        return buf[self.pos - 1];
                    }
                }
                if self.pos == 0 {
                    return 0;
                }
            } else {
                return 0;
            }
            self.stdin.consume(self.pos);
            self.pos = 0;
        }
    }
}

impl<R: BufRead> TokenStream<Vec<u8>> for FastInput<R> {
    fn token(&mut self) -> Vec<u8> {
        let mut ans = Vec::new();
        let mut parse_token = false;
        loop {
            if let Ok(buf) = self.stdin.fill_buf() {
                if !parse_token {
                    while self.pos < buf.len() && buf[self.pos] <= 32 {
                        self.pos += 1;
                    }
                }
                while self.pos < buf.len() && buf[self.pos] > 32 {
                    parse_token = true;
                    ans.push(buf[self.pos]);
                    self.pos += 1;
                }
                if self.pos != buf.len() || self.pos == 0 {
                    return ans;
                }
            }
            self.stdin.consume(self.pos);
            self.pos = 0;
        }
    }
}

macro_rules! impl_token_stream_signed {
    ($($t:ident),+) => {$(
        impl<R: BufRead> TokenStream<$t> for FastInput<R> {
           fn token(&mut self) -> $t {
                let mut ans = 0 as $t;
                let mut sign = 1 as $t;
                let mut parse_token = false;
                loop {
                    if let Ok(buf) = self.stdin.fill_buf() {
                        if !parse_token {
                            while self.pos < buf.len() &&
                            ((buf[self.pos] <= 32) || (buf[self.pos] == b'-')) {
                                if buf[self.pos] == b'-' {
                                    sign = -1;
                                }
                                self.pos += 1;
                            }
                        }
                        while self.pos < buf.len() && buf[self.pos] > 32 {
                            parse_token = true;
                            ans = ans * 10 + (buf[self.pos] - b'0') as $t;
                            self.pos += 1;
                        }
                        if self.pos != buf.len() || self.pos == 0 {
                            return ans*sign;
                        }
                    }
                    self.stdin.consume(self.pos);
                    self.pos = 0;
                }
           }
        }
    )+}
}

macro_rules! impl_token_stream_unsigned {
    ($($t:ident),+) => {$(
        impl<R: BufRead> TokenStream<$t> for FastInput<R> {
           fn token(&mut self) -> $t {
                let mut ans = 0 as $t;
                let mut parse_token = false;
                loop {
                    if let Ok(buf) = self.stdin.fill_buf() {
                        if !parse_token {
                            while self.pos < buf.len() &&
                            (buf[self.pos] <= 32) {
                                self.pos += 1;
                            }
                        }
                        while self.pos < buf.len() && buf[self.pos] > 32 {
                            parse_token = true;
                            ans = ans * 10 + (buf[self.pos] - b'0') as $t;
                            self.pos += 1;
                        }
                        if self.pos != buf.len() || self.pos == 0 {
                            return ans;
                        }
                    }
                    self.stdin.consume(self.pos);
                    self.pos = 0;
                }
           }
        }
    )+}
}

impl_token_stream_signed!(i64);
impl_token_stream_signed!(i32);
impl_token_stream_unsigned!(u64);
impl_token_stream_unsigned!(u32);
impl_token_stream_unsigned!(usize);

impl<R: BufRead> TokenStream<char> for FastInput<R> {
    fn token(&mut self) -> char {
        loop {
            if let Ok(buf) = self.stdin.fill_buf() {
                while self.pos < buf.len() {
                    self.pos += 1;
                    if buf[self.pos - 1] > 32 {
                        return buf[self.pos - 1] as char;
                    }
                }
            }
            self.stdin.consume(self.pos);
            self.pos = 0;
        }
    }
}

impl<R: BufRead> TokenStream<Vec<char>> for FastInput<R> {
    fn token(&mut self) -> Vec<char> {
        let mut ans = Vec::<char>::new();
        let mut parse_token = false;
        loop {
            if let Ok(buf) = self.stdin.fill_buf() {
                if !parse_token {
                    while self.pos < buf.len() && buf[self.pos] <= 32 {
                        self.pos += 1;
                    }
                }
                while self.pos < buf.len() && buf[self.pos] > 32 {
                    parse_token = true;
                    ans.push(buf[self.pos] as char);
                    self.pos += 1;
                }
                if self.pos != buf.len() || self.pos == 0 {
                    return ans;
                }
            }
            self.stdin.consume(self.pos);
            self.pos = 0;
        }
    }
}

impl<R: BufRead> TokenStream<String> for FastInput<R> {
    fn token(&mut self) -> String {
        let mut ans = Vec::<char>::new();
        let mut parse_token = false;
        loop {
            if let Ok(buf) = self.stdin.fill_buf() {
                if !parse_token {
                    while self.pos < buf.len() && buf[self.pos] <= 32 {
                        self.pos += 1;
                    }
                }
                while self.pos < buf.len() && buf[self.pos] > 32 {
                    parse_token = true;
                    ans.push(buf[self.pos] as char);
                    self.pos += 1;
                }
                if self.pos != buf.len() || self.pos == 0 {
                    return ans.iter().cloned().collect::<String>();
                }
            }
            self.stdin.consume(self.pos);
            self.pos = 0;
        }
    }
}

impl<R: BufRead> TokenStream<f64> for FastInput<R> {
    fn token(&mut self) -> f64 {
        let mut ans = 0u64;
        let mut floating_value = 0u64;
        let mut pow_10 = 1u64;
        let mut sign = 1i32;
        let mut parse_token = false;
        loop {
            if let Ok(buf) = self.stdin.fill_buf() {
                if !parse_token {
                    while self.pos < buf.len() && ((buf[self.pos] <= 32) || (buf[self.pos] == b'-'))
                    {
                        if buf[self.pos] == b'-' {
                            sign = -1;
                        }
                        self.pos += 1;
                    }
                }
                while self.pos < buf.len() && buf[self.pos] > 32 && buf[self.pos] != b'.' {
                    parse_token = true;
                    ans = ans * 10 + (buf[self.pos] - b'0') as u64;
                    self.pos += 1;
                }

                if self.pos != buf.len() && buf[self.pos] == b'.' {
                    self.pos += 1;

                    while self.pos < buf.len() && buf[self.pos] > 32 {
                        parse_token = true;
                        floating_value = floating_value * 10 + (buf[self.pos] - b'0') as u64;
                        pow_10 = pow_10 * 10;
                        self.pos += 1;
                    }
                }

                if self.pos != buf.len() || self.pos == 0 {
                    return (sign as f64)
                        * ((ans as f64) + (floating_value as f64) / (pow_10 as f64));
                }
            }
            self.stdin.consume(self.pos);
            self.pos = 0;
        }
    }
}

#[allow(dead_code)]
fn num_to_vec(n: u64) -> Vec<u8> {
    let mut temp_vec = Vec::<u8>::new();
    let mut temp_n = n;

    while temp_n > 0 {
        temp_vec.push((temp_n % 10) as u8);
        temp_n /= 10;
    }
    temp_vec.reverse();
    return temp_vec;
}

#[allow(unused_must_use)]
#[allow(unused_mut)]
#[allow(unused_variables)]
fn main() 
{
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut input = FastInput::from(stdin.lock());
    let mut writer = io::BufWriter::new(stdout.lock());
    println!("Hi, what's your name? ");
    let name: String = input.token();
    println!("\nWelcome {}",name);
}