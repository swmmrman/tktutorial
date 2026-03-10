use tk::cmd::*;
use tk::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();
    root.add_label(-text("Hello, world!"))?.pack(())?;
    Ok(main_loop())
}
