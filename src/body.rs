pub(crate) struct Body {
    pub energy: i32,
}

impl Body {
    pub fn new(energy: i32) -> Self {
        Self { energy }
    }

    pub fn say<S>(&mut self, lang: S) -> ()
    where
        S: Into<String>,
    {
        if self.energy <= 0 {
            return ();
        }

        self.energy -= 5;

        print!("{}", lang.into());
    }
}
