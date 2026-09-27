pub struct Cfg<T> {
    pub option: Option<T>
}

impl<T> Cfg<T> {
    pub fn some(initial: T) -> Self {
        Self {
            option: Some(initial)
        }
    }
    pub fn none() -> Self {
        Self {
            option: None
        }
    }
    pub fn default<Fnc>(self, fnc: Fnc) -> T 
    where 
        Fnc: FnOnce() -> T
    {
        match self.option {
            Some(int) => int,
            None => fnc()
        }
    }
    pub fn otherwisse(self, cfg: Cfg<T>) -> Self {
        if self.option.is_some() {
            return self ;
        } else {
            cfg
        }
    }
}
