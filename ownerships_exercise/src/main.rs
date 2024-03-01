fn stringleri_birlestir(str1: &str, str2: &str) -> String {

    let mut var = String::new();

    var.push_str(str1);
    var.push_str(str2);
    return var;
}

fn main() {
    let str1 = String::from("İstan");
    let str2 = String::from("bul");

    let birlestirilmis_stringler = stringleri_birlestir(&str1, &str2);

    println!("{}", birlestirilmis_stringler);
}