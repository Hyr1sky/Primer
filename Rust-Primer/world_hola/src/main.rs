/*
fn greetings_traveller() {
    let english: &str = "Hello, traveller!";
    let chinese: &str = "你好，旅行者！";
    let espanol: &str = "Hola, viajero!";
    let germany = "Hallo, Reisender!";
    let regions = [english, chinese, espanol, germany];
    for region in regions {
        println!("{}", region);
    }
}
*/
fn main() {
    // greetings_traveller();
    let (a, mut b): (bool, bool) = (true, true);
    println!("a: {:?}, b: {:?}", a, b);

    b = true;
    assert_eq!(a, b);
}
