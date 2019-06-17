func before(before: str): str {
	printf(before);
	return "after";
}

func main() {
	printf(before("before"));
}
