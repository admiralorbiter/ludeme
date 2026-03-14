//! pong-76 — Faithful Pong (1972) mechanic demo for Ludeme
//!
//! Mechanic families: collision-response, scoring-pressure
//! Fidelity: faithful
//!
//! Shell API contract:
//!   - Emits SessionStart on init (with ParamManifest)
//!   - Emits FrameTick every frame
//!   - Emits StateChange when game state machine transitions
//!   - Emits MomentEmit on score (bookmarkable moment)
//!   - Reads params from window.__ludeme.getParam() each frame

use js_sys::Reflect;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};

// =============================================================================
// Constants
// =============================================================================

const CANVAS_W: f64 = 640.0;
const CANVAS_H: f64 = 480.0;

const PADDLE_W: f64 = 10.0;
const PADDLE_H: f64 = 60.0;
const PADDLE_MARGIN: f64 = 20.0;

const BALL_DEFAULT_SPEED: f64 = 4.0;
const PADDLE_DEFAULT_SPEED: f64 = 5.0;
const BALL_DEFAULT_SIZE: f64 = 8.0;

const WIN_SCORE: u32 = 7;
const SERVE_DELAY_FRAMES: u64 = 60;

// =============================================================================
// Shell API — call window.__ludeme.onEvent(json)
// =============================================================================

fn emit_event(event_json: &str) {
    let window = web_sys::window().expect("no window");
    if let Ok(ludeme) = Reflect::get(&window, &JsValue::from_str("__ludeme")) {
        if let Ok(on_event) = Reflect::get(&ludeme, &JsValue::from_str("onEvent")) {
            let f = on_event.dyn_ref::<js_sys::Function>().expect("onEvent is not a function");
            let _ = f.call1(&ludeme, &JsValue::from_str(event_json));
        }
    }
}

fn get_param(key: &str) -> Option<f64> {
    let window = web_sys::window()?;
    let ludeme = Reflect::get(&window, &JsValue::from_str("__ludeme")).ok()?;
    let get_param_fn = Reflect::get(&ludeme, &JsValue::from_str("getParam")).ok()?;
    let f = get_param_fn.dyn_ref::<js_sys::Function>()?;
    let result = f.call1(&ludeme, &JsValue::from_str(key)).ok()?;
    result.as_f64()
}

// =============================================================================
// Event types (serialised to JSON for the shell)
// =============================================================================

#[derive(Serialize)]
struct FrameTickData {
    frame:         u64,
    position:      Option<[f64; 2]>,
    active_states: Vec<String>,
}

#[derive(Serialize)]
struct StateChangeData {
    from_state: String,
    to_state:   String,
    frame:      u64,
}

#[derive(Serialize)]
struct MomentEmitData {
    scene_id:    &'static str,
    frame:       u64,
    state_blob:  Option<()>,
    player_label: Option<String>,
    auto_tags:   Vec<String>,
}

#[derive(Serialize)]
struct SessionEndData {
    frame_count:         u64,
    duration_ms:         u64,
    input_log_available: bool,
}

fn emit<T: Serialize>(event_type: &str, data: &T) {
    let payload = serde_json::json!({ "type": event_type, "data": data });
    emit_event(&payload.to_string());
}

// =============================================================================
// Game state
// =============================================================================

#[derive(Debug, Clone, PartialEq)]
enum PongState {
    Serve { countdown: u64 },
    Rally,
    Scored { winner: Side },
    GameOver { winner: Side },
}

impl PongState {
    fn label(&self) -> &'static str {
        match self {
            PongState::Serve { .. }  => "serve",
            PongState::Rally         => "rally",
            PongState::Scored { .. } => "scored",
            PongState::GameOver { .. } => "game_over",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Side { Left, Right }

impl Side {
    fn label(&self) -> &'static str {
        match self { Side::Left => "left", Side::Right => "right" }
    }
}

struct Ball {
    x: f64, y: f64,
    vx: f64, vy: f64,
    size: f64,
}

impl Ball {
    fn reset(speed: f64, size: f64) -> Self {
        // Serve toward right side, slight downward angle
        Ball { x: CANVAS_W / 2.0, y: CANVAS_H / 2.0, vx: speed, vy: speed * 0.6, size }
    }
}

struct Paddle {
    y: f64,
    speed: f64,
}

impl Paddle {
    fn new() -> Self { Paddle { y: CANVAS_H / 2.0 - PADDLE_H / 2.0, speed: PADDLE_DEFAULT_SPEED } }

