#[derive(Clone, Copy, PartialEq, Debug)]
enum Frame {
    Open(u16, u16),
    Spare(u16),
    Bonus(u16),
    Strike
}

impl Frame {
    pub fn throw1(&self) -> u16 {
        match *self {
            Frame::Open(t1, _) => t1,
            Frame::Spare(t1) => t1,
            Frame::Strike => 10,
            Frame::Bonus(t) => t,
        }
    }

    pub fn throw2(&self) -> u16 {
        match *self {
            Frame::Open(_, t2) => t2,
            Frame::Spare(t1) => 10 - t1,
            Frame::Strike => unimplemented!(),
            Frame::Bonus(_) => unimplemented!(),
        }
    }

    pub fn score(&self) -> u16 {
        match *self {
            Frame::Bonus(t) => t,
            Frame::Open(t1, t2) => t1 + t2,
            Frame::Spare(_) | Frame::Strike => 10
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Rolling {
    Regular,
    Extra(u16),
    Over
}

#[derive(Debug)]
pub struct BowlingGame {
    frames: Vec<Frame>,
    rolling: Rolling,
    partial: Option<u16>
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frames: Vec::with_capacity(10),
            rolling: Rolling::Regular,
            partial: None,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), ()> {
        if pins > 10 {
            return Err(());
        }

        match self.rolling {
            Rolling::Over => {
                return Err(());
            }
            Rolling::Regular => {
                match (self.partial, pins) {
                    (None, 10) => {
                        self.frames.push(Frame::Strike);
                    }
                    (None, pins) => {
                        self.partial = Some(pins);
                    }
                    (Some(prev_pins), pins) if prev_pins + pins > 10 => {
                        return Err(());
                    }
                    (Some(prev_pins), pins) if prev_pins + pins == 10 => {
                        self.frames.push(Frame::Spare(prev_pins));
                        self.partial = None;
                    }
                    (Some(prev_pins), pins) => {
                        self.frames.push(Frame::Open(prev_pins, pins));
                        self.partial = None;
                    }
                }

                if self.frames.len() == 10 {
                    self.rolling = match self.frames.last() {
                        Some(&Frame::Spare(..)) => Rolling::Extra(1),
                        Some(&Frame::Strike) => Rolling::Extra(2),
                        _ => Rolling::Over
                    };
                }
            }
            Rolling::Extra(remaining) => {
                let frame = match pins {
                    10 => Frame::Strike,
                    _ => Frame::Bonus(pins),
                };

                match self.frames.last() {
                    Some(&Frame::Bonus(partial)) if partial + pins > 10 => {
                        return Err(());
                    },
                    _ => ()
                }

                self.frames.push(frame);

                self.rolling = if remaining == 1 {
                    Rolling::Over
                } else {
                    Rolling::Extra(remaining - 1)
                };
            }
        }

        Ok(())
    }

    pub fn score(&self) -> Result<u16, ()> {
        if self.rolling != Rolling::Over {
            return Err(());
        }

        let pins: u16 = self.frames.iter().map(Frame::score).sum();
        let bonus: u16 = self.frames[0..10]
            .windows(3)
            .map(|window| {
                match (window[0], window[1], window[2]) {
                    (Frame::Strike, Frame::Strike, frame) => 10 + frame.throw1(),
                    (Frame::Spare(..), frame, _) => frame.throw1(),
                    (Frame::Strike, frame, _) => frame.throw1() + frame.throw2(),
                    _ => 0,
                }
            })
            .sum();
        let extra_bonus: u16 = if self.frames.len() > 10 {
            let f = &self.frames[8..];
            if f.len() == 4 {
                match (f[0], f[1], f[2], f[3]) {
                    (Frame::Strike, Frame::Strike, Frame::Strike, Frame::Strike) => 20,
                    _ => 0,
                }
            } else {
                0
            }
        } else {
            0
        };

        Ok(pins + bonus + extra_bonus)
    }
}