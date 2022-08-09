use std::fs::File;
use std::io::Read;
use rlua::{Table, Function, StdLib, InitFlags};

fn main() {
    let mut luafile = File::open("script.lua").expect("aa");
    
    let mut lfile = String::new();
	luafile.read_to_string(&mut lfile).expect("aa");	

	unsafe {
		let _lua = rlua::Lua::unsafe_new_with_flags(StdLib::ALL, InitFlags::NONE).context(|lua| {
			let r = lua.load(&lfile).exec();
			let glob = lua.globals();

			//println!("GLOB : {:?}", glob);
			//println!("{:?}", lua);

			//getting values:			
			let a1;
			{
				let a = glob.get::<_, Table>("map").unwrap();
				a1 = a.get::<_, String>("command").unwrap();
			}
			println!("{}", a1);

			let b2;
			{
				let b1;
				{
					let b = glob.get::<_, Table>("map").unwrap();
					b1 = b.get::<_, Table>("pos").unwrap();
					b2 = b1.get::<_, i32>("x");
				}
			}
			println!("{:?}", b2.unwrap());

			let c = glob.get::<_, i32>("a");
			println!("{}", c.unwrap());
			glob.set::<_, i32>("a", 1).unwrap();
			let c0 = glob.get::<_, i32>("a");
			println!("{}", c0.unwrap());

			let d = glob.get::<_, Vec<usize>>("array");
			println!("{:?}", d.unwrap());
			let e = glob.get::<_, Vec<String>>("names");
			println!("{:?}", e.unwrap());

			let f = glob.get::<_, Function>("add").unwrap();
			let ff = f.call::<_, usize>(0);
			println!("{:?}", ff.unwrap());

			let g = glob.get::<_, Function>("addN").unwrap();
			let gg = g.call::<_, i32>((6, 6));
			println!("{:?}", gg.unwrap());

			let h = glob.get::<_, Function>("double").unwrap();
			let hh = h.call::<_, i32>(7);
			println!("{:?}", hh);
		}
		);
	}
}
