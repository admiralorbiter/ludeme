//! jump-feel — Platformer jump mechanics demo for Ludeme
//!
//! Mechanic families: movement, timing-windows
//! Fidelity: interpreted
//!
//! Shell API contract:
//!   - Emits SessionStart on init (with ParamManifest)
//!   - Emits FrameTick every frame
//!   - Emits StateChange when player state changes (grounded/jumping/falling/coyote)
//!   - Emits MomentEmit on coin collection and level completion
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

const PLAYER_W: f64 = 20.0;
const PLAYER_H: f64 = 28.0;

const DEFAULT_GRAVITY: f64 = 0.55;
const DEFAULT_JUMP_FORCE: f64 = -10.0;
const DEFAULT_MOVE_SPEED: f64 = 4.0;
const DEFAULT_COYOTE_FRAMES: f64 = 6.0;

const COIN_SIZE: f64 = 12.0;

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
// Event types
// =============================================================================

#[derive(Serialize)]
struct FrameTickData {
    frame: u64,
    position: Option<[f64; 2]>,
    active_states: Vec<String>,
}

#[derive(Serialize)]
struct StateChangeData {
    from_state: String,
    to_state: String,
    frame: u64,
}

#[derive(Serialize)]
struct MomentEmitData {
    scene_id: &'static str,
    frame: u64,
    state_blob: Option<()>,
    player_label: Option<String>,
    auto_tags: Vec<String>,
}

#[derive(Serialize)]
struct SessionEndData {
    frame_count: u64,
    duration_ms: u64,
    input_log_available: bool,
}

fn emit<T: Serialize>(event_type: &str, data: &T) {
    let payload = serde_json::json!({ "type": event_type, "data": data });
    emit_event(&payload.to_string());
}

// =============================================================================
// Platform and coin data
// =============================================================================

struct Platform {
    x: f64, y: f64, w: f64, h: f64,
    color: &'static str,
}

struct Coin {
    x: f64, y: f64,
    collected: bool,
}

fn create_platforms() -> Vec<Platform> {
    vec![
        // Ground
        Platform { x: 0.0,   y: 440.0, w: 640.0, h: 40.0,  color: "#1a1a2e" },
        // Platforms — ascending route
        Platform { x: 60.0,  y: 370.0, w: 100.0, h: 14.0, color: "#2a2a4e" },
        Platform { x: 200.0, y: 320.0, w: 80.0,  h: 14.0, color: "#2a2a4e" },
        Platform { x: 320.0, y: 270.0, w: 120.0, h: 14.0, color: "#2a2a4e" },
        Platform { x: 480.0, y: 310.0, w: 100.0, h: 14.0, color: "#2a2a4e" },
        // Higher tier
        Platform { x: 100.0, y: 210.0, w: 90.0,  h: 14.0, color: "#3a3a5e" },
        Platform { x: 250.0, y: 160.0, w: 100.0, h: 14.0, color: "#3a3a5e" },
        Platform { x: 420.0, y: 190.0, w: 80.0,  h: 14.0, color: "#3a3a5e" },
        Platform { x: 530.0, y: 140.0, w: 90.0,  h: 14.0, color: "#3a3a5e" },
        // Top tier — goal area
        Platform { x: 260.0, y: 80.0,  w: 120.0, h: 14.0, color: "#4a4a6e" },
    ]
}

fn create_coins() -> Vec<Coin> {
    vec![
        // Ground level
        Coin { x: 100.0, y: 420.0, collected: false },
        Coin { x: 200.0, y: 420.0, collected: false },
        Coin { x: 350.0, y: 420.0, collected: false },
        Coin { x: 500.0, y: 420.0, collected: false },
        // On platforms
        Coin { x: 100.0, y: 345.0, collected: false },
        Coin { x: 230.0, y: 295.0, collected: false },
        Coin { x: 370.0, y: 245.0, collected: false },
        Coin { x: 520.0, y: 285.0, collected: false },
        // Higher platforms
        Coin { x: 135.0, y: 185.0, collected: false },
        Coin { x: 290.0, y: 135.0, collected: false },
        Coin { x: 450.0, y: 165.0, collected: false },
        Coin { x: 565.0, y: 115.0, collected: false },
        // Top — goal coin
        Coin { x: 315.0, y: 55.0,  collected: false },
    ]
}

