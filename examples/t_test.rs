fn t_ref<'a, T: 'a>(t: &'a T) {}

fn t_bound<'a, T: 'a>(t: T) {}

struct Ref<'a, T: 'a>(&'a T);

fn main() {
    let string = String::from("xfy");

    t_bound(&string);
    t_bound(Ref(&string));
    t_bound(&Ref(&string));

    t_ref(&string);
    // t_ref(Ref(&string));
    t_ref(&Ref(&string));

    t_bound(string);
}
