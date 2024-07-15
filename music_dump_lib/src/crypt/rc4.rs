/// https://highassurance.rs/chp2/dynamic_assurance_2.html#:~:text=The%20Key%2DScheduling%20Algorithm%20(KSA,logic%20in%20Rc4%20's%20constructor.
/// this piece of code is from the upper site
pub struct Rc4 {
    s: [u8; 256],
    i: u8,
    j: u8,
}

impl Rc4 {
    /// Init a new Rc4 stream cipher instance
    pub fn new(key: &[u8]) -> Self {
        // Verify valid key length (40 to 2048 bits)
        assert!(5 <= key.len() && key.len() <= 256);

        // Zero-init our struct
        let mut rc4 = Rc4 {
            s: [0; 256],
            i: 0,
            j: 0,
        };

        // Cipher state identity permutation
        for (i, b) in rc4.s.iter_mut().enumerate() {
            // s[i] = i
            *b = i as u8;
        }

        // Process for 256 iterations, get starting cipher state permutation
        let mut j: u8 = 0;
        for i in 0..256 {
            // j = (j + s[i] + key[i % key_len]) % 256
            j = j.wrapping_add(rc4.s[i]).wrapping_add(key[i % key.len()]);

            // Swap values of s[i] and s[j]
            rc4.s.swap(i, j as usize);
        }

        // Return our initialized Rc4
        rc4
    }

    fn prga_next(&mut self) -> u8 {
        // i = (i + 1) mod 256
        self.i = self.i.wrapping_add(1);

        // j = (j + s[i]) mod 256
        self.j = self.j.wrapping_add(self.s[self.i as usize]);

        // Swap values of s[i] and s[j]
        self.s.swap(self.i as usize, self.j as usize);

        // k = s[(s[i] + s[j]) mod 256]
        self.s[(self.s[self.i as usize].wrapping_add(self.s[self.j as usize])) as usize]
    }

    pub fn apply_keystream(&mut self, data: &mut [u8]) {
        for b_ptr in data {
            *b_ptr ^= self.prga_next();
        }
    }
}