// =============================================================================
// Player state machine
// =============================================================================

#[derive(Debug, Clone, PartialEq)]
enum PlayerState {
    Grounded,
    Jumping,
    Falling,
    CoyoteTime,  // brief grace period after leaving a platform edge
}

impl PlayerState {
    fn label(&self) -> &'static str {
        match self {
            PlayerState::Grounded   => "grounded",
            PlayerState::Jumping    => "jumping",
            PlayerState::Falling    => "falling",
            PlayerState::CoyoteTime => "coyote_time",
        }
    }
}

// =============================================================================
// Game state
// =============================================================================

struct Keys {
    left: bool, right: bool, jump: bool,
}
impl Keys { fn new() -> Self { Keys { left: false, right: false, jump: false } } }

struct Game {
    ctx: CanvasRenderingContext2d,
    px: f64, py: f64,         // player position (top-left)
    vx: f64, vy: f64,         // velocity
    state: PlayerState,
    prev_state_label: String,
    coyote_counter: u64,      // frames remaining for coyote time
    jump_held: bool,          // for variable-height jump
    platforms: Vec<Platform>,
    coins: Vec<Coin>,
    score: u32,
    total_coins: u32,
    frame: u64,
    start_ms: f64,
    keys: Keys,
    completed: bool,
}

impl Game {
    fn new(ctx: CanvasRenderingContext2d, start_ms: f64) -> Self {
        let coins = create_coins();
        let total = coins.len() as u32;
        Game {
            ctx,
            px: 60.0, py: 410.0,
            vx: 0.0, vy: 0.0,
            state: PlayerState::Grounded,
            prev_state_label: "grounded".to_string(),
            coyote_counter: 0,
            jump_held: false,
            platforms: create_platforms(),
            coins,
            score: 0,
            total_coins: total,
            frame: 0,
            start_ms,
            keys: Keys::new(),
            completed: false,
        }
    }

    fn maybe_emit_state_change(&mut self) {
        let current = self.state.label().to_string();
        if current != self.prev_state_label {
            emit("state_change", &StateChangeData {
                from_state: self.prev_state_label.clone(),
                to_state: current.clone(),
                frame: self.frame,
            });
            self.prev_state_label = current;
        }
    }

    fn on_platform(&self) -> bool {
        // Check if player is standing on any platform
        let foot_y = self.py + PLAYER_H;
        for p in &self.platforms {
            if self.px + PLAYER_W > p.x && self.px < p.x + p.w
                && foot_y >= p.y && foot_y <= p.y + 6.0
                && self.vy >= 0.0
            {
                return true;
            }
        }
        false
    }

    fn snap_to_platform(&mut self) {
        let foot_y = self.py + PLAYER_H;
        for p in &self.platforms {
            if self.px + PLAYER_W > p.x && self.px < p.x + p.w
                && foot_y >= p.y && foot_y <= p.y + 10.0
                && self.vy >= 0.0
            {
                self.py = p.y - PLAYER_H;
                self.vy = 0.0;
                return;
            }
        }
    }

