use std::fs::File;
use std::io::{Write, BufReader, BufRead, stdin};
use std::io;

fn main() {
	cli();	
}

fn cli() {

	let filename = out_in("file to get initial values from", "final.nat");
	let mut params = init_from_dotnat(&filename).unwrap();

	let tp = out_in("width", "200");
	let width: usize = tp.parse().unwrap();

	let tmp = out_in("height", "200");
	let height: usize = tmp.parse().unwrap();
	
	let temp = out_in("initial zoom", "50");
	let zoom: f64 = temp.parse().unwrap();

	params.imagesize_and_zoom(width, height, zoom);

	
	let quality = out_in("quality", "2000");
	params.quality(quality.parse::<usize>().unwrap());

	let bound = out_in("bound", "5000");
	params.bound(bound.parse::<f64>().unwrap());

	let increase = out_in("how much to increase by", "1.1");
	params.increase(increase.parse::<f64>().unwrap());

	println!("{:?}", params);

	let filename_o = out_in("filename to write to", "params.nat");
	params.what_you_are_made_for(filename_o);
}

fn out_in(to_print: &str, default_val: &str) -> String{
	let mut input = String::new();
	print!("{} ({}): ", to_print, default_val);
	io::stdout().flush().unwrap();
	stdin().read_line(&mut input).expect("error: unable to read user input");
	input.pop(); // remove whitespace

	if input.len() == 0 {
		input = default_val.to_owned();
	}

	input
}


#[derive(Debug)]
struct ParamFile {
	low_x: String,
	low_y: String,
	final_zoom: String,
	precision: String,
	radius_x: Option<f64>,
	radius_y: Option<f64>,
	quality: Option<usize>,
	bound: Option<f64>,
	first_zoom: Option<f64>,
	increase: Option<f64>,
}

impl ParamFile {
	fn imagesize_and_zoom(&mut self, x: usize, y: usize, zoom: f64) {
		self.radius_x = Some((x as f64)/(2.0 * zoom));
		self.radius_y = Some((y as f64)/(2.0 * zoom));
		self.first_zoom = Some(zoom);
	}

	fn quality(&mut self, input: usize) {
		self.quality = Some(input);
	}

	fn bound(&mut self, input: f64) {
		self.bound = Some(input);
	}

	fn increase(&mut self, input: f64) {
		self.increase = Some(input);
	}
	

	fn what_you_are_made_for(self, name: String) {
		let mut file = File::create(name).unwrap();
		file.write(format!("{}\n", self.low_x).as_bytes()).unwrap();
		file.write(format!("{}\n", self.low_y).as_bytes()).unwrap();
		file.write(format!("{}\n", self.radius_x.unwrap()).as_bytes()).unwrap();
		file.write(format!("{}\n", self.radius_y.unwrap()).as_bytes()).unwrap();
		file.write(format!("{}\n", self.precision).as_bytes()).unwrap();
		file.write(format!("{}\n", self.quality.unwrap()).as_bytes()).unwrap();
		file.write(format!("{}\n", self.bound.unwrap()).as_bytes()).unwrap();
		file.write(format!("{}\n", self.first_zoom.unwrap()).as_bytes()).unwrap();
		file.write(format!("{}\n", self.final_zoom).as_bytes()).unwrap();
		file.write(format!("{}\n", self.increase.unwrap()).as_bytes()).expect("aaa");
	}
}


fn init_from_dotnat(name: &str) -> Result<ParamFile, String> {
	// tries to initialise a ParamFile from a file on the hard drive
	let file = File::open(name);

	// checks for an error opening file
	if let Err(_) = file {
		return Err("couldnt open file".to_owned());
	}

	let reader = BufReader::new(file.unwrap());

	let mut read = vec!();

	for line in reader.lines() {
		read.push(line.unwrap());
	}
	
	Ok(ParamFile {
		//popped in reverse order
		precision: read.pop().unwrap(),
		final_zoom: read.pop().unwrap(),
		low_y: read.pop().unwrap(),
		low_x: read.pop().unwrap(),

		radius_x: None,
		radius_y: None,
		quality: None,
		bound: None,
		first_zoom: None,
		increase: None,
	})
}