    fn move_up(&mut self)   { self.y = (self.y - self.speed).max(0.0); }
    fn move_down(&mut self) { self.y = (self.y + self.speed).min(CANVAS_H - PADDLE_H); }
}

struct Keys {
    w: bool, s: bool,
    up: bool, down: bool,
}

impl Keys {
    fn new() -> Self { Keys { w: false, s: false, up: false, down: false } }
}

struct Game {
    ctx:       CanvasRenderingContext2d,
    ball:      Ball,
    paddle_l:  Paddle,
    paddle_r:  Paddle,
    score_l:   u32,
    score_r:   u32,
    state:     PongState,
    prev_state_label: String,
    frame:     u64,
    start_ms:  f64,
    keys:      Keys,
}

impl Game {
    fn new(ctx: CanvasRenderingContext2d, start_ms: f64) -> Self {
        Game {
            ctx,
            ball: Ball::reset(BALL_DEFAULT_SPEED, BALL_DEFAULT_SIZE),
            paddle_l: Paddle::new(),
            paddle_r: Paddle::new(),
            score_l: 0,
            score_r: 0,
            state: PongState::Serve { countdown: SERVE_DELAY_FRAMES },
            prev_state_label: "serve".to_string(),
            frame: 0,
            start_ms,
            keys: Keys::new(),
        }
    }

    // Read params from the shell (or use defaults)
    fn read_params(&mut self) {
        let ball_speed  = get_param("ball_speed").unwrap_or(BALL_DEFAULT_SPEED);
        let paddle_speed = get_param("paddle_speed").unwrap_or(PADDLE_DEFAULT_SPEED);
        let ball_size   = get_param("ball_size").unwrap_or(BALL_DEFAULT_SIZE);

        self.paddle_l.speed = paddle_speed;
        self.paddle_r.speed = paddle_speed;
        self.ball.size      = ball_size;
        // Ball speed is applied at serve time; live changes take effect next serve
        let _ = ball_speed;
    }

    fn current_state_label(&self) -> &'static str {
        self.state.label()
    }

    fn maybe_emit_state_change(&mut self) {
        let current = self.current_state_label().to_string();
        if current != self.prev_state_label {
            emit("state_change", &StateChangeData {
                from_state: self.prev_state_label.clone(),
                to_state:   current.clone(),
                frame:      self.frame,
            });
            self.prev_state_label = current;
        }
    }

    fn tick(&mut self) {
        self.frame += 1;
        self.read_params();

        // Input
        if self.keys.w    { self.paddle_l.move_up(); }
        if self.keys.s    { self.paddle_l.move_down(); }
        if self.keys.up   { self.paddle_r.move_up(); }
        if self.keys.down { self.paddle_r.move_down(); }

        match &self.state.clone() {
            PongState::Serve { countdown } => {
                if *countdown == 0 {
                    let speed = get_param("ball_speed").unwrap_or(BALL_DEFAULT_SPEED);
                    let size  = get_param("ball_size").unwrap_or(BALL_DEFAULT_SIZE);
                    self.ball = Ball::reset(speed, size);
                    self.state = PongState::Rally;
                } else {
                    self.state = PongState::Serve { countdown: countdown - 1 };
                }
            }

            PongState::Rally => {
                // Move ball
                self.ball.x += self.ball.vx;
                self.ball.y += self.ball.vy;

                // Top/bottom wall bounce
                if self.ball.y <= 0.0 {
                    self.ball.y  = 0.0;
                    self.ball.vy = self.ball.vy.abs();
                }
                if self.ball.y + self.ball.size >= CANVAS_H {
                    self.ball.y  = CANVAS_H - self.ball.size;
                    self.ball.vy = -self.ball.vy.abs();
                }

                // Left paddle collision
                let lx = PADDLE_MARGIN;
                if self.ball.x <= lx + PADDLE_W
                    && self.ball.x + self.ball.size >= lx
                    && self.ball.y + self.ball.size >= self.paddle_l.y
                    && self.ball.y <= self.paddle_l.y + PADDLE_H
                    && self.ball.vx < 0.0
                {
                    self.ball.x  = lx + PADDLE_W;
                    self.ball.vx = self.ball.vx.abs();
                    // Angle based on hit position
                    let rel = (self.ball.y + self.ball.size / 2.0 - (self.paddle_l.y + PADDLE_H / 2.0)) / (PADDLE_H / 2.0);
                    self.ball.vy = self.ball.vx.abs() * rel * 1.2;
                }

                // Right paddle collision
                let rx = CANVAS_W - PADDLE_MARGIN - PADDLE_W;
                if self.ball.x + self.ball.size >= rx
                    && self.ball.x <= rx + PADDLE_W
                    && self.ball.y + self.ball.size >= self.paddle_r.y
                    && self.ball.y <= self.paddle_r.y + PADDLE_H
                    && self.ball.vx > 0.0
                {
                    self.ball.x  = rx - self.ball.size;
                    self.ball.vx = -self.ball.vx.abs();
                    let rel = (self.ball.y + self.ball.size / 2.0 - (self.paddle_r.y + PADDLE_H / 2.0)) / (PADDLE_H / 2.0);
                    self.ball.vy = self.ball.vx.abs() * rel * 1.2;
                }

                // Score: ball exits left
                if self.ball.x + self.ball.size < 0.0 {
                    self.score_r += 1;
                    self.on_scored(Side::Right);
                }
                // Score: ball exits right
                if self.ball.x > CANVAS_W {
                    self.score_l += 1;
                    self.on_scored(Side::Left);
                }
            }

            PongState::Scored { .. } => {
                // Brief pause — transition to Serve or GameOver on next tick
                if self.score_l >= WIN_SCORE {
                    self.state = PongState::GameOver { winner: Side::Left };
                } else if self.score_r >= WIN_SCORE {
                    self.state = PongState::GameOver { winner: Side::Right };
                } else {
                    self.state = PongState::Serve { countdown: SERVE_DELAY_FRAMES };
                }
            }

            PongState::GameOver { .. } => {
                // Session end is emitted by the RAF loop when it detects GameOver.
                // Nothing to update here — state is terminal.
            }
        }

        self.maybe_emit_state_change();

        // FrameTick: emit every frame
        let active = vec![self.current_state_label().to_string()];
        emit("frame_tick", &FrameTickData {
            frame:         self.frame,
            position:      Some([self.ball.x, self.ball.y]),
            active_states: active,
        });
    }