    fn tick(&mut self) {
        if self.completed { return; }
        self.frame += 1;

        let gravity    = get_param("gravity").unwrap_or(DEFAULT_GRAVITY);
        let jump_force = get_param("jump_force").unwrap_or(DEFAULT_JUMP_FORCE);
        let move_speed = get_param("move_speed").unwrap_or(DEFAULT_MOVE_SPEED);
        let coyote_max = get_param("coyote_time").unwrap_or(DEFAULT_COYOTE_FRAMES) as u64;

        // Horizontal movement
        self.vx = 0.0;
        if self.keys.left  { self.vx = -move_speed; }
        if self.keys.right { self.vx = move_speed; }
        self.px += self.vx;

        // Clamp to canvas
        if self.px < 0.0 { self.px = 0.0; }
        if self.px + PLAYER_W > CANVAS_W { self.px = CANVAS_W - PLAYER_W; }

        // Horizontal platform collision
        for p in &self.platforms {
            if self.py + PLAYER_H > p.y + 2.0 && self.py < p.y + p.h {
                // Hitting from the left
                if self.vx > 0.0 && self.px + PLAYER_W > p.x && self.px < p.x {
                    self.px = p.x - PLAYER_W;
                }
                // Hitting from the right
                if self.vx < 0.0 && self.px < p.x + p.w && self.px + PLAYER_W > p.x + p.w {
                    self.px = p.x + p.w;
                }
            }
        }

        // Apply gravity
        self.vy += gravity;
        self.py += self.vy;

        // Platform landing
        let on_ground = self.on_platform();
        if on_ground && self.vy >= 0.0 {
            self.snap_to_platform();
        }

        // Head bonk — hitting platform from below
        for p in &self.platforms {
            if self.vy < 0.0
                && self.px + PLAYER_W > p.x && self.px < p.x + p.w
                && self.py <= p.y + p.h && self.py + PLAYER_H > p.y + p.h
            {
                self.py = p.y + p.h;
                self.vy = 0.0;
            }
        }

        // Fell off screen — reset
        if self.py > CANVAS_H + 50.0 {
            self.px = 60.0;
            self.py = 410.0;
            self.vx = 0.0;
            self.vy = 0.0;
            self.state = PlayerState::Grounded;
        }

        // State machine transitions
        match &self.state {
            PlayerState::Grounded => {
                if !on_ground {
                    // Just walked off edge → coyote time
                    self.state = PlayerState::CoyoteTime;
                    self.coyote_counter = coyote_max;
                } else if self.keys.jump && !self.jump_held {
                    // Jump!
                    self.vy = jump_force;
                    self.state = PlayerState::Jumping;
                    self.jump_held = true;
                }
            }
            PlayerState::CoyoteTime => {
                if self.coyote_counter > 0 {
                    self.coyote_counter -= 1;
                    // Can still jump during coyote time!
                    if self.keys.jump && !self.jump_held {
                        self.vy = jump_force;
                        self.state = PlayerState::Jumping;
                        self.jump_held = true;
                    }
                } else {
                    self.state = PlayerState::Falling;
                }

                if on_ground {
                    self.state = PlayerState::Grounded;
                }
            }
            PlayerState::Jumping => {
                // Variable height: releasing jump early cuts the upward velocity
                if !self.keys.jump && self.vy < jump_force * 0.4 {
                    self.vy *= 0.5;
                }
                if self.vy >= 0.0 {
                    self.state = PlayerState::Falling;
                }
                if on_ground {
                    self.state = PlayerState::Grounded;
                }
            }
            PlayerState::Falling => {
                if on_ground {
                    self.state = PlayerState::Grounded;
                }
            }
        }

        // Track jump key release
        if !self.keys.jump { self.jump_held = false; }

        // Coin collection
        for coin in &mut self.coins {
            if coin.collected { continue; }
            let cx = coin.x + COIN_SIZE / 2.0;
            let cy = coin.y + COIN_SIZE / 2.0;
            let px_center = self.px + PLAYER_W / 2.0;
            let py_center = self.py + PLAYER_H / 2.0;
            let dist = ((cx - px_center).powi(2) + (cy - py_center).powi(2)).sqrt();
            if dist < 18.0 {
                coin.collected = true;
                self.score += 1;
                emit("moment_emit", &MomentEmitData {
                    scene_id: "coin",
                    frame: self.frame,
                    state_blob: None,
                    player_label: Some(format!("coin {}/{}", self.score, self.total_coins)),
                    auto_tags: vec![
                        "coin".into(),
                        format!("score-{}", self.score),
                        self.state.label().into(),
                    ],
                });

                // All coins collected?
                if self.score == self.total_coins {
                    self.completed = true;
                    emit("moment_emit", &MomentEmitData {
                        scene_id: "complete",
                        frame: self.frame,
                        state_blob: None,
                        player_label: Some("all coins collected!".into()),
                        auto_tags: vec!["complete".into(), format!("frame-{}", self.frame)],
                    });
                }
            }
        }

        self.maybe_emit_state_change();

        // FrameTick
        emit("frame_tick", &FrameTickData {
            frame: self.frame,
            position: Some([self.px, self.py]),
            active_states: vec![self.state.label().to_string()],
        });
    }

