/*
Central file for the polar_game module
*/

mod player;
pub mod object;
mod enemy;
mod flare;
mod sun;
mod frame;
mod high_score;
pub mod builder;
pub use self::builder::PolarGameBuilder;

use self::player::Player;
use self::object::{Part,Object,Point,collision};
use self::flare::Flare;
use self::sun::Sun;
use self::enemy::Enemy;
use self::frame::PolarFrame;
use self::high_score::HighScore;
use time;
use rand;
use rand::distributions::exponential::Exp;
use rand::distributions::IndependentSample;
use rand::distributions::range::Range;
use gg::games::{GameInput, Game};
use gg::input::keyboard::KeyboardInput;
use gg::input::joystick::JoystickInput;
use gg::games::view_details::{PolarViewDetails, ViewDetails};
use gg::rendering::{PlainText, StandardPrimitive, PolarPixel, WindowSpec, StandardRenderable};
use gg::debug::*;
use na::{Vector4, Rotation2};

pub struct PolarGame{
    player: Player,
    flares: Vec<Flare>,
    sun: Sun,
    pub input_keys: InputKeys,
    frame: PolarFrame,
    pub setup: GameSetup,
    time: Times,
    pub state: GameState,
    external_input: ExternalInput,
    view_details: PolarViewDetails,
    high_score: HighScore
}

impl PolarGame {
    pub fn new(setup: GameSetup) -> PolarGame{
        PolarGame{
            player: Player::new(setup.player_start, setup.player_width),
            flares: Vec::new(),
            sun: Sun::new(1.0),
            input_keys: InputKeys::default(),
            time: Times::new(0.0),
            frame: PolarFrame::new(0.5, 0.05, Point{x: 0.01, y: 0.02}, setup.radial_max),
            setup: setup,
            state: GameState::new(),
            external_input: Default::default(),
            view_details:  Default::default(),
            high_score: HighScore::new()
        }
    }

    pub fn reset(&mut self) {
        self.player = Player::new(self.setup.player_start, self.setup.player_width);
        self.high_score.reset();
        self.time = Times::new(0.0);
        self.flares = Vec::new();
    }

    fn update_view_details(&mut self) {
        self.view_details.radial_shift = self.player.get_position().x - 0.75;
        self.view_details.rotation_angle = self.player.get_position().y + 0.25 + self.player.get_width() / 2.0;
        self.view_details.length_total = (self.player.get_position().x + 0.25).max(1.0);
    }
}

impl Game for PolarGame {
    type Primitive = StandardPrimitive;

    fn init(&mut self) {
        self.time = Times::new(time::precise_time_s());
        self.high_score.reset();
    }

    fn update_input(&mut self) {
        self.input_keys.jump_radial = (self.external_input.kbd.get_up() as isize - (self.external_input.kbd.get_down() as isize)) as f64 * 0.3;
        
        if self.external_input.gamepad.get_y_axis().abs() > 0.1 {
            self.input_keys.jump_radial = self.external_input.gamepad.get_y_axis() * 0.3;
        }
       
        self.input_keys.jump_angle = (self.external_input.kbd.get_right() as isize - (self.external_input.kbd.get_left() as isize)) as f64 * 0.3;

        if self.external_input.gamepad.get_x_axis().abs() > 0.1 {
            self.input_keys.jump_angle = self.external_input.gamepad.get_x_axis() * 0.3;
        }
        
        match (self.external_input.kbd.get_p(), self.input_keys.pause, self.input_keys.pause_lock) {
            (true, false, false) => { self.input_keys.pause = true; self.input_keys.pause_lock = true; },
            (false, true, true) => { self.input_keys.pause_lock = false; },
            (true, true, false) => { self.input_keys.pause = false; self.input_keys.pause_lock = true; },
            (false, false, true) => { self.input_keys.pause_lock = false; },
            _ => () 
        };
        
        self.input_keys.reset = self.external_input.kbd.get_r();
    }

