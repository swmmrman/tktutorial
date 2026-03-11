use tk::*;
use tk::cmd::*;

fn main() -> TkResult<()>   {
    let tk = make_tk!()?;
    let root = tk.root();
    let danger = tk.new_ttk_style( "Danger.TFrame", None );
    danger.configure( -background("red") -borderwidth(5) -relief("raised") )?;

    root
        .add_ttk_frame( "frame" -width(200) -height(200) -style(&danger) )?
        .grid(())?;

    let f = root.add_ttk_frame( "frame2" -width(100) -height(200))?;


    // 5 pixels on all sides
    // frame.configure( -padding( 5 ))?;

    // 5 on left and right, 10 on top and bottom
    // frame.configure( -padding(( 5, 10 )))?;

    // left: 5, top: 7, right: 10, bottom: 12
    f.configure( -padding(( 5, 7, 10, 12 )))?;
    f.configure( -borderwidth(2) -relief("sunken") )?;


    Ok(main_loop())
}
