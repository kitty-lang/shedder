func int(): i32 {
	return 12;
}

func main() {
	let int = int();
	let string = "Hello, \"World\"";

	let dyn_string = `int == ${int}; string == ${string}`;

	printf(dyn_string);
}
