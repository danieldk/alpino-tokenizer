fn main() {
    cc::Build::new().file("c/libtok.c").compile("tok");
}
