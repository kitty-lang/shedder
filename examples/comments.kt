// A // B // C
func one() {
	puts("Hello, World!");
}
/* A    B    C */
func two() {
	one();
}
/* /*   A   */ */
func three() {
	two();
}
/* //   A   // */
func main() {
	three();
}
/* A */ /* B */
