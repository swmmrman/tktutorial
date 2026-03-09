use tcl::*;
use tk::cmd::*;
use tk::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?; //No error handing....
    let root = tk.root();
    root.set_wm_title("C to F and back")?; // Again?

    let c = root
        .add_ttk_frame("c" - padding((3, 3, 12, 12)))? // Zed reformatted this
        .grid(-column(0) - row(0) - sticky("nwes"))?;

    root.grid_columnconfigure(0, -weight(1))?; //Might be fun to add some
    root.grid_rowconfigure(0, -weight(1))?;

    c.add_ttk_label("Temp F" - width(7), -textvariable("Temp F"))?
        .grid(-column(2) - row(1) - sticky("we"))?;
}
