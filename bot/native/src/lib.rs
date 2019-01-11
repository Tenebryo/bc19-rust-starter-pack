extern crate cfg_if;
extern crate wasm_bindgen;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen(module = "battlecode")]
extern "C" {
    pub type BCAbstractRobot;

    #[wasm_bindgen(constructor)]
    fn new() -> BCAbstractRobot;

    #[wasm_bindgen(method)]
    fn signal(this : &BCAbstractRobot, value : u8, radius : isize) -> JsValue;

    #[wasm_bindgen(method)]
    fn castleTalk(this : &BCAbstractRobot, value : u16);

    #[wasm_bindgen(method)]
    fn proposeTrade(this : &BCAbstractRobot,karbonite : isize, fuel : isize) -> JsValue;

    #[wasm_bindgen(method)]
    fn buildUnit(this : &BCAbstractRobot,unit : usize, dx : isize, dy : isize) -> JsValue;

    /// can't call function 'move' in Rust
    #[wasm_bindgen(method, js_name=move)]
    fn move_unit(this : &BCAbstractRobot,dx : isize, dy : isize) -> JsValue;

    #[wasm_bindgen(method)]
    fn mine(this : &BCAbstractRobot) -> JsValue;

    #[wasm_bindgen(method)]
    fn give(this : &BCAbstractRobot,dx : isize, dy : isize, karbonite : isize, fuel : isize) -> JsValue;

    #[wasm_bindgen(method)]
    fn attack(this : &BCAbstractRobot,dx : isize, dy : isize) -> JsValue;

    #[wasm_bindgen(method)]
    fn getRobot(this : &BCAbstractRobot, id : usize) -> JsValue;

    #[wasm_bindgen(method)]
    fn isVisible(this : &BCAbstractRobot, robot : JsValue) -> bool;

    #[wasm_bindgen(method)]
    fn isRadioing(this : &BCAbstractRobot, robot : JsValue) -> bool;

    #[wasm_bindgen(method)]
    fn getVisibleRobotMap(this : &BCAbstractRobot) -> JsValue;

    #[wasm_bindgen(method)]
    fn getPassableMap(this : &BCAbstractRobot) -> JsValue;

    #[wasm_bindgen(method)]
    fn getKarboniteMap(this : &BCAbstractRobot) -> JsValue;

    #[wasm_bindgen(method)]
    fn getFuelMap(this : &BCAbstractRobot) -> JsValue;

    #[wasm_bindgen(method)]
    fn getVisibleRobots(this : &BCAbstractRobot) -> JsValue;

}

#[wasm_bindgen]
pub fn test_turn(this : &BCAbstractRobot) -> JsValue {
    this.buildUnit(2, 1,1)
}

// an example function that is used in the example bot
#[wasm_bindgen]
pub fn test_fn(in0 : f64, in1 : f64) -> f64 {
    in0 * in1 - in1 * in0
}