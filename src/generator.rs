use anyhow::Result;
use colored::*;
use std::io::{BufWriter, Write};
use std::time::Instant;
use crate::utils; // Import utils

pub struct Generator<W: Write> {
    writer: BufWriter<W>,
    min_len: usize,
    max_len: usize,
    verbose: bool,
    start_time: Instant,
}

impl<W: Write> Generator<W> {
    pub fn new(writer: W, min_len: usize, max_len: usize, verbose: bool) -> Self {
        Self {
            writer: BufWriter::new(writer),
            min_len,
            max_len,
            verbose,
            start_time: Instant::now(),
        }
    }

    fn write_line(&mut self, text: &str) -> Result<()> {
        let len = text.len();
        if len >= self.min_len && len <= self.max_len {
            self.writer.write_all(text.as_bytes())?;
            self.writer.write_all(b"\n")?;
            if self.verbose {
                let elapsed = self.start_time.elapsed().as_secs();
                println!("\t[{:?} s]\t{}", elapsed, format!("Generating '{}'", text).blue());
            }
        }
        Ok(())
    }

    fn append_sequences(&mut self, base: &str) -> Result<()> {
        for i in 0..=9 { self.write_line(&format!("{}{}", base, i))?; }
        for i in 0..=99 { self.write_line(&format!("{}{:02}", base, i))?; }
        for i in 0..=999 { self.write_line(&format!("{}{:03}", base, i))?; }
        for i in 0..=9999 { self.write_line(&format!("{}{:04}", base, i))?; }
        Ok(())
    }

    fn append_dates_full(&mut self, base: &str) -> Result<()> {
        for d in 1..=31 {
            for m in 1..=12 {
                for y in 1950..=2030 {
                     self.write_line(&format!("{}{:02}{:02}{}", base, d, m, y))?;
                }
            }
        }
        Ok(())
    }

    fn append_dates_short(&mut self, base: &str) -> Result<()> {
        for d in 1..=31 {
            for m in 1..=12 {
                for y in 0..=99 {
                     self.write_line(&format!("{}{:02}{:02}{:02}", base, d, m, y))?;
                }
            }
        }
        Ok(())
    }

    fn append_infix_sequences(&mut self, base: &str, symbol: char) -> Result<()> {
        for i in 0..=9 { self.write_line(&format!("{}{}{}", base, symbol, i))?; }
        for i in 0..=99 { self.write_line(&format!("{}{}{:02}", base, symbol, i))?; }
        for i in 0..=999 { self.write_line(&format!("{}{}{:03}", base, symbol, i))?; }
        for i in 0..=9999 { self.write_line(&format!("{}{}{:04}", base, symbol, i))?; }
        for d in 1..=31 { for m in 1..=12 { for y in 1950..=2030 {
            self.write_line(&format!("{}{}{:02}{:02}{}", base, symbol, d, m, y))?;
        }}}
        for d in 1..=31 { for m in 1..=12 { for y in 0..=99 {
            self.write_line(&format!("{}{}{:02}{:02}{:02}", base, symbol, d, m, y))?;
        }}}
        Ok(())
    }