    fn update_logic(&mut self, t_step: f64){
        debug_clock_start("Logic::update_logic");

        if self.input_keys.pause { return; }
        if self.input_keys.reset {
            self.reset();
            return;
        }
        
        let shift = Point{x: self.input_keys.jump_radial,
                          y: self.input_keys.jump_angle / 2.0};
        self.time.elapsed += t_step;

        self.player.update_position(shift, t_step, self.setup);
        for mut f in self.flares.iter_mut(){
            f.update_position(t_step, &self.player);
            if collision(&*f, &self.player){
                self.player.collide();;
            }
        }
        if collision(&self.sun, &self.player){
            self.player.collide();;
        }

        let current_flares = self.flares.clone();
        let (_, flares_trimmed) : (Vec<Flare>, Vec<Flare>)
            = current_flares.into_iter().partition(|f| f.terminate_flag(Point{x: -1.0, y: self.setup.radial_max + 2.0}));
        self.flares = flares_trimmed;


        if self.time.elapsed - self.time.previous_flare > self.time.til_flare{
            let mut rng = rand::thread_rng();
            let unif = Range::new(0.0, 1.0);
            let sa = unif.ind_sample(&mut rng);
            let r = unif.ind_sample(&mut rng) / 20.0 + 0.02;
            let a = unif.ind_sample(&mut rng) / 50.0 + 0.005;
            let v = unif.ind_sample(&mut rng) / 2.0 + 0.1;
            let new_flare = Flare::new(Point{x: r, y: a}, sa, v);
            self.flares.push(new_flare);
            self.time.previous_flare = self.time.elapsed;
            let emit_average = 10.0 + self.time.elapsed - self.time.start;
            let exp = Exp::new(emit_average);
            self.time.til_flare = exp.ind_sample(&mut rng);
        }

        if !self.player.destroyed {
            self.high_score.update(t_step);
        }
        
        self.update_view_details();
        debug_clock_stop("Logic::update_logic");
    }

    fn get_view(&self) -> ViewDetails {
        ViewDetails::Polar(self.view_details.clone())
    }

    fn get_renderables(&mut self, _: WindowSpec) -> Vec<Box<StandardRenderable>> {
        debug_clock_start("Render::get_renderables");
        let mut rend_vec: Vec<Part> = Vec::new();
        for f in self.frame.get_render_parts().into_iter(){
            rend_vec.push(f);
        }
        for f in self.player.get_render_parts().into_iter(){
            rend_vec.push(f);
        }
        let sun_part = self.sun.get_render_parts()[0];
        debug_clock_start("Render::get_renderables::flares");
        for f in self.flares.iter(){
            let flare_part = f.get_render_parts()[0];
            rend_vec.push(flare_part);
        }
        debug_clock_stop("Render::get_renderables::flares");
        rend_vec.push(sun_part);
        let mut output: Vec<Box<StandardRenderable>> = rend_vec.into_iter()
            .map(|p| -> Box<StandardRenderable> {Box::new(PolarPixel::from(p))}).collect();

        let score_text = self.high_score.get_score_text();
        let record_text = self.high_score.get_record_text();
        output.push(Box::new(score_text));
        output.push(Box::new(record_text));
        
        debug_clock_stop("Render::get_renderables");
        output
    }

    fn get_input<'a>(&'a mut self) -> Option<&'a mut GameInput> {
         Some(&mut self.external_input)
    }

    fn on_exit(&mut self) {
        self.high_score.update_high_score();
    }
}

#[derive(Default, Copy, Clone)]
pub struct InputKeys{
    pub jump_angle: f64,
    pub jump_radial: f64,
    pub reset: bool,
    pub pause: bool,
    pub pause_lock: bool
}

#[derive(Copy, Clone)]
pub struct GameSetup{
    pub radial_max: f64,
    pub player_start: Point,
    pub player_width: Point,
    pub tunnel_mode: bool
}

impl Default for GameSetup {
    fn default() -> Self {
        GameSetup {
            radial_max: 8.0,
            player_start: Point{x: 4.0, y: 0.75},
            player_width: Point{x: 0.02, y: 0.01},
            tunnel_mode: true
        }
    }
}

#[derive(Copy, Clone)]
pub struct GameState{
    pub player_death: bool,
    pub survival_time: f64,
}

impl GameState{
    pub fn new() -> GameState{
        GameState{ player_death: false,
                   survival_time: 0.0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Times{
    til_flare: f64,
    previous_flare: f64,
    start: f64,
    elapsed: f64,
}

impl Times{
    pub fn new(start_time: f64) -> Times{
        let mut rng = rand::thread_rng();
        let exp = Exp::new(1.0);
        Times{ til_flare: exp.ind_sample(&mut rng),
               previous_flare: start_time,
               start: start_time,
               elapsed: start_time,
        }
    }
}

#[derive(Clone, Default)]
struct ExternalInput {
    kbd: KeyboardInput,
    gamepad: JoystickInput
}

impl GameInput for ExternalInput {
    fn get_kbd_inp<'a>(&'a mut self) -> Option<&'a mut KeyboardInput> { Some(&mut self.kbd) }
    fn get_joystick_inp<'a>(&'a mut self) -> Option<&'a mut JoystickInput> { Some(&mut self.gamepad) }
}
