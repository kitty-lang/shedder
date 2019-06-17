func before() {
	printf("before");
}

func main() {
	let between = "between";

	before();
	printf(between);
	after();
}


func after() {
	printf("after");
}
