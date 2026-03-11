// cargo run --example binding_to_events

use tcl::*;
use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();

    let l = tk.root().add_ttk_label( "l" -text("Starting...") -width(25) -padding((10,5, 5, 10)))?.grid(())?;

    l.bind( event::enter(), tclosure!( tk, || l.configure( -text("Moved mouse inside") )))?;

    l.bind( event::leave(), tclosure!( tk, || l.configure( -text("Moved mouse outside") )))?;

    l.bind( event::button_press_1(), tclosure!( tk, || l.configure( -text("Clicked left mouse button") )))?;

    l.bind( event::button_press_3(), tclosure!( tk, || l.configure( -text("Clicked right mouse button") )))?;

    l.bind( event::double().button_press_1(), tclosure!( tk, || l.configure( -text("Double clicked") )))?;

    l.bind( event::button_3().motion(), tclosure!( tk, |evt_rootx, evt_rooty| -> TkResult<()> {
        Ok( l.configure( -text( format!( "right button drag to {evt_rootx} {evt_rooty}" )))? )
    }))?;

    root.bind_more( event::key_press(TkKey::Escape), tclosure!(tk, || l.configure( -text("Escape")) ))?;
    //tk.quit();
    Ok( main_loop() )
}
