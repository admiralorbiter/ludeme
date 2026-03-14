//! maze-80 — Pac-Man (1980) inspired maze chase for Ludeme
//!
//! Mechanic families: ai-behavior, state-transitions
//! Fidelity: interpreted
//!
//! Shell API contract:
//!   - Emits SessionStart on init (with ParamManifest)
//!   - Emits FrameTick every frame
//!   - Emits StateChange when game state machine transitions
//!   - Emits MomentEmit on pellet collect and ghost catch
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

const COLS: usize = 20;
const ROWS: usize = 15;
const CELL: f64 = 32.0; // each cell is 32×32

const OFFSET_X: f64 = 0.0;  // (640 - 20*32) / 2 = 0
const OFFSET_Y: f64 = 0.0;  // (480 - 15*32) / 2 = 0

const DEFAULT_MOVE_INTERVAL: f64 = 8.0;  // frames between moves
const DEFAULT_GHOST_SPEED: f64 = 12.0;    // ghost moves every N frames
const NUM_GHOSTS: usize = 3;

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
// Maze layout — 0 = wall, 1 = path with pellet, 2 = empty path, 3 = player start, 4 = ghost start
// =============================================================================

const MAZE: [[u8; COLS]; ROWS] = [
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,3,1,1,1,1,1,1,1,0,0,1,1,1,1,1,1,1,1,0],
    [0,1,0,0,1,0,0,0,1,0,0,1,0,0,0,1,0,0,1,0],
    [0,1,0,0,1,0,0,0,1,0,0,1,0,0,0,1,0,0,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,0,0,1,0,1,0,0,0,0,0,0,1,0,1,0,0,1,0],
    [0,1,1,1,1,0,1,1,1,0,0,1,1,1,0,1,1,1,1,0],
    [0,0,0,0,1,0,0,0,1,0,0,1,0,0,0,1,0,0,0,0],
    [0,1,1,1,1,0,1,1,1,4,4,1,1,1,0,1,1,1,1,0],
    [0,1,0,0,1,0,1,0,0,0,0,0,0,1,0,1,0,0,1,0],
    [0,1,1,1,1,1,1,1,1,0,0,1,1,1,1,1,1,1,1,0],
    [0,1,0,0,1,0,0,0,1,0,0,1,0,0,0,1,0,0,1,0],
    [0,1,0,0,1,0,0,0,1,0,0,1,0,0,0,1,0,0,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
];

// =============================================================================
// Game state
// =============================================================================

#[derive(Debug, Clone, PartialEq)]
enum MazeState {
    Playing,
    Caught,       // ghost caught the player — brief pause
    LevelClear,
    GameOver,
}

