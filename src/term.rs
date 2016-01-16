use letters::LETTERS_POS;
use colors::CPAIRS;
use ncurses::*;
use std::process::Command;


pub type Color = i16;


pub enum Source {
    StaticText(String),
    Subcommand(String),
}

impl Source {
    pub fn emit_string(&self) -> String {
        match *self {
            Source::StaticText(ref text) => text.clone(),
            Source::Subcommand(ref text) => {
                let split = text.split(" ").collect::<Vec<&str>>();
                let args = &split[1..];
                let cmd = split[0];
                let output = Command::new(cmd).args(args).output();
                match output {
                    Ok(r) => String::from_utf8(r.stdout).unwrap(),
                    Err(r) => "-".to_string(),
                }
            }
        }
    }
}

pub enum Content {
    Empty,
    Text(Source),
    BigText(Source),
}

// ======================================================================
// Window

pub struct Window {
    pub x: i32,
    pub y: i32,
    pub border_v: u64,
    pub border_h: u64,
    pub width: i32,
    pub height: i32,
    pub children: Vec<Window>,
    pub content: Content,
    pub _win: WINDOW,
}


impl Window {
    pub fn new(x: i32, y: i32, width: i32, height: i32, content: Content) -> Window {
        Window {
            x: x,
            y: y,
            width: width,
            height: height,
            border_h: 0,
            border_v: 0,
            children: vec![],
            content: content,
            _win: Window::_create_win(x, y, width, height),
        }
    }

    pub fn draw(&self) {
        let content = match self.content {
            Content::Empty => "".to_string(),
            Content::Text(ref source) => source.emit_string(),
            Content::BigText(ref source) => source.emit_string(),
        };

        werase(self._win);
        match self.content {
            Content::Empty => (),
            Content::Text(_) => self.print(&content, 1, 1, *CPAIRS.get("WHITE_ON_BLACK").unwrap()),
            Content::BigText(_) => {
                self.print_big(&content, 2, 1, *CPAIRS.get("BLACK_ON_WHITE").unwrap())
            }
        };
        box_(self._win, self.border_h, self.border_v);
        wrefresh(self._win);
    }

    fn _create_win(x: i32, y: i32, width: i32, height: i32) -> WINDOW {
        let win = newwin(height, width, y, x);
        wrefresh(win);
        win
    }

    pub fn print(&self, text: &String, x: i32, y: i32, color: Color) {
        wmove(self._win, y, x);
        wattron(self._win, COLOR_PAIR(color) as i32);
        for c in text.chars() {
            wprintw(self._win, &*c.to_string());
            if c == '\n' {
                wprintw(self._win, &*" ".to_string());
            }
        }
        wattroff(self._win, COLOR_PAIR(color) as i32);
    }

    pub fn print_big(&self, text: &String, mut x: i32, y: i32, color: Color) {
        for c in text.chars() {
            let pos = LETTERS_POS.get(&c);
            match pos {
                Some(ps) => {
                    for p in ps.iter() {
                        self.print(&" ".to_string(), x + p.0, y + p.1, color);
                    }
                }
                _ => (),
            }
            x += 8;
        }
    }
}

// ======================================================================
// Canvas

pub struct Canvas {
    pub width: i32,
    pub height: i32,
    pub root: Window,
    pub delay: i32,
}

impl Canvas {
    pub fn new() -> Canvas {
        Canvas {
            width: 0,
            height: 0,
            delay: 1000,
            root: Window::new(0, 0, 0, 0, Content::Empty),
        }
    }

    pub fn refresh(&mut self) {
        for win in &self.root.children {
            win.draw();
        }
        mv(0, 0);
    }

    pub fn initialize(&mut self) {
        // Start ncurses.
        initscr();
        raw();
        keypad(stdscr, true);
        noecho();

        // Invisible cursor.
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

        // start colors.
        start_color();

        // Get the screen bounds.
        getmaxyx(stdscr, &mut self.height, &mut self.width);

        refresh();
        let (w, h) = (self.width, self.height);
        self.root = Window::new(0, 0, w, h, Content::Empty);
    }

    pub fn cleanup(&mut self) {
        endwin();
    }
}

impl Drop for Canvas {
    fn drop(&mut self) {
        self.cleanup();
    }
}

pub fn destroy_win(win: WINDOW) {
    println!("Destroying {:?}", win);
    let ch = ' ' as chtype;
    wborder(win, ch, ch, ch, ch, ch, ch, ch, ch);
    wrefresh(win);
    delwin(win);
}
