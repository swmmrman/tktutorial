use tcl::*;
use tk::cmd::*;
use tk::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?; //No error handing....
    let root = tk.root();
    root.set_wm_title("Temp convertor")?; // Again?

    let c = root
        .add_ttk_frame("c" - padding((3, 3, 12, 12)))? // Zed reformatted this
        .grid(-column(0) - row(0) - sticky("nwes"))?;

    root.grid_columnconfigure(0, -weight(1))?; //Might be fun to add some
    root.grid_rowconfigure(0, -weight(1))?;

    let tempc = c.add_ttk_entry("tempc" - width(7) -textvariable("tempc"))?
        .grid(-column(2) - row(1) - sticky("we"))?;

    c.add_ttk_label("tempf" -textvariable("tempf"))?
        .grid( -column(2) -row(2) -sticky("we"))?;

    c.add_ttk_button( "calc" -text("Convert to °F") -command("calculate"))?
        .grid( -column(3) -row(3) -sticky("w"))?;

    c.add_ttk_label("flbl" -text("°C"))?
        .grid( -column(3) -row(1) -sticky("w"))?;

    c.add_ttk_label("eqlbl" -text("converts to"))?
        .grid( -column(1) -row(2) -sticky("e"))?;

    c.add_ttk_label("clbl" -text("°F"))?
        .grid( -column(3) -row(2) -sticky("w"))?;

    c.winfo_children()?
        .iter()
        .try_for_each( |child| child.grid_configure( -padx(5) -pady(5)))?;

    tempc.focus()?;

    #[proc]
    fn calculate() -> TkResult<()> {
        let interp = tcl_interp!();
        let tempc = interp.get_double("tempc");
        match tempc {
            Ok(tempc) => {
                let tempf = f64::floor( ((tempc * 1.8) + 32.0) * 10.0 ) / 10.0;
                interp.set_double("tempf", tempf)
            },
            Err(_) => interp.set("tempf", ""),
        };
        Ok(())
    }

    //Tutorial says safe due to #[proc]
    unsafe { tk.def_proc("calculate", calculate);}
    root.bind_more( event::key_press(TkKey::Return), "calculate")?;

    Ok(main_loop())
}
