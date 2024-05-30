// Написать функцию, принимающую слайс generic объектов и выводящую на экран все элементы этого слайса.
// Продемонстрировать, что данная функция работает с любыми типами, реализующими необходимый трейт.
// Для вывода объектов на экран можно использовать трейт Printable из примеров выше.
// Данный трейт потребуется реализовать на типах, которые будут использоваться при демонстрации.

trait Printable {
    fn print(&self);
}

fn print_all<T: Printable>(items: &[T]) {
    for item in items {
        item.print();
    }
}

impl Printable for i32 {
    fn print(&self) {
        println!("{} ", self);
    }
}

impl Printable for char {
    fn print(&self) {
        println!("{} ", self);
    }
}

impl Printable for &str {
    fn print(&self) {
        println!("{} ", self);
    }
}

fn main() {
    print_all(&[30, 50]);
    print_all(&['🙂', '5']);
    print_all(&["Строка", "Другая строка"]);
}