    fn draw(&self) {
        let ctx = &self.ctx;

        // Sky gradient background
        ctx.set_fill_style_str("#0a0a14");
        ctx.fill_rect(0.0, 0.0, CANVAS_W, CANVAS_H);

        // Subtle grid lines for depth
        ctx.set_stroke_style_str("rgba(255,255,255,0.03)");
        let mut y = 0.0;
        while y < CANVAS_H {
            ctx.begin_path();
            ctx.move_to(0.0, y);
            ctx.line_to(CANVAS_W, y);
            ctx.stroke();
            y += 40.0;
        }

        // Platforms
        for p in &self.platforms {
            ctx.set_fill_style_str(p.color);
            ctx.fill_rect(p.x, p.y, p.w, p.h);
            // Top edge highlight
            ctx.set_fill_style_str("rgba(255,255,255,0.1)");
            ctx.fill_rect(p.x, p.y, p.w, 2.0);
        }

        // Coins
        for coin in &self.coins {
            if coin.collected { continue; }
            ctx.set_fill_style_str("#f59e0b");
            ctx.begin_path();
            let _ = ctx.arc(
                coin.x + COIN_SIZE / 2.0, coin.y + COIN_SIZE / 2.0,
                COIN_SIZE / 2.0, 0.0, std::f64::consts::TAU,
            );
            ctx.fill();
            // Inner shine
            ctx.set_fill_style_str("#fbbf24");
            ctx.begin_path();
            let _ = ctx.arc(
                coin.x + COIN_SIZE / 2.0 - 1.0, coin.y + COIN_SIZE / 2.0 - 1.0,
                COIN_SIZE / 4.0, 0.0, std::f64::consts::TAU,
            );
            ctx.fill();
        }

        // Player
        let color = match self.state {
            PlayerState::Grounded   => "#14b8a6",  // teal
            PlayerState::Jumping    => "#22c55e",  // green
            PlayerState::Falling    => "#ef4444",  // red
            PlayerState::CoyoteTime => "#a855f7",  // purple (visible coyote indicator!)
        };
        ctx.set_fill_style_str(color);
        ctx.fill_rect(self.px, self.py, PLAYER_W, PLAYER_H);
        // Eyes
        ctx.set_fill_style_str("#0a0a0f");
        let eye_y = self.py + 8.0;
        let eye_x = if self.vx >= 0.0 { self.px + 12.0 } else { self.px + 4.0 };
        ctx.fill_rect(eye_x, eye_y, 3.0, 4.0);
        ctx.fill_rect(eye_x + 5.0, eye_y, 3.0, 4.0);

        // Velocity arrow (shows jump arc direction)
        if self.state != PlayerState::Grounded {
            let arrow_len = (self.vy * 2.0).min(30.0).max(-30.0);
            ctx.set_stroke_style_str("rgba(255,255,255,0.3)");
            ctx.set_line_width(1.5);
            ctx.begin_path();
            let center_x = self.px + PLAYER_W / 2.0;
            ctx.move_to(center_x, self.py - 4.0);
            ctx.line_to(center_x, self.py - 4.0 + arrow_len);
            ctx.stroke();
            ctx.set_line_width(1.0);
        }

        // HUD
        ctx.set_fill_style_str("rgba(255,255,255,0.7)");
        ctx.set_font("14px 'JetBrains Mono', monospace");
        ctx.set_text_align("left");
        let _ = ctx.fill_text(
            &format!("Coins: {}/{}", self.score, self.total_coins),
            8.0, 20.0,
        );

        // State indicator
        ctx.set_fill_style_str(color);
        ctx.set_font("11px 'JetBrains Mono', monospace");
        ctx.set_text_align("right");
        let _ = ctx.fill_text(self.state.label(), CANVAS_W - 8.0, 20.0);

        // Frame + coyote counter
        ctx.set_fill_style_str("rgba(255,255,255,0.25)");
        ctx.set_text_align("left");
        let coyote_info = if self.coyote_counter > 0 {
            format!(" coyote:{}", self.coyote_counter)
        } else {
            String::new()
        };
        let _ = ctx.fill_text(
            &format!("f{}{}", self.frame, coyote_info),
            8.0, CANVAS_H - 8.0,
        );

        // Controls hint
        ctx.set_text_align("right");
        let _ = ctx.fill_text("←/→ move · Space jump", CANVAS_W - 8.0, CANVAS_H - 8.0);

        // Completion overlay
        if self.completed {
            ctx.set_fill_style_str("rgba(10,10,15,0.75)");
            ctx.fill_rect(0.0, 0.0, CANVAS_W, CANVAS_H);
            ctx.set_fill_style_str("#22c55e");
            ctx.set_font("bold 28px 'Inter', sans-serif");
            ctx.set_text_align("center");
            let _ = ctx.fill_text("ALL COINS COLLECTED!", CANVAS_W / 2.0, CANVAS_H / 2.0 - 10.0);
            ctx.set_fill_style_str("#f59e0b");
            ctx.set_font("16px 'Inter', sans-serif");
            let _ = ctx.fill_text(
                &format!("{} coins in {} frames", self.total_coins, self.frame),
                CANVAS_W / 2.0, CANVAS_H / 2.0 + 20.0,
            );
            ctx.set_fill_style_str("rgba(255,255,255,0.4)");
            ctx.set_font("14px 'Inter', sans-serif");
            let _ = ctx.fill_text("Refresh to play again", CANVAS_W / 2.0, CANVAS_H / 2.0 + 48.0);
        }
    }
}