    fn on_scored(&mut self, winner: Side) {
        // Emit MomentEmit — this is a bookmarkable moment
        emit("moment_emit", &MomentEmitData {
            scene_id:     "score",
            frame:        self.frame,
            state_blob:   None,
            player_label: Some(format!("{} scores", winner.label())),
            auto_tags:    vec![
                "score".to_string(),
                winner.label().to_string(),
                format!("{}-{}", self.score_l, self.score_r),
            ],
        });
        self.state = PongState::Scored { winner };
    }

    fn draw(&self) {
        let ctx = &self.ctx;

        // Clear
        ctx.set_fill_style_str("#0a0a0f");
        ctx.fill_rect(0.0, 0.0, CANVAS_W, CANVAS_H);

        // Centre line (dashed)
        ctx.set_stroke_style_str("rgba(255,255,255,0.15)");
        ctx.set_line_dash(&js_sys::Array::of2(
            &JsValue::from_f64(8.0), &JsValue::from_f64(8.0)
        )).ok();
        ctx.begin_path();
        ctx.move_to(CANVAS_W / 2.0, 0.0);
        ctx.line_to(CANVAS_W / 2.0, CANVAS_H);
        ctx.stroke();
        ctx.set_line_dash(&js_sys::Array::new()).ok();

        // Paddles
        ctx.set_fill_style_str("#f5f5f5");
        ctx.fill_rect(PADDLE_MARGIN, self.paddle_l.y, PADDLE_W, PADDLE_H);
        ctx.fill_rect(CANVAS_W - PADDLE_MARGIN - PADDLE_W, self.paddle_r.y, PADDLE_W, PADDLE_H);

        // Ball (amber accent for Ludeme style)
        ctx.set_fill_style_str("#f59e0b");
        ctx.fill_rect(self.ball.x, self.ball.y, self.ball.size, self.ball.size);

        // Score
        ctx.set_fill_style_str("rgba(255,255,255,0.7)");
        ctx.set_font("48px 'JetBrains Mono', monospace");
        ctx.set_text_align("right");
        let _ = ctx.fill_text(&self.score_l.to_string(), CANVAS_W / 2.0 - 20.0, 60.0);
        ctx.set_text_align("left");
        let _ = ctx.fill_text(&self.score_r.to_string(), CANVAS_W / 2.0 + 20.0, 60.0);

        // State label
        ctx.set_fill_style_str("rgba(255,255,255,0.25)");
        ctx.set_font("11px 'JetBrains Mono', monospace");
        ctx.set_text_align("left");
        let _ = ctx.fill_text(&format!("{} | f{}", self.state.label(), self.frame), 8.0, CANVAS_H - 8.0);

        // Controls hint
        ctx.set_text_align("right");
        let _ = ctx.fill_text("W/S  ·  ↑/↓", CANVAS_W - 8.0, CANVAS_H - 8.0);

        // Game over overlay
        if let PongState::GameOver { winner } = &self.state {
            ctx.set_fill_style_str("rgba(10,10,15,0.75)");
            ctx.fill_rect(0.0, 0.0, CANVAS_W, CANVAS_H);
            ctx.set_fill_style_str("#f59e0b");
            ctx.set_font("bold 32px 'Inter', sans-serif");
            ctx.set_text_align("center");
            let msg = format!("{} wins", winner.label());
            let _ = ctx.fill_text(&msg, CANVAS_W / 2.0, CANVAS_H / 2.0);
            ctx.set_fill_style_str("rgba(255,255,255,0.4)");
            ctx.set_font("14px 'Inter', sans-serif");
            let _ = ctx.fill_text("Refresh to play again", CANVAS_W / 2.0, CANVAS_H / 2.0 + 36.0);
        }

        // Serve countdown overlay
        if let PongState::Serve { countdown } = &self.state {
            if *countdown > SERVE_DELAY_FRAMES / 2 {
                ctx.set_fill_style_str("rgba(245,158,11,0.5)");
                ctx.set_font("bold 20px 'Inter', sans-serif");
                ctx.set_text_align("center");
                let _ = ctx.fill_text("SERVE", CANVAS_W / 2.0, CANVAS_H / 2.0 + 60.0);
            }
        }
    }
}

