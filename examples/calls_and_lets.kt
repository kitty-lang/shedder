func before() {
	puts("before");
}

func main() {
	let between = "between";

	before();
	puts(between);
	after();
}


func after() {
	puts("after");
}