impl MazeState {
    fn label(&self) -> &'static str {
        match self {
            MazeState::Playing    => "playing",
            MazeState::Caught     => "caught",
            MazeState::LevelClear => "level_clear",
            MazeState::GameOver   => "game_over",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir { Up, Down, Left, Right }

impl Dir {
    fn delta(&self) -> (i32, i32) {
        match self {
            Dir::Up    => (0, -1),
            Dir::Down  => (0, 1),
            Dir::Left  => (-1, 0),
            Dir::Right => (1, 0),
        }
    }
}

struct Ghost {
    col: usize,
    row: usize,
    color: &'static str,
}

struct Game {
    ctx:        CanvasRenderingContext2d,
    grid:       [[u8; COLS]; ROWS],
    player_col: usize,
    player_row: usize,
    player_dir: Dir,
    next_dir:   Option<Dir>,
    ghosts:     Vec<Ghost>,
    score:      u32,
    pellets_left:   u32,
    lives:      u32,
    state:      MazeState,
    prev_state_label: String,
    frame:      u64,
    start_ms:   f64,
    move_timer: u64,
    ghost_timer: u64,
    pause_timer: u64,
}

impl Game {
    fn new(ctx: CanvasRenderingContext2d, start_ms: f64) -> Self {
        let mut grid = MAZE;
        let mut player_col = 1;
        let mut player_row = 1;
        let mut ghosts = Vec::new();
        let mut pellets = 0u32;
        let ghost_colors = ["#ef4444", "#a855f7", "#14b8a6"];
        let mut ghost_idx = 0;

        for r in 0..ROWS {
            for c in 0..COLS {
                match grid[r][c] {
                    1 => pellets += 1,
                    3 => {
                        player_col = c;
                        player_row = r;
                        grid[r][c] = 2; // clear start marker
                    }
                    4 => {
                        if ghost_idx < NUM_GHOSTS {
                            ghosts.push(Ghost {
                                col: c, row: r,
                                color: ghost_colors[ghost_idx % ghost_colors.len()],
                            });
                            ghost_idx += 1;
                        }
                        grid[r][c] = 2;
                    }
                    _ => {}
                }
            }
        }

        // Ensure we have the right number of ghosts
        while ghosts.len() < NUM_GHOSTS {
            ghosts.push(Ghost { col: 9, row: 8, color: ghost_colors[ghosts.len() % ghost_colors.len()] });
        }

        Game {
            ctx,
            grid,
            player_col, player_row,
            player_dir: Dir::Right,
            next_dir: None,
            ghosts,
            score: 0,
            pellets_left: pellets,
            lives: 3,
            state: MazeState::Playing,
            prev_state_label: "playing".to_string(),
            frame: 0,
            start_ms,
            move_timer: 0,
            ghost_timer: 0,
            pause_timer: 0,
        }
    }

    fn is_walkable(&self, col: i32, row: i32) -> bool {
        if col < 0 || row < 0 || col >= COLS as i32 || row >= ROWS as i32 {
            return false;
        }
        self.grid[row as usize][col as usize] != 0
    }

    fn try_move_player(&mut self, dir: Dir) -> bool {
        let (dx, dy) = dir.delta();
        let nc = self.player_col as i32 + dx;
        let nr = self.player_row as i32 + dy;
        if self.is_walkable(nc, nr) {
            self.player_col = nc as usize;
            self.player_row = nr as usize;
            self.player_dir = dir;
            true
        } else {
            false
        }
    }

    fn move_ghost_toward_player(&self, ghost: &Ghost) -> (usize, usize) {
        // Simple chase AI: try to move toward player, else random valid direction
        let dirs = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];
        let mut best_dir = None;
        let mut best_dist = f64::MAX;

        for dir in &dirs {
            let (dx, dy) = dir.delta();
            let nc = ghost.col as i32 + dx;
            let nr = ghost.row as i32 + dy;
            if self.is_walkable(nc, nr) {
                let dist = ((nc - self.player_col as i32) as f64).powi(2)
                         + ((nr - self.player_row as i32) as f64).powi(2);
                if dist < best_dist {
                    best_dist = dist;
                    best_dir = Some((nc as usize, nr as usize));
                }
            }
        }

        best_dir.unwrap_or((ghost.col, ghost.row))
    }

    fn check_ghost_collision(&self) -> bool {
        self.ghosts.iter().any(|g| g.col == self.player_col && g.row == self.player_row)
    }

    fn maybe_emit_state_change(&mut self) {
        let current = self.state.label().to_string();
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

        let move_interval = get_param("move_speed").unwrap_or(DEFAULT_MOVE_INTERVAL) as u64;
        let ghost_interval = get_param("ghost_speed").unwrap_or(DEFAULT_GHOST_SPEED) as u64;

        match self.state.clone() {
            MazeState::Playing => {
                // Move player
                self.move_timer += 1;
                if self.move_timer >= move_interval {
                    self.move_timer = 0;

                    // Try the buffered direction first
                    if let Some(nd) = self.next_dir {
                        if self.try_move_player(nd) {
                            self.next_dir = None;
                        } else {
                            // Fall back to current direction
                            self.try_move_player(self.player_dir);
                        }
                    } else {
                        self.try_move_player(self.player_dir);
                    }

                    // Collect pellet
                    if self.grid[self.player_row][self.player_col] == 1 {
                        self.grid[self.player_row][self.player_col] = 2;
                        self.score += 10;
                        self.pellets_left -= 1;

                        emit("moment_emit", &MomentEmitData {
                            scene_id:    "pellet",
                            frame:       self.frame,
                            state_blob:  None,
                            player_label: Some(format!("pellet at ({},{})", self.player_col, self.player_row)),
                            auto_tags:   vec!["pellet".into(), format!("score-{}", self.score)],
                        });

                        if self.pellets_left == 0 {
                            self.state = MazeState::LevelClear;
                        }
                    }
                }

                // Move ghosts
                self.ghost_timer += 1;
                if self.ghost_timer >= ghost_interval {
                    self.ghost_timer = 0;
                    let positions: Vec<(usize, usize)> = self.ghosts.iter()
                        .map(|g| self.move_ghost_toward_player(g))
                        .collect();
                    for (i, (nc, nr)) in positions.into_iter().enumerate() {
                        self.ghosts[i].col = nc;
                        self.ghosts[i].row = nr;
                    }
                }

                // Check collision
                if self.check_ghost_collision() {
                    self.lives -= 1;
                    emit("moment_emit", &MomentEmitData {
                        scene_id:    "caught",
                        frame:       self.frame,
                        state_blob:  None,
                        player_label: Some("ghost caught player".into()),
                        auto_tags:   vec!["caught".into(), format!("lives-{}", self.lives)],
                    });

                    if self.lives == 0 {
                        self.state = MazeState::GameOver;
                    } else {
                        self.state = MazeState::Caught;
                        self.pause_timer = 60; // 1 second pause
                    }
                }
            }

            MazeState::Caught => {
                if self.pause_timer > 0 {
                    self.pause_timer -= 1;
                } else {
                    // Reset positions
                    self.player_col = 1;
                    self.player_row = 1;
                    self.player_dir = Dir::Right;
                    // Reset ghosts to start
                    self.ghosts[0].col = 9; self.ghosts[0].row = 8;
                    if self.ghosts.len() > 1 { self.ghosts[1].col = 10; self.ghosts[1].row = 8; }
                    if self.ghosts.len() > 2 { self.ghosts[2].col = 9; self.ghosts[2].row = 8; }
                    self.state = MazeState::Playing;
                }
            }

            MazeState::LevelClear | MazeState::GameOver => {
                // Terminal states
            }
        }

        self.maybe_emit_state_change();

        // FrameTick
        let active = vec![self.state.label().to_string()];
        emit("frame_tick", &FrameTickData {
            frame:         self.frame,
            position:      Some([self.player_col as f64 * CELL, self.player_row as f64 * CELL]),
            active_states: active,
        });
    }

    fn draw(&self) {
        let ctx = &self.ctx;

        // Clear
        ctx.set_fill_style_str("#0a0a0f");
        ctx.fill_rect(0.0, 0.0, CANVAS_W, CANVAS_H);

        // Draw grid
        for r in 0..ROWS {
            for c in 0..COLS {
                let x = OFFSET_X + c as f64 * CELL;
                let y = OFFSET_Y + r as f64 * CELL;

                match self.grid[r][c] {
                    0 => {
                        // Wall
                        ctx.set_fill_style_str("#1a1a2e");
                        ctx.fill_rect(x, y, CELL, CELL);
                        // Wall border
                        ctx.set_stroke_style_str("#2a2a4e");
                        ctx.stroke_rect(x + 0.5, y + 0.5, CELL - 1.0, CELL - 1.0);
                    }
                    1 => {
                        // Pellet
                        ctx.set_fill_style_str("#f59e0b");
                        ctx.begin_path();
                        let _ = ctx.arc(x + CELL / 2.0, y + CELL / 2.0, 3.0, 0.0, std::f64::consts::TAU);
                        ctx.fill();
                    }
                    _ => {} // empty path
                }
            }
        }

        // Draw player (yellow circle)
        {
            let px = OFFSET_X + self.player_col as f64 * CELL + CELL / 2.0;
            let py = OFFSET_Y + self.player_row as f64 * CELL + CELL / 2.0;
            ctx.set_fill_style_str("#fbbf24");
            ctx.begin_path();
            let _ = ctx.arc(px, py, CELL / 2.0 - 2.0, 0.0, std::f64::consts::TAU);
            ctx.fill();

            // Eyes indicate direction
            ctx.set_fill_style_str("#0a0a0f");
            let (ex, ey) = match self.player_dir {
                Dir::Up    => (0.0, -4.0),
                Dir::Down  => (0.0, 4.0),
                Dir::Left  => (-4.0, 0.0),
                Dir::Right => (4.0, 0.0),
            };
            ctx.begin_path();
            let _ = ctx.arc(px + ex - 3.0, py + ey - 2.0, 2.5, 0.0, std::f64::consts::TAU);
            ctx.fill();
            ctx.begin_path();
            let _ = ctx.arc(px + ex + 3.0, py + ey - 2.0, 2.5, 0.0, std::f64::consts::TAU);
            ctx.fill();
        }

        // Draw ghosts
        for ghost in &self.ghosts {
            let gx = OFFSET_X + ghost.col as f64 * CELL + CELL / 2.0;
            let gy = OFFSET_Y + ghost.row as f64 * CELL + CELL / 2.0;
            ctx.set_fill_style_str(ghost.color);
            ctx.begin_path();
            let _ = ctx.arc(gx, gy, CELL / 2.0 - 2.0, 0.0, std::f64::consts::TAU);
            ctx.fill();

            // Ghost eyes
            ctx.set_fill_style_str("#ffffff");
            ctx.begin_path();
            let _ = ctx.arc(gx - 4.0, gy - 2.0, 3.0, 0.0, std::f64::consts::TAU);
            ctx.fill();
            ctx.begin_path();
            let _ = ctx.arc(gx + 4.0, gy - 2.0, 3.0, 0.0, std::f64::consts::TAU);
            ctx.fill();
            ctx.set_fill_style_str("#111");
            ctx.begin_path();
            let _ = ctx.arc(gx - 3.0, gy - 2.0, 1.5, 0.0, std::f64::consts::TAU);
            ctx.fill();
            ctx.begin_path();
            let _ = ctx.arc(gx + 5.0, gy - 2.0, 1.5, 0.0, std::f64::consts::TAU);
            ctx.fill();
        }

        // HUD: score + lives
        ctx.set_fill_style_str("rgba(255,255,255,0.7)");
        ctx.set_font("16px 'JetBrains Mono', monospace");
        ctx.set_text_align("left");
        let _ = ctx.fill_text(&format!("Score: {}", self.score), 8.0, CANVAS_H - 8.0);
        ctx.set_text_align("right");
        let _ = ctx.fill_text(&format!("Lives: {}", self.lives), CANVAS_W - 8.0, CANVAS_H - 8.0);

        // State label + frame
        ctx.set_fill_style_str("rgba(255,255,255,0.25)");
        ctx.set_font("11px 'JetBrains Mono', monospace");
        ctx.set_text_align("center");
        let _ = ctx.fill_text(
            &format!("{} | f{}", self.state.label(), self.frame),
            CANVAS_W / 2.0, CANVAS_H - 8.0,
        );

        // Game over overlay
        if self.state == MazeState::GameOver {
            ctx.set_fill_style_str("rgba(10,10,15,0.75)");
            ctx.fill_rect(0.0, 0.0, CANVAS_W, CANVAS_H);
            ctx.set_fill_style_str("#ef4444");
            ctx.set_font("bold 32px 'Inter', sans-serif");
            ctx.set_text_align("center");
            let _ = ctx.fill_text("GAME OVER", CANVAS_W / 2.0, CANVAS_H / 2.0 - 10.0);
            ctx.set_fill_style_str("#f59e0b");
            ctx.set_font("20px 'Inter', sans-serif");
            let _ = ctx.fill_text(&format!("Score: {}", self.score), CANVAS_W / 2.0, CANVAS_H / 2.0 + 24.0);
            ctx.set_fill_style_str("rgba(255,255,255,0.4)");
            ctx.set_font("14px 'Inter', sans-serif");
            let _ = ctx.fill_text("Refresh to play again", CANVAS_W / 2.0, CANVAS_H / 2.0 + 52.0);
        }

        // Level clear overlay
        if self.state == MazeState::LevelClear {
            ctx.set_fill_style_str("rgba(10,10,15,0.75)");
            ctx.fill_rect(0.0, 0.0, CANVAS_W, CANVAS_H);
            ctx.set_fill_style_str("#22c55e");
            ctx.set_font("bold 32px 'Inter', sans-serif");
            ctx.set_text_align("center");
            let _ = ctx.fill_text("LEVEL CLEAR!", CANVAS_W / 2.0, CANVAS_H / 2.0 - 10.0);
            ctx.set_fill_style_str("#f59e0b");
            ctx.set_font("20px 'Inter', sans-serif");
            let _ = ctx.fill_text(&format!("Score: {}", self.score), CANVAS_W / 2.0, CANVAS_H / 2.0 + 24.0);
        }

        // Caught flash
        if self.state == MazeState::Caught {
            ctx.set_fill_style_str("rgba(239,68,68,0.2)");
            ctx.fill_rect(0.0, 0.0, CANVAS_W, CANVAS_H);
        }
    }
}

// =============================================================================
// Entry point
// =============================================================================

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
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
        "demo_id":   "maze-80",
        "branch_id": "main",
        "seed":      42,
        "param_manifest": {
            "params": [
                {
                    "key": "move_speed", "label": "Player speed",
                    "kind": "float", "default": 8.0, "min": 2.0, "max": 20.0, "step": 1.0,
                    "group": "Speed"
                },
                {
                    "key": "ghost_speed", "label": "Ghost speed",
                    "kind": "float", "default": 12.0, "min": 4.0, "max": 30.0, "step": 1.0,
                    "group": "Speed"
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
                "ArrowUp"    | "w" | "W" => { g.next_dir = Some(Dir::Up);    e.prevent_default(); }
                "ArrowDown"  | "s" | "S" => { g.next_dir = Some(Dir::Down);  e.prevent_default(); }
                "ArrowLeft"  | "a" | "A" => { g.next_dir = Some(Dir::Left);  e.prevent_default(); }
                "ArrowRight" | "d" | "D" => { g.next_dir = Some(Dir::Right); e.prevent_default(); }
                _ => {}
            }
        });
        window.add_event_listener_with_callback("keydown", keydown.as_ref().unchecked_ref())?;
        keydown.forget();
    }

    // --- RAF loop -------------------------------------------------------
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        {
            let mut game_borrow = game.borrow_mut();
            game_borrow.tick();
            game_borrow.draw();

            if matches!(game_borrow.state, MazeState::GameOver | MazeState::LevelClear) {
                let duration = web_sys::window()
                    .and_then(|w| w.performance())
                    .map(|p| p.now() - game_borrow.start_ms)
                    .unwrap_or(0.0) as u64;
                emit("session_end", &SessionEndData {
                    frame_count:         game_borrow.frame,
                    duration_ms:         duration,
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