    pub fn generate_for_word(&mut self, name: &str, minimal: bool, chars: &str) -> Result<()> {
        let n_cap = utils::capitalize(name);
        let n_may = name.to_uppercase();
        let n_rev: String = name.chars().rev().collect();
        let n_rev_cap = utils::capitalize(&n_rev); 
        let n_cap_rev: String = n_cap.chars().rev().collect();
        let n_rev_may: String = n_rev.to_uppercase();

        if self.verbose {
            let elapsed = self.start_time.elapsed().as_secs();
             println!("\t[{:?} s]\t{}", elapsed, format!("Generating '{}'", name).blue());
        }

        self.write_line(name)?;
        self.write_line(&n_cap)?;
        self.write_line(&n_may)?;

        if !minimal {
            self.write_line(&n_rev)?;
            self.write_line(&n_cap_rev)?;
            self.write_line(&n_rev_cap)?;
            self.write_line(&n_rev_may)?;
        }

        let l33t_v = utils::l33t_vowels(name);
        let l33t_v_cap = utils::l33t_vowels(&n_cap);
        let l33t_v_may = utils::l33t_vowels(&n_may);

        self.write_line(&l33t_v)?;
        self.write_line(&l33t_v_cap)?;
        self.write_line(&l33t_v_may)?;

        let has_s = name.contains(['s', 'S']);
        if has_s {
            self.write_line(&utils::l33t_vowels_s(name))?;
            self.write_line(&utils::l33t_vowels_s(&n_cap))?;
            self.write_line(&utils::l33t_vowels_s(&n_may))?;
        }

        if name.contains(['a', 'A']) {
            self.write_line(&utils::l33t_a(name))?;
            self.write_line(&utils::l33t_a(&n_cap))?;
            self.write_line(&utils::l33t_a(&n_may))?;
        }
        if name.contains(['e', 'E']) {
            self.write_line(&utils::l33t_e(name))?;
            self.write_line(&utils::l33t_e(&n_cap))?;
            self.write_line(&utils::l33t_e(&n_may))?;
        }
        if name.contains(['i', 'I']) {
            self.write_line(&utils::l33t_i(name))?;
            self.write_line(&utils::l33t_i(&n_cap))?;
            self.write_line(&utils::l33t_i(&n_may))?;
        }
        if name.contains(['o', 'O']) {
            self.write_line(&utils::l33t_o(name))?;
            self.write_line(&utils::l33t_o(&n_cap))?;
            self.write_line(&utils::l33t_o(&n_may))?;
        }

        for symbol in chars.chars() {
            self.write_line(&format!("{}{}", name, symbol))?;
            self.write_line(&format!("{}{}", n_cap, symbol))?;
            self.write_line(&format!("{}{}", n_may, symbol))?;

            if !minimal {
                self.write_line(&format!("{}{}", n_rev, symbol))?;
                self.write_line(&format!("{}{}", n_cap_rev, symbol))?;
                self.write_line(&format!("{}{}", n_rev_cap, symbol))?;
                self.write_line(&format!("{}{}", n_rev_may, symbol))?;
            }

            self.write_line(&format!("{}{}", utils::l33t_vowels_s(name), symbol))?;
            self.write_line(&format!("{}{}", utils::l33t_vowels_s(&n_cap), symbol))?;
            self.write_line(&format!("{}{}", utils::l33t_vowels_s(&n_may), symbol))?;

            if has_s {
                self.write_line(&format!("{}{}", utils::l33t_s_only(name), symbol))?;
                self.write_line(&format!("{}{}", utils::l33t_s_only(&n_cap), symbol))?;
                self.write_line(&format!("{}{}", utils::l33t_s_only(&n_may), symbol))?;
            }

            if name.contains(['a', 'A']) {
                self.write_line(&format!("{}{}", utils::l33t_a(name), symbol))?;
                self.write_line(&format!("{}{}", utils::l33t_a(&n_cap), symbol))?;
                self.write_line(&format!("{}{}", utils::l33t_a(&n_may), symbol))?;
            }
            if name.contains(['e', 'E']) {
                self.write_line(&format!("{}{}", utils::l33t_e(name), symbol))?;
                self.write_line(&format!("{}{}", utils::l33t_e(&n_cap), symbol))?;
                self.write_line(&format!("{}{}", utils::l33t_e(&n_may), symbol))?;
            }
            if name.contains(['i', 'I']) {
                self.write_line(&format!("{}{}", utils::l33t_i(name), symbol))?;
                self.write_line(&format!("{}{}", utils::l33t_i(&n_cap), symbol))?;
                self.write_line(&format!("{}{}", utils::l33t_i(&n_may), symbol))?;
            }
            if name.contains(['o', 'O']) {
                self.write_line(&format!("{}{}", utils::l33t_o(name), symbol))?;
                self.write_line(&format!("{}{}", utils::l33t_o(&n_cap), symbol))?;
                self.write_line(&format!("{}{}", utils::l33t_o(&n_may), symbol))?;
            }
        }

        self.append_sequences(name)?;
        
        for i in 0..=9 { self.write_line(&format!("{}{}", n_cap, i))?; }
        for i in 0..=99 { self.write_line(&format!("{}{:02}", n_cap, i))?; }
        for i in 0..=999 { self.write_line(&format!("{}{:03}", n_cap, i))?; }
        for i in 0..=9999 { self.write_line(&format!("{}{:04}", n_cap, i))?; }

        self.append_sequences(&n_may)?;

        if !minimal {
            self.append_sequences(&n_rev)?;
            for i in 0..=9 { self.write_line(&format!("{}{}", n_cap_rev, i))?; }
            for i in 0..=99 { self.write_line(&format!("{}{:02}", n_cap_rev, i))?; }
            for i in 0..=999 { self.write_line(&format!("{}{:03}", n_cap_rev, i))?; }
            for i in 0..=9999 { self.write_line(&format!("{}{:04}", n_cap_rev, i))?; }

            for i in 0..=9 { self.write_line(&format!("{}{}", n_rev_cap, i))?; }
            for i in 0..=99 { self.write_line(&format!("{}{:02}", n_rev_cap, i))?; }
            for i in 0..=999 { self.write_line(&format!("{}{:03}", n_rev_cap, i))?; }
            for i in 0..=9999 { self.write_line(&format!("{}{:04}", n_rev_cap, i))?; }
            
            self.append_sequences(&n_rev_may)?;
        }

        self.append_dates_full(name)?;
        self.append_dates_full(&n_cap)?;
        self.append_dates_full(&n_may)?;

        if !minimal {
            self.append_dates_full(&n_rev)?;
            self.append_dates_full(&n_cap_rev)?;
            self.append_dates_full(&n_rev_cap)?;
            self.append_dates_full(&n_rev_may)?;
        }

        self.append_dates_short(name)?;
        self.append_dates_short(&n_cap)?;
        self.append_dates_short(&n_may)?;

        if !minimal {
            self.append_dates_short(&n_rev)?;
            self.append_dates_short(&n_cap_rev)?;
            self.append_dates_short(&n_rev_cap)?;
            self.append_dates_short(&n_rev_may)?;
        }

        for symbol in chars.chars() {
            self.append_infix_sequences(name, symbol)?;
            self.append_infix_sequences(&n_cap, symbol)?;
            self.append_infix_sequences(&n_may, symbol)?;

            if !minimal {
                self.append_infix_sequences(&n_rev, symbol)?;
                self.append_infix_sequences(&n_cap_rev, symbol)?;
                self.append_infix_sequences(&n_rev_cap, symbol)?;
                self.append_infix_sequences(&n_rev_may, symbol)?;
            }

            self.append_infix_sequences(&utils::l33t_vowels(name), symbol)?;
            self.append_infix_sequences(&utils::l33t_vowels(&n_cap), symbol)?;

            if has_s {
                self.append_infix_sequences(&utils::l33t_vowels_s(name), symbol)?;
                self.append_infix_sequences(&utils::l33t_s_only(name), symbol)?;
                self.append_infix_sequences(&utils::l33t_s_only(&n_cap), symbol)?;
                self.append_infix_sequences(&utils::l33t_s_only(&n_may), symbol)?;
            }

            if name.contains(['a', 'A']) {
                self.append_infix_sequences(&utils::l33t_a(name), symbol)?;
                self.append_infix_sequences(&utils::l33t_a(&n_cap), symbol)?;
                self.append_infix_sequences(&utils::l33t_a(&n_may), symbol)?;
            }
            if name.contains(['e', 'E']) {
                self.append_infix_sequences(&utils::l33t_e(name), symbol)?;
                self.append_infix_sequences(&utils::l33t_e(&n_cap), symbol)?;
                self.append_infix_sequences(&utils::l33t_e(&n_may), symbol)?;
            }
            if name.contains(['i', 'I']) {
                self.append_infix_sequences(&utils::l33t_i(name), symbol)?;
                self.append_infix_sequences(&utils::l33t_i(&n_cap), symbol)?;
                self.append_infix_sequences(&utils::l33t_i(&n_may), symbol)?;
            }
            if name.contains(['o', 'O']) {
                self.append_infix_sequences(&utils::l33t_o(name), symbol)?;
                self.append_infix_sequences(&utils::l33t_o(&n_cap), symbol)?;
                self.append_infix_sequences(&utils::l33t_o(&n_may), symbol)?;
            }
        }
        Ok(())
    }
}