// =============================================================================
// Entry point
// =============================================================================

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no window");
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
        "demo_id":   "jump-feel",
        "branch_id": "main",
        "seed":      42,
        "param_manifest": {
            "params": [
                {
                    "key": "gravity", "label": "Gravity",
                    "kind": "float", "default": 0.55, "min": 0.1, "max": 1.5, "step": 0.05,
                    "group": "Physics"
                },
                {
                    "key": "jump_force", "label": "Jump force",
                    "kind": "float", "default": -10.0, "min": -15.0, "max": -4.0, "step": 0.5,
                    "group": "Physics"
                },
                {
                    "key": "move_speed", "label": "Move speed",
                    "kind": "float", "default": 4.0, "min": 1.0, "max": 8.0, "step": 0.5,
                    "group": "Movement"
                },
                {
                    "key": "coyote_time", "label": "Coyote time (frames)",
                    "kind": "float", "default": 6.0, "min": 0.0, "max": 20.0, "step": 1.0,
                    "group": "Timing"
                }
            ]
        }
    }));

    // --- Keyboard input ------------------------------------------------
    {
        let game_ref = game.clone();
        let keydown = Closure::<dyn FnMut(KeyboardEvent)>::new(move |e: KeyboardEvent| {
            let mut g = game_ref.borrow_mut();
            match e.key().as_str() {
                "ArrowLeft"  | "a" | "A" => g.keys.left  = true,
                "ArrowRight" | "d" | "D" => g.keys.right = true,
                " "                      => { g.keys.jump  = true; e.prevent_default(); }
                "ArrowUp" | "w" | "W"    => { g.keys.jump  = true; e.prevent_default(); }
                _ => {}
            }
        });
        window.add_event_listener_with_callback("keydown", keydown.as_ref().unchecked_ref())?;
        keydown.forget();
    }
    {
        let game_ref = game.clone();
        let keyup = Closure::<dyn FnMut(KeyboardEvent)>::new(move |e: KeyboardEvent| {
            let mut g = game_ref.borrow_mut();
            match e.key().as_str() {
                "ArrowLeft"  | "a" | "A" => g.keys.left  = false,
                "ArrowRight" | "d" | "D" => g.keys.right = false,
                " "                      => g.keys.jump  = false,
                "ArrowUp" | "w" | "W"    => g.keys.jump  = false,
                _ => {}
            }
        });
        window.add_event_listener_with_callback("keyup", keyup.as_ref().unchecked_ref())?;
        keyup.forget();
    }

    // --- RAF loop -------------------------------------------------------
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        {
            let mut game_borrow = game.borrow_mut();
            game_borrow.tick();
            game_borrow.draw();

            if game_borrow.completed {
                let duration = web_sys::window()
                    .and_then(|w| w.performance())
                    .map(|p| p.now() - game_borrow.start_ms)
                    .unwrap_or(0.0) as u64;
                emit("session_end", &SessionEndData {
                    frame_count: game_borrow.frame,
                    duration_ms: duration,
                    input_log_available: false,
                });
                return;
            }
        }

        web_sys::window()
            .expect("no window")
            .request_animation_frame(
                f.borrow().as_ref().unwrap().as_ref().unchecked_ref()
            )
            .expect("rAF failed");
    }));

    window.request_animation_frame(
        g.borrow().as_ref().unwrap().as_ref().unchecked_ref()
    )?;

    Ok(())
}