// =============================================================================
// Entry point
// =============================================================================

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Grab the canvas the shell put in the DOM
    let window   = web_sys::window().expect("no window");
    let document = window.document().expect("no document");

    let canvas: HtmlCanvasElement = document
        .get_element_by_id("ludeme-canvas")
        .expect("no #ludeme-canvas element")
        .dyn_into()?;

    canvas.set_width(CANVAS_W as u32);
    canvas.set_height(CANVAS_H as u32);

    let ctx: CanvasRenderingContext2d = canvas
        .get_context("2d")?
        .expect("no 2d context")
        .dyn_into()?;

    let start_ms = window.performance().map(|p| p.now()).unwrap_or(0.0);
    let game = Rc::new(RefCell::new(Game::new(ctx, start_ms)));

    // Emit SessionStart with param manifest
    emit("session_start", &serde_json::json!({
        "demo_id":   "pong-76",
        "branch_id": "main",
        "seed":      42,
        "param_manifest": {
            "params": [
                {
                    "key": "ball_speed",    "label": "Ball speed",
                    "kind": "float", "default": 4.0, "min": 1.0, "max": 10.0, "step": 0.5,
                    "group": "Ball"
                },
                {
                    "key": "paddle_speed", "label": "Paddle speed",
                    "kind": "float", "default": 5.0, "min": 1.0, "max": 12.0, "step": 0.5,
                    "group": "Paddles"
                },
                {
                    "key": "ball_size",   "label": "Ball size",
                    "kind": "float", "default": 8.0, "min": 4.0, "max": 24.0, "step": 2.0,
                    "group": "Ball"
                }
            ]
        }
    }));

    // --- Keyboard input ----------------------------------------------------
    {
        let game_ref = game.clone();
        let keydown = Closure::<dyn FnMut(KeyboardEvent)>::new(move |e: KeyboardEvent| {
            let mut g = game_ref.borrow_mut();
            match e.key().as_str() {
                "w" | "W"        => g.keys.w    = true,
                "s" | "S"        => g.keys.s    = true,
                "ArrowUp"        => { g.keys.up   = true; e.prevent_default(); }
                "ArrowDown"      => { g.keys.down = true; e.prevent_default(); }
                _ => {}
            }
        });
        window.add_event_listener_with_callback("keydown", keydown.as_ref().unchecked_ref())?;
        keydown.forget(); // keep alive
    }
    {
        let game_ref = game.clone();
        let keyup = Closure::<dyn FnMut(KeyboardEvent)>::new(move |e: KeyboardEvent| {
            let mut g = game_ref.borrow_mut();
            match e.key().as_str() {
                "w" | "W"   => g.keys.w    = false,
                "s" | "S"   => g.keys.s    = false,
                "ArrowUp"   => g.keys.up   = false,
                "ArrowDown" => g.keys.down = false,
                _ => {}
            }
        });
        window.add_event_listener_with_callback("keyup", keyup.as_ref().unchecked_ref())?;
        keyup.forget();
    }

    // --- Request animation frame loop -------------------------------------
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        {
            let mut game_borrow = game.borrow_mut();
            game_borrow.tick();
            game_borrow.draw();

            // Stop the loop on game over
            if matches!(game_borrow.state, PongState::GameOver { .. }) {
                let duration = web_sys::window()
                    .and_then(|w| w.performance())
                    .map(|p| p.now() - game_borrow.start_ms)
                    .unwrap_or(0.0) as u64;
                emit("session_end", &SessionEndData {
                    frame_count:         game_borrow.frame,
                    duration_ms:         duration,
                    input_log_available: false,
                });
                return; // don't schedule next frame
            }
        }

        // Schedule next frame
        web_sys::window()
            .expect("no window")
            .request_animation_frame(
                f.borrow().as_ref().unwrap().as_ref().unchecked_ref()
            )
            .expect("rAF failed");
    }));

    // Start the loop
    window.request_animation_frame(
        g.borrow().as_ref().unwrap().as_ref().unchecked_ref()
    )?;

    Ok(())
}
