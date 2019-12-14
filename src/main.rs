use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Background::Col, Color},
    input::{ButtonState, Key},
    lifecycle::{run, Event, Settings, State, Window},
    Result,
};
use rgy::{debug::NullDebugger, Config, Key as GBKey, Stream, System, VRAM_HEIGHT, VRAM_WIDTH};
use std::cell::RefCell;
use std::rc::Rc;
use stdweb::web::Date;

const SCALE: f32 = 2.0;

struct Context {
    sys: System<NullDebugger>,
    display: Display,
    kbd: Keyboard,
}

impl Context {
    fn poll(&mut self) {
        for _ in 0..3 {
            self.sys.poll();
        }
    }
}

fn state(state: ButtonState) -> bool {
    match state {
        ButtonState::Pressed => true,
        ButtonState::Held => true,
        ButtonState::Released => false,
        ButtonState::NotPressed => false,
    }
}

impl State for Context {
    fn new() -> Result<Self> {
        let kbd = Keyboard::new();
        let display = Display::new();
        let cfg = Config::new();
        let hw = Hardware::new(display.clone(), kbd.clone());

        let rom = include_bytes!(env!("WABOY_ROM"));

        let sys = System::new(cfg, rom, hw, NullDebugger);

        Ok(Self { sys, display, kbd })
    }

    fn update(&mut self, _window: &mut Window) -> Result<()> {
        self.poll();
        Ok(())
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        self.poll();

        let mut kbd = self.kbd.0.borrow_mut();

        match *event {
            Event::Key(Key::Up, s) => {
                kbd.up = state(s);
            }
            Event::Key(Key::Down, s) => {
                kbd.down = state(s);
            }
            Event::Key(Key::Left, s) => {
                kbd.left = state(s);
            }
            Event::Key(Key::Right, s) => {
                kbd.right = state(s);
            }
            Event::Key(Key::Z, s) => {
                kbd.a = state(s);
            }
            Event::Key(Key::X, s) => {
                kbd.b = state(s);
            }
            Event::Key(Key::N, s) => {
                kbd.start = state(s);
            }
            Event::Key(Key::M, s) => {
                kbd.select = state(s);
            }
            Event::Key(Key::Escape, ButtonState::Pressed) => {
                window.close();
            }
            _ => (),
        }

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        self.poll();

        window.clear(Color::WHITE)?;

        let ps = self.display.0.borrow();
        let ps = ps
            .iter()
            .enumerate()
            .map(|(x, v)| v.iter().enumerate().map(move |(y, p)| (x, y, p)))
            .flatten();

        for (x, y, p) in ps {
            let col = Color::from_rgba(
                (p & 0xffu32) as u8,
                ((p >> 8) & 0xffu32) as u8,
                ((p >> 16) & 0xffu32) as u8,
                1.0,
            );
            window.draw(
                &Rectangle::new((x as f32 * SCALE, y as f32 * SCALE), (SCALE, SCALE)),
                Col(col),
            )
        }

        Ok(())
    }
}

fn main() {
    run::<Context>(
        "Gayboy",
        Vector::new(
            (VRAM_WIDTH as f32 * SCALE) as u32,
            (VRAM_HEIGHT as f32 * SCALE) as u32,
        ),
        Settings {
            update_rate: 0.01,
            ..Settings::default()
        },
    );
}

#[derive(Clone, Debug)]
struct Display(Rc<RefCell<Vec<Vec<u32>>>>);

#[derive(Clone, Debug)]
struct Keyboard(Rc<RefCell<Inner>>);

#[derive(Default, Debug)]
struct Inner {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    a: bool,
    b: bool,
    start: bool,
    select: bool,
}

impl Display {
    fn new() -> Self {
        Self(Rc::new(RefCell::new(vec![
            vec![0u32; VRAM_HEIGHT];
            VRAM_WIDTH
        ])))
    }
}

impl Keyboard {
    fn new() -> Self {
        Self(Rc::new(RefCell::new(Inner::default())))
    }
}

struct Hardware {
    display: Display,
    kbd: Keyboard,
}

impl Hardware {
    fn new(display: Display, kbd: Keyboard) -> Self {
        Self { display, kbd }
    }
}

impl rgy::Hardware for Hardware {
    fn vram_update(&mut self, line: usize, buffer: &[u32]) {
        let y = line;

        for (x, col) in buffer.iter().enumerate() {
            self.display.0.borrow_mut()[x][y] = *col;
        }
    }

    fn joypad_pressed(&mut self, key: GBKey) -> bool {
        match key {
            GBKey::Up => self.kbd.0.borrow().up,
            GBKey::Down => self.kbd.0.borrow().down,
            GBKey::Left => self.kbd.0.borrow().left,
            GBKey::Right => self.kbd.0.borrow().right,
            GBKey::A => self.kbd.0.borrow().a,
            GBKey::B => self.kbd.0.borrow().b,
            GBKey::Start => self.kbd.0.borrow().start,
            GBKey::Select => self.kbd.0.borrow().select,
        }
    }

    fn sound_play(&mut self, _stream: Box<dyn Stream>) {}

    fn clock(&mut self) -> u64 {
        #[cfg(target_arch = "wasm32")]
        {
            Date::now() as u64 * 1000
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let epoch = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Couldn't get epoch");
            epoch.as_micros() as u64
        }
    }

    fn send_byte(&mut self, _b: u8) {}

    fn recv_byte(&mut self) -> Option<u8> {
        None
    }

    fn sched(&mut self) -> bool {
        true
    }

    fn load_ram(&mut self, size: usize) -> Vec<u8> {
        vec![0; size]
    }

    fn save_ram(&mut self, _ram: &[u8]) {}
}
