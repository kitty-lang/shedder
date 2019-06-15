func before(before: str): str {
	puts(before);
	return "after";
}

func main() {
	puts(before("before"));
}
